use crate::models::{Author, ImgInfo, VideoParseInfo};
use crate::parser::VideoParser;
use crate::utils::{create_no_redirect_client, create_http_client};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use serde_json::Value;

pub struct PipixiaParser;

#[async_trait]
impl VideoParser for PipixiaParser {
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
        
        let parsed_url = url::Url::parse(location)?;
        let video_id = parsed_url.path()
            .trim_matches('/')
            .replace("item/", "");
        
        if video_id.is_empty() {
            return Err(anyhow!("无法从分享链接中解析视频ID"));
        }
        
        self.parse_video_id(&video_id).await
    }
    
    async fn parse_video_id(&self, video_id: &str) -> Result<VideoParseInfo> {
        let req_url = format!(
            "https://api.pipix.com/bds/cell/cell_comment/?offset=0&cell_type=1&api_version=1&cell_id={}&ac=wifi&channel=huawei_1319_64&aid=1319&app_name=super",
            video_id
        );
        
        let client = create_http_client()?;
        let response = client
            .get(&req_url)
            .header("User-Agent", crate::utils::DEFAULT_USER_AGENT)
            .send()
            .await?;
        
        let json: Value = response.json().await?;
        
        let data = json.pointer("/data/cell_comments/0/comment_info/item")
            .ok_or_else(|| anyhow!("无法获取视频数据"))?;
        
        let author_id = data.pointer("/author/id")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let mut images = Vec::new();
        if let Some(multi_image) = data.pointer("/note/multi_image").and_then(|v| v.as_array()) {
            for image_item in multi_image {
                if let Some(image_url) = image_item.pointer("/url_list/0/url").and_then(|v| v.as_str()) {
                    if !image_url.is_empty() {
                        images.push(ImgInfo {
                            url: image_url.to_string(),
                            live_photo_url: None,
                        });
                    }
                }
            }
        }
        
        let mut video_url = data.pointer("/video/video_high/url_list/0/url")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        if let Some(comments) = data.pointer("/comments").and_then(|v| v.as_array()) {
            for comment in comments {
                let comment_author_id = comment.pointer("/item/author/id")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                
                if comment_author_id == author_id {
                    if let Some(comment_video_url) = comment.pointer("/item/video/video_high/url_list/0/url")
                        .and_then(|v| v.as_str()) 
                    {
                        if !comment_video_url.is_empty() {
                            video_url = Some(comment_video_url.to_string());
                            break;
                        }
                    }
                }
            }
        }
        
        let mut info = VideoParseInfo::new();
        
        info.author = Author {
            uid: author_id.to_string(),
            name: data.pointer("/author/name")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            avatar: data.pointer("/author/avatar/download_list/0/url")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
        };
        
        info.title = data.pointer("/content")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        
        info.video_url = video_url;
        
        info.cover_url = data.pointer("/cover/url_list/0/url")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        info.images = images;
        
        Ok(info)
    }
}

