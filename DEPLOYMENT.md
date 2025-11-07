# 🚀 Rust Video Parser - 部署指南

## 📦 Docker Hub 镜像

**镜像地址：** `eginner01/rust_video_parser:latest`

**Docker Hub 仓库：** https://hub.docker.com/r/eginner01/rust_video_parser

**镜像大小：** ~145 MB

**Digest：** `sha256:c932005572d1caaed061fb620d9e8835b8fbac5d782e026bc5b8836dfba0dd8f`

---

## 🎯 快速部署（推荐）

### 一键部署

```bash
docker pull eginner01/rust_video_parser:latest
docker run -d --name rust_video_parser -p 8080:8080 eginner01/rust_video_parser:latest
```

访问：http://localhost:8080

### 完整配置部署

```bash
docker run -d \
  --name rust_video_parser \
  -p 8080:8080 \
  -e RUST_LOG=info \
  -e SERVER_PORT=8080 \
  --restart unless-stopped \
  --memory="256m" \
  --cpus="0.5" \
  eginner01/rust_video_parser:latest
```

### 使用 Docker Compose

创建 `docker-compose.yml`：

```yaml
version: '3.8'

services:
  rust_video_parser:
    image: eginner01/rust_video_parser:latest
    container_name: rust_video_parser
    restart: unless-stopped
    ports:
      - "8080:8080"
    environment:
      - RUST_LOG=info
      - SERVER_PORT=8080
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/"]
      interval: 30s
      timeout: 10s
      retries: 3
      start_period: 10s
```

启动：

```bash
docker-compose up -d
```

---

## 🔧 管理命令

### 启动/停止/重启

```bash
# 启动
docker start rust_video_parser

# 停止
docker stop rust_video_parser

# 重启
docker restart rust_video_parser

# 查看状态
docker ps | grep rust_video_parser
```

### 查看日志

```bash
# 实时日志
docker logs -f rust_video_parser

# 最近 100 行
docker logs --tail 100 rust_video_parser

# 带时间戳
docker logs -t rust_video_parser
```

### 更新镜像

```bash
# 1. 拉取最新镜像
docker pull eginner01/rust_video_parser:latest

# 2. 停止并删除旧容器
docker stop rust_video_parser
docker rm rust_video_parser

# 3. 启动新容器
docker run -d --name rust_video_parser -p 8080:8080 eginner01/rust_video_parser:latest
```

或使用脚本：

```bash
#!/bin/bash
docker pull eginner01/rust_video_parser:latest && \
docker stop rust_video_parser && \
docker rm rust_video_parser && \
docker run -d --name rust_video_parser -p 8080:8080 --restart unless-stopped eginner01/rust_video_parser:latest
```

---

## 🌐 不同环境部署

### Windows 部署

```powershell
# PowerShell
docker pull eginner01/rust_video_parser:latest
docker run -d --name rust_video_parser -p 8080:8080 --restart unless-stopped eginner01/rust_video_parser:latest

# 打开浏览器
Start-Process "http://localhost:8080"
```

### Linux 部署

```bash
# Ubuntu/Debian
docker pull eginner01/rust_video_parser:latest
docker run -d --name rust_video_parser -p 8080:8080 --restart unless-stopped eginner01/rust_video_parser:latest

# CentOS/RHEL
sudo docker pull eginner01/rust_video_parser:latest
sudo docker run -d --name rust_video_parser -p 8080:8080 --restart unless-stopped eginner01/rust_video_parser:latest
```

### macOS 部署

```bash
docker pull eginner01/rust_video_parser:latest
docker run -d --name rust_video_parser -p 8080:8080 --restart unless-stopped eginner01/rust_video_parser:latest
open http://localhost:8080
```

### 云服务器部署

```bash
# 阿里云/腾讯云/AWS
docker pull eginner01/rust_video_parser:latest
docker run -d \
  --name rust_video_parser \
  -p 8080:8080 \
  --restart unless-stopped \
  eginner01/rust_video_parser:latest

# 配置防火墙（如需要）
sudo ufw allow 8080/tcp
```

---

## 🔒 生产环境部署

### 使用 Nginx 反向代理

`/etc/nginx/sites-available/rust_video_parser`：

```nginx
server {
    listen 80;
    server_name video.example.com;

    location / {
        proxy_pass http://localhost:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

启用配置：

```bash
sudo ln -s /etc/nginx/sites-available/rust_video_parser /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl reload nginx
```

### 使用 HTTPS（Let's Encrypt）

```bash
# 安装 certbot
sudo apt install certbot python3-certbot-nginx

