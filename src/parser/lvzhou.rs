use crate::models::VideoParseInfo;
use crate::parser::VideoParser;
use crate::utils::create_http_client;
use anyhow::Result;
use async_trait::async_trait;
use regex::Regex;
use scraper::{Html, Selector};

pub struct LvzhouParser;

#[async_trait]
impl VideoParser for LvzhouParser {
    async fn parse_share_url(&self, share_url: &str) -> Result<VideoParseInfo> {
        let client = create_http_client()?;
        
        let html = client
            .get(share_url)
            .header("User-Agent", crate::utils::DEFAULT_USER_AGENT)
            .send()
            .await?
            .text()
            .await?;
        
        self.parse_html(&html)
    }
    
    async fn parse_video_id(&self, video_id: &str) -> Result<VideoParseInfo> {
        let share_url = format!("https://m.oasis.weibo.cn/v1/h5/share?sid={}", video_id);
        self.parse_share_url(&share_url).await
    }
}

impl LvzhouParser {
    fn parse_html(&self, html: &str) -> Result<VideoParseInfo> {
        let document = Html::parse_document(html);
        
        let mut info = VideoParseInfo::new();
        
        // 提取视频URL
        let video_selector = Selector::parse("video").unwrap();
        if let Some(video_element) = document.select(&video_selector).next() {
            if let Some(src) = video_element.value().attr("src") {
                info.video_url = Some(src.to_string());
            }
        }
        
        // 提取作者头像
        let avatar_selector = Selector::parse("a.avatar img").unwrap();
        if let Some(avatar_element) = document.select(&avatar_selector).next() {
            if let Some(src) = avatar_element.value().attr("src") {
                info.author.avatar = src.to_string();
            }
        }
        
        // 提取封面
        let cover_selector = Selector::parse("div.video-cover").unwrap();
        if let Some(cover_element) = document.select(&cover_selector).next() {
            if let Some(style) = cover_element.value().attr("style") {
                let re = Regex::new(r"background-image:url\((.*?)\)")?;
                if let Some(caps) = re.captures(style) {
                    if let Some(cover_url) = caps.get(1) {
                        info.cover_url = Some(cover_url.as_str().to_string());
                    }
                }
            }
        }
        
        // 提取标题
        let title_selector = Selector::parse("div.status-title").unwrap();
        if let Some(title_element) = document.select(&title_selector).next() {
            info.title = title_element.text().collect::<String>().trim().to_string();
        }
        
        // 提取作者昵称
        let nickname_selector = Selector::parse("div.nickname").unwrap();
        if let Some(nickname_element) = document.select(&nickname_selector).next() {
            info.author.name = nickname_element.text().collect::<String>().trim().to_string();
        }
        
        Ok(info)
    }
}

