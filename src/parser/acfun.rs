use crate::models::VideoParseInfo;
use crate::parser::VideoParser;
use crate::utils::create_http_client;
use anyhow::Result;
use async_trait::async_trait;
use regex::Regex;
use serde_json::Value;

pub struct AcfunParser;

#[async_trait]
impl VideoParser for AcfunParser {
    async fn parse_share_url(&self, share_url: &str) -> Result<VideoParseInfo> {
        let client = create_http_client()?;
        
        let html = client
            .get(share_url)
            .header("User-Agent", "Mozilla/5.0 (iPhone; CPU iPhone OS 11_0 like Mac OS X) AppleWebKit/604.1.38")
            .send()
            .await?
            .text()
            .await?;
        
        let mut info = VideoParseInfo::new();
        
        // 提取videoInfo
        let video_info_re = Regex::new(r"var videoInfo =\s(.*?);")?;
        if let Some(caps) = video_info_re.captures(&html) {
            if let Some(json_str) = caps.get(1) {
                let json_str = json_str.as_str().trim();
                if let Ok(json) = serde_json::from_str::<Value>(json_str) {
                    info.title = json.pointer("/title")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();
                    
                    info.cover_url = json.pointer("/cover")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());
                }
            }
        }
        
        // 提取playInfo
        let play_info_re = Regex::new(r"var playInfo =\s(.*?);")?;
        if let Some(caps) = play_info_re.captures(&html) {
            if let Some(json_str) = caps.get(1) {
                let json_str = json_str.as_str().trim();
                if let Ok(json) = serde_json::from_str::<Value>(json_str) {
                    info.video_url = json.pointer("/streams/0/playUrls/0")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());
                    // 视频地址是m3u8格式
                }
            }
        }
        
        Ok(info)
    }
    
    async fn parse_video_id(&self, video_id: &str) -> Result<VideoParseInfo> {
        // acid格式: ac36935385
        let req_url = format!("https://www.acfun.cn/v/{}", video_id);
        self.parse_share_url(&req_url).await
    }
}

