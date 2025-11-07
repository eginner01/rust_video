use crate::models::{Author, ImgInfo, VideoParseInfo};
use crate::parser::VideoParser;
use crate::utils::{create_http_client, extract_json_from_html};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use serde_json::Value;

pub struct KuaishouParser;

#[async_trait]
impl VideoParser for KuaishouParser {
    async fn parse_share_url(&self, share_url: &str) -> Result<VideoParseInfo> {
        let client = create_http_client()?;
        
        let response = client
            .get(share_url)
            .header("User-Agent", crate::utils::DEFAULT_USER_AGENT)
            .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8")
            .send()
            .await?;
        
        let final_url = response.url().to_string();
        
        let final_url = final_url.replace("/fw/long-video/", "/fw/photo/");
        
        // 获取页面内容
        let html = client
            .get(&final_url)
            .header("User-Agent", crate::utils::DEFAULT_USER_AGENT)
            .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8")
            .send()
            .await?
            .text()
            .await?;
        
        let pattern = r"window\.INIT_STATE\s*=\s*(.*?)</script>";
        let json_str = extract_json_from_html(&html, pattern)?;
        
        let json: Value = serde_json::from_str(&json_str)?;
        
        let data = self.find_video_data(&json)?;
        
        let result_code = data.pointer("/result")
            .and_then(|v| v.as_i64())
            .ok_or_else(|| anyhow!("无法获取result字段"))?;
        
        if result_code != 1 {
            return Err(anyhow!("获取作品信息失败: result={}", result_code));
        }
        
        self.extract_video_info(&data)
    }
}

impl KuaishouParser {
    fn find_video_data(&self, json: &Value) -> Result<Value> {
        if let Some(obj) = json.as_object() {
            for (_, value) in obj {
                if let Some(value_obj) = value.as_object() {
                    if value_obj.contains_key("result") && value_obj.contains_key("photo") {
                        return Ok(value.clone());
                    }
                }
            }
        }
        
        Err(anyhow!("未找到视频数据"))
    }
    
    fn extract_video_info(&self, data: &Value) -> Result<VideoParseInfo> {
        let photo = data.pointer("/photo")
            .ok_or_else(|| anyhow!("未找到photo字段"))?;
        
        let mut info = VideoParseInfo::new();
        
        // 提取作者信息
        info.author = Author {
            uid: String::new(),
            name: photo.pointer("/userName")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            avatar: photo.pointer("/headUrl")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
        };
        
        info.title = photo.pointer("/caption")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        
        info.video_url = photo.pointer("/mainMvUrls/0/url")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        info.cover_url = photo.pointer("/coverUrls/0/url")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        if let Some(cdn) = photo.pointer("/ext_params/atlas/cdn/0").and_then(|v| v.as_str()) {
            if let Some(list) = photo.pointer("/ext_params/atlas/list").and_then(|v| v.as_array()) {
                for item in list {
                    if let Some(path) = item.as_str() {
                        let img_url = format!("https://{}/{}", cdn, path);
                        info.images.push(ImgInfo {
                            url: img_url,
                            live_photo_url: None,
                        });
                    }
                }
            }
        }
        
        Ok(info)
    }
}

