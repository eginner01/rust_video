use crate::models::{Author, VideoParseInfo};
use crate::parser::VideoParser;
use crate::utils::create_http_client;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use scraper::{Html, Selector};
use serde_json::Value;

pub struct XinpianchangParser;

#[async_trait]
impl VideoParser for XinpianchangParser {
    async fn parse_share_url(&self, share_url: &str) -> Result<VideoParseInfo> {
        let client = create_http_client()?;
        
        let html = client
            .get(share_url)
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
            .header("Upgrade-Insecure-Requests", "1")
            .header("Referer", "https://www.xinpianchang.com/")
            .send()
            .await?
            .text()
            .await?;
        
        let document = Html::parse_document(&html);
        
        // 查找 __NEXT_DATA__ script标签
        let script_selector = Selector::parse("#__NEXT_DATA__").unwrap();
        
        let json_str = document
            .select(&script_selector)
            .next()
            .map(|element| element.text().collect::<String>())
            .ok_or_else(|| anyhow!("无法找到视频数据"))?;
        
        let json: Value = serde_json::from_str(&json_str)?;
        
        let data = json.pointer("/props/pageProps/detail")
            .ok_or_else(|| anyhow!("无法获取视频数据"))?;
        
        let mut info = VideoParseInfo::new();
        
        info.title = data.pointer("/title")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        
        info.video_url = data.pointer("/video/content/progressive/0/url")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        info.cover_url = data.pointer("/cover")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        info.author = Author {
            uid: String::new(),
            name: data.pointer("/author/userinfo/username")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            avatar: data.pointer("/author/userinfo/avatar")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
        };
        
        Ok(info)
    }
}