# 获取证书
sudo certbot --nginx -d video.example.com

# 自动续期
sudo certbot renew --dry-run
```

### 使用 Systemd 管理 Docker 容器

创建 `/etc/systemd/system/rust-video-parser.service`：

```ini
[Unit]
Description=Rust Video Parser Container
After=docker.service
Requires=docker.service

[Service]
Type=oneshot
RemainAfterExit=yes
ExecStartPre=-/usr/bin/docker stop rust_video_parser
ExecStartPre=-/usr/bin/docker rm rust_video_parser
ExecStartPre=/usr/bin/docker pull eginner01/rust_video_parser:latest
ExecStart=/usr/bin/docker run -d --name rust_video_parser -p 8080:8080 --restart unless-stopped eginner01/rust_video_parser:latest
ExecStop=/usr/bin/docker stop rust_video_parser

[Install]
WantedBy=multi-user.target
```

启用服务：

```bash
sudo systemctl enable rust-video-parser
sudo systemctl start rust-video-parser
sudo systemctl status rust-video-parser
```

---

## 📊 性能优化

### 资源限制

```bash
docker run -d \
  --name rust_video_parser \
  -p 8080:8080 \
  --memory="512m" \
  --memory-swap="1g" \
  --cpus="1.0" \
  --restart unless-stopped \
  eginner01/rust_video_parser:latest
```

### 日志管理

```bash
docker run -d \
  --name rust_video_parser \
  -p 8080:8080 \
  --log-driver json-file \
  --log-opt max-size=10m \
  --log-opt max-file=3 \
  --restart unless-stopped \
  eginner01/rust_video_parser:latest
```

---

## 🔍 故障排查

### 检查容器状态

```bash
# 查看运行状态
docker ps -a | grep rust_video_parser

# 查看详细信息
docker inspect rust_video_parser

# 查看资源使用
docker stats rust_video_parser
```

### 测试连接

```bash
# 测试 HTTP 接口
curl http://localhost:8080/

# 测试解析 API
curl "http://localhost:8080/video/share/url/parse?url=test"

# 查看支持的平台
curl http://localhost:8080/platforms
```

### 常见问题

#### 问题 1: 端口被占用

```bash
# 查看端口占用
netstat -tuln | grep 8080

# 更换端口
docker run -d --name rust_video_parser -p 3000:8080 eginner01/rust_video_parser:latest
```

#### 问题 2: 容器无法启动

```bash
# 查看日志
docker logs rust_video_parser

# 删除并重新创建
docker rm -f rust_video_parser
docker run -d --name rust_video_parser -p 8080:8080 eginner01/rust_video_parser:latest
```

#### 问题 3: 内存不足

```bash
# 增加内存限制
docker update --memory="512m" rust_video_parser

# 或重新创建容器
docker stop rust_video_parser
docker rm rust_video_parser
docker run -d --name rust_video_parser -p 8080:8080 --memory="512m" eginner01/rust_video_parser:latest
```

---

## 📝 环境变量

| 变量 | 说明 | 默认值 |
|------|------|--------|
| `RUST_LOG` | 日志级别 (trace, debug, info, warn, error) | `info` |
| `SERVER_PORT` | 服务端口 | `8080` |

使用示例：

```bash
docker run -d \
  --name rust_video_parser \
  -p 8080:8080 \
  -e RUST_LOG=debug \
  -e SERVER_PORT=8080 \
  eginner01/rust_video_parser:latest
```

---

## 🎯 验证部署

部署完成后，验证服务是否正常：

```bash
# 1. 检查容器运行状态
docker ps | grep rust_video_parser

# 2. 检查服务健康
curl http://localhost:8080/

# 3. 测试解析功能
curl "http://localhost:8080/platforms"

# 4. 打开 Web 界面
# 浏览器访问: http://localhost:8080
```

---

## 📚 相关链接

- **Docker Hub:** https://hub.docker.com/r/eginner01/rust_video_parser
- **GitHub:** https://github.com/eginner01/rust_video
- **文档:** https://github.com/eginner01/rust_video#readme

---

## 🙏 支持

如有问题，请提交 Issue：https://github.com/eginner01/rust_video/issues

**Made with ❤️ and 🦀 Rust**

