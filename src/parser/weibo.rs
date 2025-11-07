use crate::models::{ImgInfo, VideoParseInfo};
use crate::parser::VideoParser;
use crate::utils::create_http_client;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use regex::Regex;
use serde_json::Value;

pub struct WeiboParser;

#[async_trait]
impl VideoParser for WeiboParser {
    async fn parse_share_url(&self, share_url: &str) -> Result<VideoParseInfo> {
        let parsed_url = url::Url::parse(share_url)?;
        
        // Handle video URLs (show?fid= or /tv/show/)
        if share_url.contains("show?fid=") {
            let video_id = parsed_url
                .query_pairs()
                .find(|(key, _)| key == "fid")
                .map(|(_, value)| value.to_string())
                .ok_or_else(|| anyhow!("无法从URL中解析视频ID"))?;
            
            return self.parse_video_id(&video_id).await;
        } else if share_url.contains("/tv/show/") {
            let video_id = parsed_url.path().replace("/tv/show/", "");
            return self.parse_video_id(&video_id).await;
        } else {
            // Handle regular post URLs (potential image albums)
            let path_parts: Vec<&str> = parsed_url.path().trim_matches('/').split('/').collect();
            if path_parts.len() >= 2 {
                let post_id = path_parts[path_parts.len() - 1];
                return self.parse_post_url(post_id, share_url).await;
            }
        }
        
        Err(anyhow!("不支持的微博URL格式"))
    }
    
    async fn parse_video_id(&self, video_id: &str) -> Result<VideoParseInfo> {
        let req_url = format!("https://h5.video.weibo.com/api/component?page=/show/{}", video_id);
        
        let client = create_http_client()?;
        let body = format!(
            r#"{{"Component_Play_Playinfo":{{"oid":"{}"}}}}"#,
            video_id
        );
        
        let response = client
            .post(&req_url)
            .header("User-Agent", crate::utils::DEFAULT_USER_AGENT)
            .header("Referer", format!("https://h5.video.weibo.com/show/{}", video_id))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Cookie", "login_sid_t=6b652c77c1a4bc50cb9d06b24923210d")
            .body(format!("data={}", body))
            .send()
            .await?;
        
        let json: Value = response.json().await?;
        
        let data = json.pointer("/data/Component_Play_Playinfo")
            .ok_or_else(|| anyhow!("无法获取视频数据"))?;
        
        // 获取视频URL（第一个最高码率）
        let mut video_url = None;
        if let Some(urls) = data.pointer("/urls").and_then(|v| v.as_object()) {
            for (_, value) in urls {
                if let Some(url_str) = value.as_str() {
                    video_url = Some(format!("https:{}", url_str));
                    break;
                }
            }
        }
        
        let mut info = VideoParseInfo::new();
        
        info.title = data.pointer("/title")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        
        info.video_url = video_url;
        
        info.cover_url = data.pointer("/cover_image")
            .and_then(|v| v.as_str())
            .map(|s| format!("https:{}", s));
        
        info.author.name = data.pointer("/author")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        
        info.author.avatar = data.pointer("/avatar")
            .and_then(|v| v.as_str())
            .map(|s| format!("https:{}", s))
            .unwrap_or_default();
        
        Ok(info)
    }
}

impl WeiboParser {
    /// 解析帖子URL（图集等）
    async fn parse_post_url(&self, post_id: &str, original_url: &str) -> Result<VideoParseInfo> {
        // 尝试移动端API
        let req_url = format!("https://m.weibo.cn/statuses/show?id={}", post_id);
        let client = create_http_client()?;
        
        let response = client
            .get(&req_url)
            .header("User-Agent", "Mozilla/5.0 (iPhone; CPU iPhone OS 14_0 like Mac OS X) AppleWebKit/605.1.15")
            .header("Referer", "https://m.weibo.cn/")
            .header("Content-Type", "application/json;charset=UTF-8")
            .header("X-Requested-With", "XMLHttpRequest")
            .send()
            .await;
        
        if let Ok(resp) = response {
            if let Ok(json) = resp.json::<Value>().await {
                if let Some(data) = json.pointer("/data") {
                    if let Ok(result) = self.parse_mobile_api_data(data) {
                        return Ok(result);
                    }
                }
            }
        }
        
        // 降级到桌面页面解析
        let response = client
            .get(original_url)
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
            .send()
            .await?;
        
        let html = response.text().await?;
        
        self.parse_html_page(&html)
    }
    
    /// 从移动端API数据中提取信息
    fn parse_mobile_api_data(&self, data: &Value) -> Result<VideoParseInfo> {
        let mut info = VideoParseInfo::new();
        
        info.title = self.clean_text(
            data.pointer("/text")
                .and_then(|v| v.as_str())
                .unwrap_or("")
        );
        
        info.author.name = data.pointer("/user/screen_name")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        
        info.author.avatar = data.pointer("/user/avatar_large")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        
        // 提取图片
        if let Some(pics) = data.pointer("/pics").and_then(|v| v.as_array()) {
            for pic in pics {
                let image_url = pic.pointer("/large/url")
                    .and_then(|v| v.as_str())
                    .or_else(|| pic.pointer("/original/url").and_then(|v| v.as_str()))
                    .or_else(|| pic.pointer("/bmiddle/url").and_then(|v| v.as_str()))
                    .or_else(|| pic.pointer("/url").and_then(|v| v.as_str()));
                
                if let Some(url) = image_url {
                    if !url.is_empty() {
                        info.images.push(ImgInfo {
                            url: url.to_string(),
                            live_photo_url: None,
                        });
                    }
                }
            }
        }
        
        Ok(info)
    }
    
    /// 从HTML页面中提取信息
    fn parse_html_page(&self, html: &str) -> Result<VideoParseInfo> {
        // 提取$render_data
        let pattern = r"\$render_data\s*=\s*(.*?)\[0\]";
        let re = Regex::new(pattern)?;
        
        let json_str = re.captures(html)
            .and_then(|caps| caps.get(1))
            .map(|m| format!("{}[0]", m.as_str()))
            .ok_or_else(|| anyhow!("无法从HTML中提取微博数据"))?;
        
        let json: Value = serde_json::from_str(&json_str)?;
        
        let mut info = VideoParseInfo::new();
        
        info.title = self.clean_text(
            json.pointer("/status/text")
                .and_then(|v| v.as_str())
                .unwrap_or("")
        );
        
        info.author.name = json.pointer("/status/user/screen_name")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        
        info.author.avatar = json.pointer("/status/user/avatar_large")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        
        // 提取图片
        if let Some(pics) = json.pointer("/status/pics").and_then(|v| v.as_array()) {
            for pic in pics {
                let image_url = pic.pointer("/large/url")
                    .and_then(|v| v.as_str())
                    .or_else(|| pic.pointer("/original/url").and_then(|v| v.as_str()))
                    .or_else(|| pic.pointer("/bmiddle/url").and_then(|v| v.as_str()))
                    .or_else(|| pic.pointer("/url").and_then(|v| v.as_str()));
                
                if let Some(url) = image_url {
                    if !url.is_empty() {
                        info.images.push(ImgInfo {
                            url: url.to_string(),
                            live_photo_url: None,
                        });
                    }
                }
            }
        }
        
        Ok(info)
    }
    
    /// 清理HTML标签
    fn clean_text(&self, text: &str) -> String {
        let re = Regex::new(r"<[^>]*>").unwrap();
        re.replace_all(text, "").trim().to_string()
    }
}

