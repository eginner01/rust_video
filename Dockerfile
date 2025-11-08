# 多阶段构建 - 前端构建阶段
FROM node:18-alpine AS frontend-builder

WORKDIR /frontend

# 复制前端依赖文件
COPY frontend/package.json frontend/package-lock.json ./

# 安装依赖
RUN npm ci --prefer-offline --no-audit

# 复制前端源代码
COPY frontend/ ./

# 构建前端项目（跳过类型检查，并指定输出到 dist 目录）
RUN npx vite build --outDir dist

# 后端构建阶段
FROM rust:alpine AS backend-builder

# 安装构建依赖（包括 OpenSSL 静态库）
RUN apk add --no-cache musl-dev openssl-dev openssl-libs-static pkgconfig

WORKDIR /app

# 复制依赖文件
COPY Cargo.toml Cargo.lock ./

# 复制源代码
COPY src ./src

# 从前端构建阶段复制构建结果（后端编译时需要）
COPY --from=frontend-builder /frontend/dist ./templates/dist

# 构建后端项目
RUN cargo build --release

# 最终运行阶段
FROM alpine:latest

# 安装运行时依赖
RUN apk add --no-cache ca-certificates libgcc

# 创建非root用户
RUN addgroup -g 1000 appuser && \
    adduser -D -u 1000 -G appuser appuser

WORKDIR /app

# 从构建阶段复制文件
COPY --from=backend-builder /app/target/release/rust_video_parser /usr/local/bin/
COPY --from=frontend-builder /frontend/dist ./templates/dist

# 设置权限
RUN chown -R appuser:appuser /app

# 切换到非root用户
USER appuser

# 暴露端口
EXPOSE 8080

# 设置环境变量
ENV RUST_LOG=info \
    SERVER_PORT=8080

# 健康检查
HEALTHCHECK --interval=30s --timeout=10s --start-period=40s --retries=3 \
    CMD wget --no-verbose --tries=1 --spider http://localhost:8080/ || exit 1

# 运行应用
CMD ["rust_video_parser"]
