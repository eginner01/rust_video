use crate::models::{HttpResponse, VideoParseInfo, VideoSource};
use crate::parser::{parse_video_id, parse_video_share_url, get_supported_platforms};
use crate::utils::extract_url_from_string;
use axum::{
    body::Body,
    extract::{Query, ConnectInfo},
    http::{header, StatusCode, Method, Uri},
    middleware::{self, Next},
    response::{Html, IntoResponse, Json, Response},
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::time::Instant;
use tower_http::cors::CorsLayer;
use reqwest::Client;

async fn logger_middleware(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    method: Method,
    uri: Uri,
    req: axum::extract::Request,
    next: Next,
) -> Response {
    let start = Instant::now();
    let method_str = method.to_string();
    let path = uri.path().to_string();
    let query = uri.query().map(|q| format!("?{}", q)).unwrap_or_default();
    
    // 处理请求
    let response = next.run(req).await;
    
    let duration = start.elapsed();
    let status = response.status().as_u16();
    
    // 格式化响应时间
    let duration_str = if duration.as_secs() > 0 {
        format!("{:.2}s", duration.as_secs_f64())
    } else if duration.as_millis() > 0 {
        format!("{}ms", duration.as_millis())
    } else {
        format!("{}μs", duration.as_micros())
    };
    
    println!(
        "[RUST] {} | {:>3} | {:>8} | {:>15} | {:<6} \"{}{}\"",
        chrono::Local::now().format("%Y/%m/%d - %H:%M:%S"),
        status,
        duration_str,
        addr.ip(),
        method_str,
        path,
        query
    );
    
    response
}

pub async fn start_server(port: u16) -> anyhow::Result<()> {
    // 打印启动信息
    println!("\n╔══════════════════════════════════════════════════════════╗");
    println!("║     🦀 Rust Video Parser v2.1.0                         ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");
    
    println!("[RUST-debug] Registering routes:");
    println!("[RUST-debug] GET    /                          --> index_handler");
    println!("[RUST-debug] GET    /video/share/url/parse     --> parse_share_url_handler");
    println!("[RUST-debug] GET    /video/id/parse            --> parse_video_id_handler");
    println!("[RUST-debug] GET    /platforms                 --> platforms_handler");
    println!("[RUST-debug] GET    /proxy/video               --> proxy_video_handler");
    println!("[RUST-debug] GET    /proxy/image               --> proxy_image_handler\n");
    
    let app = Router::new()
        .route("/", get(index_handler))
        .route("/video/share/url/parse", get(parse_share_url_handler))
        .route("/video/id/parse", get(parse_video_id_handler))
        .route("/platforms", get(platforms_handler))
        .route("/proxy/video", get(proxy_video_handler))
        .route("/proxy/image", get(proxy_image_handler))
        .layer(middleware::from_fn(logger_middleware))
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("🚀 Listening on http://{}", addr);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await?;

    Ok(())
}

async fn index_handler() -> impl IntoResponse {
    Html(include_str!("../templates/index.html"))
}

#[derive(Debug, Deserialize)]
struct ParseShareUrlQuery {
    url: String,
}

async fn parse_share_url_handler(
    Query(params): Query<ParseShareUrlQuery>,
) -> Json<HttpResponse<VideoParseInfo>> {
    let url = match extract_url_from_string(&params.url) {
        Ok(url) => url,
        Err(e) => {
            return Json(HttpResponse::error(format!("URL提取失败: {}", e)));
        }
    };
    
    match parse_video_share_url(&url).await {
        Ok(info) => Json(HttpResponse::success(info)),
        Err(e) => Json(HttpResponse::error(format!("视频解析失败: {}", e))),
    }
}

#[derive(Debug, Deserialize)]
struct ParseVideoIdQuery {
    source: String,
    video_id: String,
}

async fn parse_video_id_handler(
    Query(params): Query<ParseVideoIdQuery>,
) -> Json<HttpResponse<VideoParseInfo>> {
    // 解析平台来源
    let source = match VideoSource::from_str(&params.source) {
        Some(s) => s,
        None => {
            return Json(HttpResponse::error(format!(
                "不支持的平台: {}",
                params.source
            )));
        }
    };

    // 解析视频
    match parse_video_id(source, &params.video_id).await {
        Ok(info) => Json(HttpResponse::success(info)),
        Err(e) => Json(HttpResponse::error(format!("视频解析失败: {}", e))),
    }
}

#[derive(Debug, Serialize)]
struct PlatformInfo {
    source: String,
    name: String,
    domains: Vec<String>,
}

async fn platforms_handler() -> Json<HttpResponse<Vec<PlatformInfo>>> {
    let platforms = get_supported_platforms()
        .into_iter()
        .map(|(source, name, domains)| PlatformInfo {
            source: source.as_str().to_string(),
            name: name.to_string(),
            domains: domains.iter().map(|s| s.to_string()).collect(),
        })
        .collect();

    Json(HttpResponse::success(platforms))
}

#[derive(Debug, Deserialize)]
struct ProxyQuery {
    url: String,
}

async fn proxy_video_handler(Query(params): Query<ProxyQuery>) -> impl IntoResponse {
    tracing::info!("🎬 代理视频请求: {}", params.url);
    
    let client = match Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .timeout(std::time::Duration::from_secs(30))
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("创建HTTP客户端失败: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                [(header::CONTENT_TYPE, "text/plain")],
                "创建HTTP客户端失败",
            )
                .into_response();
        }
    };
    
    let response = match client.get(&params.url).send().await {
        Ok(r) => r,
        Err(e) => {
            tracing::error!("获取视频失败: {}", e);
            return (
                StatusCode::BAD_GATEWAY,
                [(header::CONTENT_TYPE, "text/plain")],
                format!("获取视频失败: {}", e),
            )
                .into_response();
        }
    };

    // 获取响应头
    let content_type = response
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("video/mp4")
        .to_string();

    let content_length = response
        .headers()
        .get(reqwest::header::CONTENT_LENGTH)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("0")
        .to_string();

    // 获取响应体
    let bytes = match response.bytes().await {
        Ok(b) => b,
        Err(e) => {
            tracing::error!("读取视频数据失败: {}", e);
            return (
                StatusCode::BAD_GATEWAY,
                [(header::CONTENT_TYPE, "text/plain")],
                format!("读取视频数据失败: {}", e),
            )
                .into_response();
        }
    };

    tracing::info!("✅ 成功代理视频，大小: {} bytes", bytes.len());
    
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, content_type)
        .header(header::CONTENT_LENGTH, content_length)
        .header(header::CACHE_CONTROL, "public, max-age=31536000")
        .header(header::ACCEPT_RANGES, "bytes")
        .body(Body::from(bytes))
        .unwrap()
        .into_response()
}

