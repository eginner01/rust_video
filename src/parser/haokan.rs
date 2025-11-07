use crate::models::{Author, VideoParseInfo};
use crate::parser::VideoParser;
use crate::utils::create_http_client;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use serde_json::Value;

pub struct HaokanParser;

#[async_trait]
impl VideoParser for HaokanParser {
    async fn parse_share_url(&self, share_url: &str) -> Result<VideoParseInfo> {
        let parsed_url = url::Url::parse(share_url)?;
        
        let video_id = parsed_url.query_pairs()
            .find(|(key, _)| key == "vid")
            .map(|(_, value)| value.to_string())
            .ok_or_else(|| anyhow!("无法从分享链接中解析视频ID"))?;
        
        self.parse_video_id(&video_id).await
    }
    
    async fn parse_video_id(&self, video_id: &str) -> Result<VideoParseInfo> {
        let req_url = format!("https://haokan.baidu.com/v?_format=json&vid={}", video_id);
        
        let client = create_http_client()?;
        let response = client
            .get(&req_url)
            .header("User-Agent", crate::utils::DEFAULT_USER_AGENT)
            .send()
            .await?;
        
        let json: Value = response.json().await?;
        
        // 检查返回状态
        let errno = json.pointer("/errno")
            .and_then(|v| v.as_i64())
            .unwrap_or(-1);
        
        if errno != 0 {
            let error = json.pointer("/error")
                .and_then(|v| v.as_str())
                .unwrap_or("API返回错误");
            return Err(anyhow!("好看视频API错误: {}", error));
        }
        
        let data = json.pointer("/data/apiData/curVideoMeta")
            .ok_or_else(|| anyhow!("无法获取视频数据"))?;
        
        let mut info = VideoParseInfo::new();
        
        info.title = data.pointer("/title")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        
        info.video_url = data.pointer("/playurl")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        info.cover_url = data.pointer("/poster")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        info.author = Author {
            uid: data.pointer("/mth/mthid")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            name: data.pointer("/mth/author_name")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            avatar: data.pointer("/mth/author_photo")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
        };
        
        Ok(info)
    }
}

