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
            .ok_or_else(|| anyhow!("无法获取数据"))?;
        
        let mut info = VideoParseInfo::new();
        
        // 获取标题和作者信息
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
        
        // 检查是否有videos字段（视频内容）
        if let Some(videos) = data.get("videos").and_then(|v| v.as_object()) {
            // 获取第一个视频的key
            if let Some((_video_key, video_data)) = videos.iter().next() {
                // 获取视频URL
                info.video_url = video_data.pointer("/url")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                
                // 获取封面URL - 确保使用jpg格式
                if let Some(cover_urls) = video_data.pointer("/cover_urls").and_then(|v| v.as_array()) {
                    if let Some(cover) = cover_urls.first().and_then(|v| v.as_str()) {
                        // 确保封面是jpg格式
                        info.cover_url = Some(Self::ensure_jpg_format(cover));
                    }
                }
                
                // 如果没有封面，尝试从imgs获取
                if info.cover_url.is_none() {
                    if let Some(imgs_array) = data.pointer("/imgs").and_then(|v| v.as_array()) {
                        if let Some(first_img) = imgs_array.first() {
                            if let Some(img_url) = first_img.pointer("/url").and_then(|v| v.as_str()) {
                                info.cover_url = Some(Self::ensure_jpg_format(img_url));
                            }
                        }
                    }
                }
            }
        }
        
        // 检查是否有imgs字段（图片内容）
        if let Some(imgs_array) = data.pointer("/imgs").and_then(|v| v.as_array()) {
            for img in imgs_array {
                if let Some(img_url) = img.pointer("/url").and_then(|v| v.as_str()) {
                    // 确保图片URL是jpg格式
                    let img_url = Self::ensure_jpg_format(img_url);
                    
                    let img_info = crate::models::ImgInfo {
                        url: img_url,
                        live_photo_url: None,
                    };
                    info.images.push(img_info);
                }
            }
        }
        
        // 如果既没有视频也没有图片，返回错误
        if info.video_url.is_none() && info.images.is_empty() {
            return Err(anyhow!("未找到视频或图片内容"));
        }
        
        Ok(info)
    }
    
    /// 确保URL是jpg格式
    fn ensure_jpg_format(url: &str) -> String {
        // 如果URL已经包含jpg后缀，直接返回
        if url.contains(".jpg") || url.contains(".jpeg") {
            return url.to_string();
        }
        
        // 如果URL包含其他图片格式，替换为jpg
        let url = url.replace(".png", ".jpg")
            .replace(".webp", ".jpg")
            .replace(".gif", ".jpg");
        
        // 如果URL包含查询参数，确保在参数前添加.jpg
        if url.contains('?') && !url.contains(".jpg") {
            url.replace('?', ".jpg?")
        } else if !url.contains(".jpg") {
            // 如果没有任何扩展名，添加.jpg
            format!("{}.jpg", url)
        } else {
            url
        }
    }
}

