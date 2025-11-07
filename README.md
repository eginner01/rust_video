<div align="center">

# 🎬 Rust Video Parser

**高性能视频解析工具 - 支持 21 个主流平台**

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Docker](https://img.shields.io/badge/docker-ready-blue.svg)](https://hub.docker.com)
[![Version](https://img.shields.io/badge/version-2.2.1-brightgreen.svg)](#)

[功能特性](#-功能特性) • [快速开始](#-快速开始) • [API 文档](#-api-文档) • [部署指南](#-部署方式) • [开发说明](#-开发)

</div>

---

## 📖 简介

Rust Video Parser 是一个使用 Rust 开发的高性能短视频去水印解析工具，支持抖音、快手、B站、小红书等 21 个主流平台。提供 Web 界面和 RESTful API，支持视频、图集、封面解析和在线下载

**核心特性**：
- 🚀 高性能：Rust 异步运行时，2500+ req/s 并发
- 🎨 现代界面：5 种主题，科幻粒子特效，响应式设计
- 🐳 快速部署：Docker 一键启动，零依赖运行
- 🔒 安全稳定：内存安全，完整错误处理

---

## ✨ 功能特性

### 🎯 视频解析
- ✅ 支持 **21 个平台**：抖音、快手、B站、小红书、微博等
- ✅ **多种内容**：视频、图集、LivePhoto、封面
- ✅ **无水印下载**：去除平台水印
- ✅ **在线播放**：内置视频播放器

### 🎨 用户界面
- ✅ **5 种主题**：科技蓝、优雅紫、商务灰、清新绿、热情橙
- ✅ **响应式设计**：完美适配桌面端和移动端
- ✅ **操作便捷**：独立卡片设计，操作按钮前置

### 🔧 技术特性
- ✅ **HTTP API**：RESTful 接口，支持跨域
- ✅ **视频代理**：绕过 CORS 限制
- ✅ **异步处理**：Tokio 异步运行时
- ✅ **日志记录**：结构化日志，易于调试

---

## 🎯 支持平台

<div align="center">

| 抖音 | 快手 | B站 | 小红书 | 微博 | 西瓜 | 火山 |
|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |

| 皮皮虾 | 微视 | 最右 | 绿洲 | 全民 | 梨视频 | 好看 |
|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |

| 虎牙 | 皮皮搞笑 | AcFun | 豆拍 | K歌 | 六间房 | 新片场 |
|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |

</div>

---

## 🚀 快速开始

### 方法 1: Docker 部署（推荐）

```bash
# 使用 Docker Compose
docker-compose up -d

# 或使用 Docker 命令
docker run -d -p 8080:8080 --name rust_video_parser rust_video_parser:latest
```

访问：http://localhost:8080

### 方法 2: 源码编译

```bash
# 1. 克隆项目
git clone https://github.com/your-username/rust_video_parser.git
cd rust_video_parser

# 2. 编译（需要 Rust 1.70+）
cargo build --release

# 3. 运行
./target/release/rust_video_parser

# 4. 访问
open http://localhost:8080
```

### 方法 3: 预编译二进制

从 [Releases](https://github.com/your-username/rust_video_parser/releases) 下载对应平台的二进制文件：

```bash
# Linux/macOS
chmod +x rust_video_parser
./rust_video_parser

# Windows
rust_video_parser.exe
```

---

## 💻 使用方式

### Web 界面

1. 打开浏览器访问 `http://localhost:8080`
2. 粘贴视频分享链接
3. 点击 **"🚀 立即解析"**
4. 查看结果，在线播放或下载

**支持的链接格式**：
```
https://v.douyin.com/xxxxxx/
2.02 复制打开抖音，看看【xxx】 https://v.douyin.com/xxx/
http://xhslink.com/xxxxx
```

### 命令行工具

```bash
# 解析视频
rust_video_parser parse "https://v.douyin.com/xxxxxx/"

# 列出支持的平台
rust_video_parser platforms

# 指定端口启动
rust_video_parser --port 3000
```

---

## 📡 API 文档

### 1. 解析视频链接

**接口**: `GET /video/share/url/parse`

**参数**:
| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| url | string | 是 | 视频分享链接或包含链接的文本 |

**请求示例**:
```bash
curl "http://localhost:8080/video/share/url/parse?url=https://v.douyin.com/xxxxxx/"
```

**响应示例**:
```json
{
  "code": 200,
  "msg": "解析成功",
  "data": {
    "author": {
      "uid": "MS4wLjABAAAA...",
      "name": "作者昵称",
      "avatar": "https://p3.douyinpic.com/..."
    },
    "title": "视频标题",
    "video_url": "https://v3-web.douyinvod.com/...",
    "cover_url": "https://p3.douyinpic.com/...",
    "images": []
  }
}
```

### 2. 视频 ID 解析

**接口**: `GET /video/id/parse`

**参数**:
| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| source | string | 是 | 平台标识 (douyin, kuaishou, bilibili, redbook) |
| video_id | string | 是 | 视频 ID |

**请求示例**:
```bash
curl 'http://127.0.0.1:8080/video/share/url/parse?url=视频分享链接' | jq
```

### 3. 视频代理

**接口**: `GET /proxy/video?url={video_url}`

用于绕过 CORS 限制，代理视频资源。

### 4. 图片代理

**接口**: `GET /proxy/image?url={image_url}`

用于绕过 CORS 限制，代理图片资源。

### 5. 支持平台列表

**接口**: `GET /platforms`

返回所有支持的平台列表。

---

## 🐳 部署方式

### Docker Compose（推荐）

创建 `docker-compose.yml`:

```yaml
version: '3.8'

services:
  rust_video_parser:
    image: rust_video_parser:latest
    container_name: rust_video_parser
    restart: unless-stopped
    ports:
      - "8080:8080"
    environment:
      - RUST_LOG=info
```

启动：
```bash
docker-compose up -d
```

### Systemd 服务

创建 `/etc/systemd/system/rust-video-parser.service`:

```ini
[Unit]
Description=Rust Video Parser
After=network.target

[Service]
Type=simple
User=www-data
WorkingDirectory=/opt/rust_video_parser
ExecStart=/opt/rust_video_parser/rust_video_parser
Restart=always

[Install]
WantedBy=multi-user.target
```

启动服务：
```bash
sudo systemctl enable rust-video-parser
sudo systemctl start rust-video-parser
```

### Nginx 反向代理

```nginx
server {
    listen 80;
    server_name video.example.com;

    location / {
        proxy_pass http://localhost:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    }
}
```

---

## 🧪 测试

项目包含 Python 测试脚本：

```bash
# 安装依赖
pip install requests

# 运行测试
python test_api.py

# 或使用 PowerShell 脚本（Windows）
.\run_test.ps1
```

测试覆盖 3 个平台（抖音视频、抖音图文、小红书），自动验证 API 功能。

---

## 🛠️ 开发

### 项目结构

```
rust_vedio/
├── src/
│   ├── main.rs              # 程序入口
│   ├── models.rs            # 数据模型
│   ├── server.rs            # HTTP 服务器
│   ├── utils.rs             # 工具函数
│   └── parser/              # 解析器模块 (21 个平台)
├── templates/
│   └── index.html           # 前端界面
├── Cargo.toml               # 项目配置
├── Dockerfile               # Docker 配置
└── test_api.py              # API 测试脚本
```

### 技术栈

| 组件 | 技术 |
|------|------|
| **Web 框架** | Axum 0.7 |
| **异步运行时** | Tokio 1.x |
| **HTTP 客户端** | Reqwest 0.11 |
| **HTML 解析** | Scraper 0.18 |
| **JSON 处理** | Serde JSON 1.0 |
| **日志** | Tracing 0.1 |

### 添加新平台

1. 在 `src/parser/` 创建新解析器：

```rust
use crate::models::VideoParseInfo;
use crate::parser::VideoParser;
use anyhow::Result;
use async_trait::async_trait;

pub struct NewPlatformParser;

#[async_trait]
impl VideoParser for NewPlatformParser {
    async fn parse_share_url(&self, share_url: &str) -> Result<VideoParseInfo> {
        // 实现解析逻辑
        todo!()
    }
}
```

2. 在 `src/parser/mod.rs` 注册解析器
3. 在 `src/models.rs` 添加平台枚举

### 代码规范

```bash
# 格式化代码
cargo fmt

# 代码检查
cargo clippy

# 运行测试
cargo test
```

---

## ⚙️ 配置

### 环境变量

| 变量 | 说明 | 默认值 |
|------|------|--------|
| `RUST_LOG` | 日志级别 | `info` |
| `SERVER_PORT` | 服务端口 | `8080` |

### 命令行参数

```bash
rust_video_parser [OPTIONS] [COMMAND]

OPTIONS:
    -p, --port <PORT>    设置服务端口 [default: 8080]
    -h, --help           显示帮助信息
    -V, --version        显示版本信息

COMMANDS:
    serve      启动 HTTP 服务器
    parse      解析视频链接
    platforms  列出支持的平台
```

---

## 📊 性能指标

| 指标 | 数值 |
|------|------|
| **并发处理** | 2,500 req/s |
| **内存占用** | ~8 MB (idle) |
| **启动时间** | ~50 ms |
| **响应时间** | < 500 ms |
| **二进制大小** | ~6 MB (release) |

---

## 🤝 贡献

欢迎贡献代码！请遵循以下步骤：

1. Fork 本项目
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 提交 Pull Request

### 贡献指南

- 遵循 Rust 代码规范
- 添加必要的注释和文档
- 编写单元测试
- 更新 CHANGELOG.md

---

## 📄 许可证

本项目采用 [MIT](LICENSE) 许可证。

---

## ⚠️ 免责声明

- 本工具仅供**学习交流**使用
- 请勿用于**商业用途**
- 解析内容版权归**原作者所有**
- 请**尊重原创**，合理使用

---

## 🙏 致谢

- 感谢原 Go 版本 [parse-video](https://github.com/wujunwei928/parse-video)
- 感谢 Rust 社区的优秀生态系统
- 感谢所有贡献者和用户

---
## 🌟 Star History

如果这个项目对你有帮助，请给个 **Star** ⭐

[![Star History Chart](https://api.star-history.com/svg?repos=your-username/rust_video_parser&type=Date)](https://star-history.com/#your-username/rust_video_parser&Date)

---

<div align="center">

**Made with ❤️ and 🦀 Rust**

[⬆ 返回顶部](#-rust-video-parser)

</div>
docker run -d \
  --name rust_video_parser \
  -p 8088:8080 \
  -e RUST_LOG=info \
  -v $(pwd)/logs:/app/logs \
  --restart unless-stopped \
  p0jy2tygm7zcbv.xuanyuan.run/rust_video_parser:latest