async fn proxy_image_handler(Query(params): Query<ProxyQuery>) -> impl IntoResponse {
    tracing::debug!("🖼️ 代理图片请求: {}", params.url);

    // 创建HTTP客户端
    let client = match Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .timeout(std::time::Duration::from_secs(15))
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("创建HTTP客户端失败: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                [(header::CONTENT_TYPE, "text/plain")],
                "创建HTTP客户端失败",
            )
                .into_response();
        }
    };
    
    let response = match client.get(&params.url).send().await {
        Ok(r) => r,
        Err(e) => {
            tracing::error!("获取图片失败: {}", e);
            return (
                StatusCode::BAD_GATEWAY,
                [(header::CONTENT_TYPE, "text/plain")],
                format!("获取图片失败: {}", e),
            )
                .into_response();
        }
    };
    
    let content_type = response
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("image/jpeg")
        .to_string();
    
    let bytes = match response.bytes().await {
        Ok(b) => b,
        Err(e) => {
            tracing::error!("读取图片数据失败: {}", e);
            return (
                StatusCode::BAD_GATEWAY,
                [(header::CONTENT_TYPE, "text/plain")],
                format!("读取图片数据失败: {}", e),
            )
                .into_response();
        }
    };
    
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, content_type)
        .header(header::CACHE_CONTROL, "public, max-age=31536000")
        .body(Body::from(bytes))
        .unwrap()
        .into_response()
}

