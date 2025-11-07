mod models;
mod parser;
mod server;
mod utils;

use clap::{Parser as ClapParser, Subcommand};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(ClapParser)]
#[command(name = "rust_video_parser")]
#[command(author = "Rust Video Parser Team")]
#[command(version = "1.0.0")]
#[command(about = "短视频去水印解析工具 - 支持20+平台", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// HTTP服务器端口
    #[arg(short, long, default_value = "8080")]
    port: u16,
}

#[derive(Subcommand)]
enum Commands {
    /// 启动HTTP服务器
    Serve {
        #[arg(short, long, default_value = "8080")]
        port: u16,
    },
    Parse {
        url: String,
    },
    Platforms,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_video_parser=info".into()),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(false)
                .with_thread_ids(false)
                .with_thread_names(false)
                .with_file(false)
                .with_line_number(false)
                .compact(),
        )
        .init();

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Serve { port }) => {
            server::start_server(port).await?;
        }
        Some(Commands::Parse { url }) => {
            parse_video(&url).await?;
        }
        Some(Commands::Platforms) => {
            list_platforms();
        }
        None => {
            server::start_server(cli.port).await?;
        }
    }

    Ok(())
}

/// 解析视频并打印结果
async fn parse_video(url_text: &str) -> anyhow::Result<()> {
    use crate::utils::extract_url_from_string;
    use crate::parser::parse_video_share_url;

    println!("🔍 正在解析视频...\n");

    // 提取URL
    let url = extract_url_from_string(url_text)?;
    println!("📎 提取到URL: {}\n", url);

    // 解析视频
    match parse_video_share_url(&url).await {
        Ok(info) => {
            println!("✅ 解析成功!\n");
            println!("📺 标题: {}", info.title);
            println!("👤 作者: {} ({})", info.author.name, info.author.uid);
            
            if let Some(video_url) = &info.video_url {
                println!("🎬 视频地址: {}", video_url);
            }
            
            if let Some(cover_url) = &info.cover_url {
                println!("🖼️  封面地址: {}", cover_url);
            }
            
            if !info.images.is_empty() {
                println!("🎨 图集 ({} 张):", info.images.len());
                for (i, img) in info.images.iter().enumerate() {
                    println!("  [{}] {}", i + 1, img.url);
                    if let Some(live_photo) = &img.live_photo_url {
                        println!("      LivePhoto: {}", live_photo);
                    }
                }
            }

            println!("\n📋 JSON格式:");
            println!("{}", serde_json::to_string_pretty(&info)?);
        }
        Err(e) => {
            println!("❌ 解析失败: {}", e);
        }
    }

    Ok(())
}

fn list_platforms() {
    use crate::parser::get_supported_platforms;

    println!("🎯 支持的视频平台:\n");
    
    let platforms = get_supported_platforms();
    for (i, (source, name, domains)) in platforms.iter().enumerate() {
        println!("{}. {} ({})", i + 1, name, source.as_str());
        println!("   支持域名: {}", domains.join(", "));
        println!();
    }
    
    println!("总计支持 {} 个平台", platforms.len());
}
