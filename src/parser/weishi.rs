use crate::models::{Author, VideoParseInfo};
use crate::parser::VideoParser;
use crate::utils::create_http_client;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use serde_json::Value;

pub struct WeishiParser;

#[async_trait]
impl VideoParser for WeishiParser {
    async fn parse_share_url(&self, share_url: &str) -> Result<VideoParseInfo> {
        let parsed_url = url::Url::parse(share_url)?;
        
        let video_id = parsed_url.query_pairs()
            .find(|(key, _)| key == "id")
            .map(|(_, value)| value.to_string())
            .ok_or_else(|| anyhow!("无法从分享链接中解析视频ID"))?;
        
        self.parse_video_id(&video_id).await
    }
    
    async fn parse_video_id(&self, video_id: &str) -> Result<VideoParseInfo> {
        let req_url = format!("https://h5.weishi.qq.com/webapp/json/weishi/WSH5GetPlayPage?feedid={}", video_id);
        
        let client = create_http_client()?;
        let response = client
            .get(&req_url)
            .header("User-Agent", crate::utils::DEFAULT_USER_AGENT)
            .send()
            .await?;
        
        let json: Value = response.json().await?;
        
        // 检查返回状态
        let ret = json.pointer("/ret")
            .and_then(|v| v.as_i64())
            .unwrap_or(-1);
        
        if ret != 0 {
            let msg = json.pointer("/msg")
                .and_then(|v| v.as_str())
                .unwrap_or("API返回错误");
            return Err(anyhow!("微视API错误: {}", msg));
        }
        
        // 检查视频状态
        if let Some(err_msg) = json.pointer("/data/errmsg").and_then(|v| v.as_str()) {
            if !err_msg.is_empty() {
                return Err(anyhow!("视频错误: {}", err_msg));
            }
        }
        
        // 提取数据
        let data = json.pointer("/data/feeds/0")
            .ok_or_else(|| anyhow!("无法获取视频数据"))?;
        
        let mut info = VideoParseInfo::new();
        
        info.author = Author {
            uid: String::new(),
            name: data.pointer("/poster/nick")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            avatar: data.pointer("/poster/avatar")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
        };
        
        info.title = data.pointer("/feed_desc_withat")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        
        info.video_url = data.pointer("/video_url")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        info.cover_url = data.pointer("/images/0/url")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        Ok(info)
    }
}

