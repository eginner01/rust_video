use crate::models::{Author, VideoParseInfo};
use crate::parser::VideoParser;
use crate::utils::create_http_client;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use serde::Deserialize;

pub struct BilibiliParser;

#[derive(Debug, Deserialize)]
struct BiliViewResponse {
    code: i32,
    message: String,
    data: BiliViewData,
}

#[derive(Debug, Deserialize)]
struct BiliViewData {
    #[allow(dead_code)]
    bvid: String,
    title: String,
    pic: String,
    owner: BiliOwner,
    pages: Vec<BiliPage>,
}

#[derive(Debug, Deserialize)]
struct BiliOwner {
    mid: i64,
    name: String,
    face: String,
}

#[derive(Debug, Deserialize)]
struct BiliPage {
    cid: i64,
}

#[derive(Debug, Deserialize)]
struct BiliPlayResponse {
    code: i32,
    message: String,
    data: BiliPlayData,
}

#[derive(Debug, Deserialize)]
struct BiliPlayData {
    durl: Vec<BiliDurl>,
}

#[derive(Debug, Deserialize)]
struct BiliDurl {
    url: String,
}

#[async_trait]
impl VideoParser for BilibiliParser {
    async fn parse_share_url(&self, share_url: &str) -> Result<VideoParseInfo> {
        // 提取BVID
        let bvid = self.extract_bvid(share_url).await?;
        
        // 获取视频信息
        let view_url = format!("https://api.bilibili.com/x/web-interface/view?bvid={}", bvid);
        let client = create_http_client()?;
        
        let view_resp: BiliViewResponse = client
            .get(&view_url)
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
            .header("Referer", "https://www.bilibili.com/")
            .send()
            .await?
            .json()
            .await?;
        
        if view_resp.code != 0 {
            return Err(anyhow!("B站API返回错误: {} (code: {})", view_resp.message, view_resp.code));
        }
        
        let view_data = view_resp.data;
        
        // 获取第一个分P的cid
        let cid = view_data.pages.first()
            .ok_or_else(|| anyhow!("没有找到视频分P"))?
            .cid;
        
        // 获取播放链接
        let play_url = format!(
            "https://api.bilibili.com/x/player/playurl?otype=json&fnver=0&fnval=0&qn=80&bvid={}&cid={}&platform=html5",
            bvid, cid
        );
        
        let play_resp: BiliPlayResponse = client
            .get(&play_url)
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
            .header("Referer", "https://www.bilibili.com/")
            .send()
            .await?
            .json()
            .await?;
        
        if play_resp.code != 0 {
            return Err(anyhow!("B站播放API返回错误: {} (code: {})", play_resp.message, play_resp.code));
        }
        
        let video_url = play_resp.data.durl.first()
            .ok_or_else(|| anyhow!("未找到视频播放地址"))?
            .url.clone();
        
        // 构建返回结果
        let mut info = VideoParseInfo::new();
        info.author = Author {
            uid: view_data.owner.mid.to_string(),
            name: view_data.owner.name,
            avatar: view_data.owner.face,
        };
        info.title = view_data.title;
        info.video_url = Some(video_url);
        info.cover_url = Some(view_data.pic);
        
        Ok(info)
    }
}

impl BilibiliParser {
    /// 提取BVID
    async fn extract_bvid(&self, url: &str) -> Result<String> {
        let parsed_url = url::Url::parse(url)?;
        
        // 处理 b23.tv 短链
        if parsed_url.host_str() == Some("b23.tv") {
            return Box::pin(self.resolve_short_url(url)).await;
        }
        
        // 处理 bilibili.com 链接
        if let Some(host) = parsed_url.host_str() {
            if host.contains("bilibili.com") {
                let path = parsed_url.path();
                let parts: Vec<&str> = path.trim_matches('/').split('/').collect();
                
                if parts.len() >= 2 && parts[0] == "video" {
                    if parts[1].starts_with("BV") {
                        return Ok(parts[1].to_string());
                    }
                }
            }
        }
        
        Err(anyhow!("不是有效的B站视频链接"))
    }
    
    async fn resolve_short_url(&self, url: &str) -> Result<String> {
        let client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()?;
        
        let response = client.get(url).send().await?;
        
        let location = response
            .headers()
            .get("location")
            .ok_or_else(|| anyhow!("无法从b23.tv获取重定向链接"))?
            .to_str()?;
        
        Box::pin(self.extract_bvid(location)).await
    }
}

