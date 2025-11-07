use crate::models::{Author, VideoParseInfo};
use crate::parser::VideoParser;
use crate::utils::create_http_client;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use serde_json::{json, Value};

pub struct ZuiyouParser;

#[async_trait]
impl VideoParser for ZuiyouParser {
    async fn parse_share_url(&self, share_url: &str) -> Result<VideoParseInfo> {
        let parsed_url = url::Url::parse(share_url)?;
        
        let pid = parsed_url.query_pairs()
            .find(|(key, _)| key == "pid")
            .map(|(_, value)| value.to_string())
            .ok_or_else(|| anyhow!("无法从分享链接中解析视频ID"))?;
        
        let pid_int: i64 = pid.parse()
            .map_err(|_| anyhow!("无效的视频ID"))?;
        
        self.parse_video_by_pid(pid_int).await
    }
}

impl ZuiyouParser {
    async fn parse_video_by_pid(&self, pid: i64) -> Result<VideoParseInfo> {
        let post_data = json!({
            "h_av": "5.2.13.011",
            "pid": pid
        });
        
        let client = create_http_client()?;
        let response = client
            .post("https://share.xiaochuankeji.cn/planck/share/post/detail_h5")
            .header("User-Agent", crate::utils::DEFAULT_USER_AGENT)
            .json(&post_data)
            .send()
            .await?;
        
        let json: Value = response.json().await?;
        
        let data = json.pointer("/data/post")
            .ok_or_else(|| anyhow!("无法获取视频数据"))?;
        
        // 获取视频key
        let video_key = data.pointer("/imgs/0/id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("无法获取视频key"))?;
        
        let mut info = VideoParseInfo::new();
        
        // 使用video_key获取视频URL
        let video_path = format!("/videos/{}/url", video_key);
        info.video_url = data.pointer(&video_path)
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        // 获取封面
        let cover_path = format!("/videos/{}/cover_urls/0", video_key);
        info.cover_url = data.pointer(&cover_path)
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        info.title = data.pointer("/content")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        
        info.author = Author {
            uid: String::new(),
            name: data.pointer("/member/name")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            avatar: data.pointer("/member/avatar_urls/origin/urls/0")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
        };
        
        Ok(info)
    }
}

