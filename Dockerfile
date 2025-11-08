# 多阶段构建 - 前端构建阶段
FROM node:18-alpine AS frontend-builder

WORKDIR /frontend

# 复制前端依赖文件
COPY frontend/package.json frontend/package-lock.json ./

# 安装依赖
RUN npm ci --prefer-offline --no-audit

# 复制前端源代码
COPY frontend/ ./

# 构建前端项目（输出到 ../templates/dist）
RUN npm run build

# 后端构建阶段
FROM rust:1.75-alpine AS backend-builder

# 安装构建依赖
RUN apk add --no-cache musl-dev openssl-dev pkgconfig

WORKDIR /app

# 复制依赖文件并预构建依赖（利用Docker缓存）
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# 复制源代码
COPY src ./src

# 重新构建（只编译变更的代码）
RUN touch src/main.rs && cargo build --release

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
COPY --from=frontend-builder /frontend/../templates/dist ./templates/dist
COPY templates/index.html ./templates/

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
