use crate::models::{Author, VideoParseInfo};
use crate::parser::VideoParser;
use crate::utils::create_http_client;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use regex::Regex;
use serde_json::Value;

pub struct HuyaParser;

#[async_trait]
impl VideoParser for HuyaParser {
    async fn parse_share_url(&self, share_url: &str) -> Result<VideoParseInfo> {
        let re = Regex::new(r"/(\d+)\.html")?;
        
        let video_id = re.captures(share_url)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str())
            .ok_or_else(|| anyhow!("无法从分享链接中解析视频ID"))?;
        
        self.parse_video_id(video_id).await
    }
    
    async fn parse_video_id(&self, video_id: &str) -> Result<VideoParseInfo> {
        let req_url = format!("https://liveapi.huya.com/moment/getMomentContent?videoId={}", video_id);
        
        let client = create_http_client()?;
        let response = client
            .get(&req_url)
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
            .header("Referer", "https://v.huya.com/")
            .send()
            .await?;
        
        let json: Value = response.json().await?;
        
        let video_data = json.pointer("/data/moment/videoInfo")
            .ok_or_else(|| anyhow!("无法获取视频数据"))?;
        
        let mut info = VideoParseInfo::new();
        
        info.title = video_data.pointer("/videoTitle")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        
        info.video_url = video_data.pointer("/definitions/0/url")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        info.cover_url = video_data.pointer("/videoCover")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        info.author = Author {
            uid: video_data.pointer("/uid")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            name: video_data.pointer("/actorNick")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            avatar: video_data.pointer("/actorAvatarUrl")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
        };
        
        Ok(info)
    }
}

