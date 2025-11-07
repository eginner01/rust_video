use crate::models::{Author, VideoParseInfo};
use crate::parser::VideoParser;
use crate::utils::create_http_client;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use serde_json::Value;

pub struct QuanminParser;

#[async_trait]
impl VideoParser for QuanminParser {
    async fn parse_share_url(&self, share_url: &str) -> Result<VideoParseInfo> {
        let parsed_url = url::Url::parse(share_url)?;
        
        let video_id = parsed_url.query_pairs()
            .find(|(key, _)| key == "vid")
            .map(|(_, value)| value.to_string())
            .ok_or_else(|| anyhow!("无法从分享链接中解析视频ID"))?;
        
        self.parse_video_id(&video_id).await
    }
    
    async fn parse_video_id(&self, video_id: &str) -> Result<VideoParseInfo> {
        let req_url = format!(
            "https://quanmin.hao222.com/wise/growth/api/sv/immerse?source=share-h5&pd=qm_share_mvideo&_format=json&vid={}",
            video_id
        );
        
        let client = create_http_client()?;
        let response = client
            .get(&req_url)
            .header("User-Agent", crate::utils::DEFAULT_USER_AGENT)
            .send()
            .await?;
        
        let json: Value = response.json().await?;
        
        let errno = json.pointer("/errno")
            .and_then(|v| v.as_i64())
            .unwrap_or(-1);
        
        if errno != 0 {
            let error = json.pointer("/error")
                .and_then(|v| v.as_str())
                .unwrap_or("API返回错误");
            return Err(anyhow!("全民视频API错误: {}", error));
        }
        
        if let Some(status_text) = json.pointer("/data/meta/statusText").and_then(|v| v.as_str()) {
            if !status_text.is_empty() {
                return Err(anyhow!("视频状态错误: {}", status_text));
            }
        }
        
        let data = json.pointer("/data")
            .ok_or_else(|| anyhow!("无法获取视频数据"))?;
        
        let mut info = VideoParseInfo::new();
        
        info.author = Author {
            uid: data.pointer("/author/id")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            name: data.pointer("/author/name")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            avatar: data.pointer("/author/icon")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
        };
        
        info.title = data.pointer("/meta/title")
            .and_then(|v| v.as_str())
            .or_else(|| data.pointer("/shareInfo/title").and_then(|v| v.as_str()))
            .unwrap_or("")
            .to_string();
        
        info.video_url = data.pointer("/meta/video_info/clarityUrl/1/url")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        info.cover_url = data.pointer("/meta/image")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        Ok(info)
    }
}

