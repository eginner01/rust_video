use crate::models::VideoParseInfo;
use crate::parser::VideoParser;
use crate::utils::create_http_client;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use serde_json::{json, Value};

pub struct PipigaoxiaoParser;

#[async_trait]
impl VideoParser for PipigaoxiaoParser {
    async fn parse_share_url(&self, share_url: &str) -> Result<VideoParseInfo> {
        let parsed_url = url::Url::parse(share_url)?;
        
        let video_id = parsed_url.path()
            .replace("/pp/post/", "");
        
        if video_id.is_empty() {
            return Err(anyhow!("无法从分享链接中解析视频ID"));
        }
        
        self.parse_video_id(&video_id).await
    }
    
    async fn parse_video_id(&self, video_id: &str) -> Result<VideoParseInfo> {
        let req_url = "https://share.ippzone.com/ppapi/share/fetch_content";
        
        let post_data = json!({
            "pid": video_id.parse::<i64>().unwrap_or(0),
            "type": "post",
            "mid": null
        });
        
        let client = create_http_client()?;
        let response = client
            .post(req_url)
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36")
            .header("Referer", req_url)
            .json(&post_data)
            .send()
            .await?;
        
        let json: Value = response.json().await?;
        
        // 检查是否有错误消息
        if let Some(msg) = json.pointer("/msg") {
            if msg.is_string() {
                return Err(anyhow!("皮皮搞笑API错误: {}", msg.as_str().unwrap()));
            }
        }
        
        let data = json.pointer("/data/post")
            .ok_or_else(|| anyhow!("无法获取视频数据"))?;
        
        let id = data.pointer("/imgs/0/id")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        
        let video_path = format!("/videos/{}/url", id);
        let video_url = data.pointer(&video_path)
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        let cover_url = format!("https://file.ippzone.com/img/view/id/{}", id);
        
        let mut info = VideoParseInfo::new();
        
        info.title = data.pointer("/content")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        
        info.video_url = video_url;
        info.cover_url = Some(cover_url);
        
        Ok(info)
    }
}

