use crate::models::{Author, ImgInfo, VideoParseInfo};
use crate::parser::VideoParser;
use crate::utils::{create_http_client, extract_json_from_html};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use serde_json::Value;

pub struct RedbookParser;

#[async_trait]
impl VideoParser for RedbookParser {
    async fn parse_share_url(&self, share_url: &str) -> Result<VideoParseInfo> {
        let client = create_http_client()?;

        let html = client
            .get(share_url)
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/129.0.0.0 Safari/537.36 Edg/129.0.0.0")
            .send()
            .await?
            .text()
            .await?;
        
        // 保存HTML用于调试
        if let Err(e) = std::fs::write("debug_redbook.html", &html) {
            tracing::warn!("无法保存调试HTML: {}", e);
        }
        
        // 提取JSON数据
        let pattern = r"window\.__INITIAL_STATE__\s*=\s*(.*?)</script>";
        let json_str = extract_json_from_html(&html, pattern)?;
        
        tracing::debug!("提取的JSON长度: {} 字符", json_str.len());

        let mut json_str = json_str.trim().trim_start_matches('\u{feff}').to_string();

        use regex::Regex;
        let undefined_re = Regex::new(r":undefined\b|,undefined\b|\[undefined\b").unwrap();
        json_str = undefined_re.replace_all(&json_str, |caps: &regex::Captures| {
            let matched = caps.get(0).unwrap().as_str();
            if matched.starts_with(':') {
                ":null"
            } else if matched.starts_with(',') {
                ",null"
            } else {
                "[null"
            }
        }).to_string();
        
        tracing::debug!("已将undefined替换为null");

        if let Err(e) = std::fs::write("debug_redbook_cleaned.json", &json_str) {
            tracing::warn!("无法保存调试JSON: {}", e);
        }

        let json: Value = serde_json::from_str(&json_str).map_err(|e| {
            tracing::error!("JSON解析失败: {}", e);
            tracing::error!("JSON前100字符: {}", &json_str.chars().take(100).collect::<String>());
            if json_str.len() > 100 {
                let start = e.column().saturating_sub(50);
                let end = (e.column() + 50).min(json_str.len());
                tracing::error!("错误位置附近: {}", &json_str[start..end]);
            }
            anyhow!("无法解析JSON数据: {}，已保存调试文件到 debug_redbook.html 和 debug_redbook.json", e)
        })?;
        
        tracing::info!("✅ JSON解析成功");

        let note_id = json.pointer("/note/currentNoteId")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("无法获取note ID"))?;
        
        tracing::debug!("Note ID: {}", note_id);

        let note_path = format!("/note/noteDetailMap/{}/note", note_id);
        let note = json.pointer(&note_path)
            .ok_or_else(|| anyhow!("无法获取note数据，路径: {}", note_path))?;
        
        self.extract_video_info(note)
    }
}

impl RedbookParser {
    /// 从JSON数据中提取视频信息
    fn extract_video_info(&self, note: &Value) -> Result<VideoParseInfo> {
        let mut info = VideoParseInfo::new();
        
        // 提取作者信息
        info.author = Author {
            uid: note.pointer("/user/userId")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            name: note.pointer("/user/nickname")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            avatar: note.pointer("/user/avatar")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
        };


        info.title = note.pointer("/title")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        
        // 提取视频URL
        info.video_url = note.pointer("/video/media/stream/h264/0/masterUrl")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        // 提取封面
        info.cover_url = note.pointer("/imageList/0/urlDefault")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        // 如果没有视频，提取图集
        if info.video_url.is_none() {
            if let Some(image_list) = note.pointer("/imageList").and_then(|v| v.as_array()) {
                for image in image_list {
                    if let Some(url) = image.pointer("/urlDefault").and_then(|v| v.as_str()) {
                        if url.is_empty() {
                            continue;
                        }
                        
                        // 处理图片URL
                        let img_url = self.process_image_url(url);
                        
                        // 检查是否有LivePhoto
                        let live_photo_url = if image.pointer("/livePhoto").and_then(|v| v.as_bool()).unwrap_or(false) {
                            image.pointer("/stream/h264")
                                .and_then(|v| v.as_array())
                                .and_then(|arr| arr.first())
                                .and_then(|v| v.pointer("/masterUrl"))
                                .and_then(|v| v.as_str())
                                .map(|s| s.to_string())
                        } else {
                            None
                        };
                        
                        info.images.push(ImgInfo {
                            url: img_url,
                            live_photo_url,
                        });
                    }
                }
            }
        }
        
        Ok(info)
    }
    
    /// 处理图片URL
    fn process_image_url(&self, url: &str) -> String {
        // 提取图片ID
        if let Some(slash_pos) = url.rfind('/') {
            let after_slash = &url[slash_pos + 1..];
            if let Some(exclaim_pos) = after_slash.find('!') {
                let img_id = &after_slash[..exclaim_pos];
                
                // 检查是否包含 spectrum/
                let spectrum_str = if url.contains("spectrum") {
                    "spectrum/"
                } else {
                    ""
                };

                if url.contains("notes_pre_post") {
                    return format!(
                        "https://ci.xiaohongshu.com/notes_pre_post/{}{}?imageView2/format/jpg",
                        spectrum_str, img_id
                    );
                }
            }
        }
        
        // 无法处理，返回原URL
        url.to_string()
    }
}

