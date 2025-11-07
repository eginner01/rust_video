use crate::models::{Author, VideoParseInfo};
use crate::parser::VideoParser;
use crate::utils::create_http_client;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use regex::Regex;
use serde_json::Value;

pub struct QuanminkgeParser;

#[async_trait]
impl VideoParser for QuanminkgeParser {
    async fn parse_share_url(&self, share_url: &str) -> Result<VideoParseInfo> {
        let parsed_url = url::Url::parse(share_url)?;
        
        let video_id = parsed_url.query_pairs()
            .find(|(key, _)| key == "s")
            .map(|(_, value)| value.to_string())
            .ok_or_else(|| anyhow!("无法从分享链接中解析视频ID"))?;
        
        self.parse_video_id(&video_id).await
    }
    
    async fn parse_video_id(&self, video_id: &str) -> Result<VideoParseInfo> {
        let req_url = format!("https://kg.qq.com/node/play?s={}", video_id);
        
        let client = create_http_client()?;
        let html = client
            .get(&req_url)
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
            .send()
            .await?
            .text()
            .await?;
        
        let re = Regex::new(r"window\.__DATA__ = (.*?);")?;
        let json_str = re.captures(&html)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str().trim())
            .ok_or_else(|| anyhow!("无法从HTML中提取数据"))?;
        
        let json: Value = serde_json::from_str(json_str)?;
        
        let data = json.pointer("/detail")
            .ok_or_else(|| anyhow!("无法获取视频数据"))?;
        
        let mut info = VideoParseInfo::new();
        
        info.title = data.pointer("/content")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        
        info.video_url = data.pointer("/playurl_video")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        info.cover_url = data.pointer("/cover")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        info.author = Author {
            uid: data.pointer("/uid")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            name: data.pointer("/nick")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            avatar: data.pointer("/avatar")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
        };
        
        Ok(info)
    }
}

