use crate::models::{Author, VideoParseInfo};
use crate::parser::VideoParser;
use crate::utils::{create_no_redirect_client, create_http_client, extract_json_from_html};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use serde_json::Value;

pub struct XiguaParser;

#[async_trait]
impl VideoParser for XiguaParser {
    async fn parse_share_url(&self, share_url: &str) -> Result<VideoParseInfo> {
        // 禁用重定向获取视频ID
        let client = create_no_redirect_client()?;
        
        let response = client
            .get(share_url)
            .header("User-Agent", crate::utils::DEFAULT_USER_AGENT)
            .send()
            .await?;
        
        // 获取重定向的Location
        let location = response
            .url()
            .path()
            .trim_matches('/')
            .replace("video/", "");
        
        if location.is_empty() {
            return Err(anyhow!("无法从分享链接中解析视频ID"));
        }
        
        self.parse_video_id(&location).await
    }
    
    async fn parse_video_id(&self, video_id: &str) -> Result<VideoParseInfo> {
        let req_url = format!(
            "https://m.ixigua.com/douyin/share/video/{}?aweme_type=107&schema_type=1&utm_source=copy&utm_campaign=client_share&utm_medium=android&app=aweme",
            video_id
        );
        
        let client = create_http_client()?;
        let html = client
            .get(&req_url)
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
            .header("Cookie", "MONITOR_WEB_ID=7892c49b-296e-4499-8704-e47c1b150c18; ixigua-a-s=1")
            .send()
            .await?
            .text()
            .await?;
        
        // 提取JSON数据
        let pattern = r"window._ROUTER_DATA\s*=\s*(.*?)</script>";
        let json_str = extract_json_from_html(&html, pattern)?;
        
        let json: Value = serde_json::from_str(&json_str)?;
        
        // 提取视频数据
        let video_data = self.find_video_data(&json, video_id)?;
        
        self.extract_video_info(&video_data)
    }
}

impl XiguaParser {
    /// 从JSON中查找视频数据
    fn find_video_data(&self, json: &Value, video_id: &str) -> Result<Value> {
        // 尝试从loaderData中找到视频数据
        if let Some(loader_data) = json.get("loaderData") {
            if let Some(obj) = loader_data.as_object() {
                // 遍历所有键查找视频数据
                for (key, value) in obj {
                    if key.contains("video") || key.contains(video_id) {
                        if let Some(video_info_res) = value.get("videoInfoRes") {
                            if let Some(item_list) = video_info_res.get("item_list") {
                                if let Some(items) = item_list.as_array() {
                                    if let Some(first_item) = items.first() {
                                        return Ok(first_item.clone());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        Err(anyhow!("无法解析西瓜视频数据"))
    }
    
    /// 从JSON数据中提取视频信息
    fn extract_video_info(&self, data: &Value) -> Result<VideoParseInfo> {
        let mut info = VideoParseInfo::new();
        
        // 提取作者信息
        info.author = Author {
            uid: data.pointer("/author/user_id")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            name: data.pointer("/author/nickname")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            avatar: data.pointer("/author/avatar_thumb/url_list/0")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
        };
        
        // 提取标题
        info.title = data.pointer("/desc")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        
        // 提取视频URL
        info.video_url = data.pointer("/video/play_addr/url_list/0")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        // 提取封面
        info.cover_url = data.pointer("/video/cover/url_list/0")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        Ok(info)
    }
}

