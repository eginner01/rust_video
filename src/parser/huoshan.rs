use crate::models::VideoParseInfo;
use crate::parser::VideoParser;
use crate::utils::{create_no_redirect_client, create_http_client};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use serde_json::Value;

pub struct HuoshanParser;

#[async_trait]
impl VideoParser for HuoshanParser {
    async fn parse_share_url(&self, share_url: &str) -> Result<VideoParseInfo> {
        let client = create_no_redirect_client()?;
        
        let response = client
            .get(share_url)
            .header("User-Agent", crate::utils::DEFAULT_USER_AGENT)
            .send()
            .await?;
        
        let location = response.headers()
            .get("location")
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| anyhow!("无法获取重定向地址"))?;
        
        // 从location中提取item_id
        let parsed_url = url::Url::parse(location)?;
        let video_id = parsed_url.query_pairs()
            .find(|(key, _)| key == "item_id")
            .map(|(_, value)| value.to_string())
            .ok_or_else(|| anyhow!("无法从分享链接中解析视频ID"))?;
        
        self.parse_video_id(&video_id).await
    }
    
    async fn parse_video_id(&self, video_id: &str) -> Result<VideoParseInfo> {
        let req_url = format!("https://share.huoshan.com/api/item/info?item_id={}", video_id);
        
        let client = create_http_client()?;
        let response = client
            .get(&req_url)
            .header("User-Agent", crate::utils::DEFAULT_USER_AGENT)
            .send()
            .await?;
        
        let json: Value = response.json().await?;
        
        // 提取数据
        let data = json.pointer("/data/item_info")
            .ok_or_else(|| anyhow!("无法获取视频数据"))?;
        
        let mut info = VideoParseInfo::new();
        
        info.video_url = data.pointer("/url")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        info.cover_url = data.pointer("/cover")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        Ok(info)
    }
}

