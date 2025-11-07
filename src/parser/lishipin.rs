use crate::models::VideoParseInfo;
use crate::parser::VideoParser;
use crate::utils::create_http_client;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use serde_json::Value;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct LishipinParser;

#[async_trait]
impl VideoParser for LishipinParser {
    async fn parse_share_url(&self, share_url: &str) -> Result<VideoParseInfo> {
        let parsed_url = url::Url::parse(share_url)?;
        
        let video_id = parsed_url.path()
            .replace("/detail_", "");
        
        if video_id.is_empty() {
            return Err(anyhow!("无法从分享链接中解析视频ID"));
        }
        
        self.parse_video_id(&video_id).await
    }
    
    async fn parse_video_id(&self, video_id: &str) -> Result<VideoParseInfo> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let req_url = format!(
            "https://www.pearvideo.com/videoStatus.jsp?contId={}&mrd={}",
            video_id, timestamp
        );
        
        let client = create_http_client()?;
        let response = client
            .get(&req_url)
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36")
            .header("Referer", format!("https://www.pearvideo.com/detail_{}", video_id))
            .send()
            .await?;
        
        let json: Value = response.json().await?;
        
        let video_info = json.pointer("/videoInfo")
            .ok_or_else(|| anyhow!("无法获取视频数据"))?;
        
        let video_src_url = video_info.pointer("/videos/srcUrl")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("无法获取视频源URL"))?;
        
        let system_time = json.pointer("/systemTime")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let video_url = video_src_url.replace(system_time, &format!("cont-{}", video_id));
        
        let mut info = VideoParseInfo::new();
        
        info.video_url = Some(video_url);
        
        info.cover_url = video_info.pointer("/video_image")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        Ok(info)
    }
}

