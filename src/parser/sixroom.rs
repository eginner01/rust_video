use crate::models::{Author, VideoParseInfo};
use crate::parser::VideoParser;
use crate::utils::create_http_client;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use serde_json::Value;

pub struct SixroomParser;

#[async_trait]
impl VideoParser for SixroomParser {
    async fn parse_share_url(&self, share_url: &str) -> Result<VideoParseInfo> {
        let parsed_url = url::Url::parse(share_url)?;
        
        let video_id = if share_url.contains("watchMini.php?vid=") {
            parsed_url.query_pairs()
                .find(|(key, _)| key == "vid")
                .map(|(_, value)| value.to_string())
                .ok_or_else(|| anyhow!("无法从分享链接中解析视频ID"))?
        } else {
            parsed_url.path().replace("/v/", "")
        };
        
        if video_id.is_empty() {
            return Err(anyhow!("无法从分享链接中解析视频ID"));
        }
        
        self.parse_video_id(&video_id).await
    }
    
    async fn parse_video_id(&self, video_id: &str) -> Result<VideoParseInfo> {
        let req_url = format!(
            "https://v.6.cn/coop/mobile/index.php?padapi=minivideo-watchVideo.php&av=3.0&encpass=&logiuid=&isnew=1&from=0&vid={}",
            video_id
        );
        
        let client = create_http_client()?;
        let response = client
            .get(&req_url)
            .header("User-Agent", crate::utils::DEFAULT_USER_AGENT)
            .header("Referer", format!("https://m.6.cn/v/{}", video_id))
            .send()
            .await?;
        
        let json: Value = response.json().await?;
        
        let data = json.pointer("/content")
            .ok_or_else(|| anyhow!("无法获取视频数据"))?;
        
        let mut info = VideoParseInfo::new();
        
        info.title = data.pointer("/title")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        
        info.video_url = data.pointer("/playurl")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        info.cover_url = data.pointer("/picurl")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        info.author = Author {
            uid: String::new(),
            name: data.pointer("/alias")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            avatar: data.pointer("/picuser")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
        };
        
        Ok(info)
    }
}

