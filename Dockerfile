# 构建阶段
FROM rust:latest AS builder

WORKDIR /app

# 复制依赖文件
COPY Cargo.toml Cargo.lock ./

# 复制源代码
COPY src ./src
COPY templates ./templates

# 构建项目
RUN cargo build --release

# 运行阶段
FROM debian:bookworm-slim

# 安装运行时依赖
RUN apt-get update && \
    apt-get install -y ca-certificates libssl3 && \
    rm -rf /var/lib/apt/lists/*

# 从构建阶段复制二进制文件
COPY --from=builder /app/target/release/rust_video_parser /usr/local/bin/
COPY --from=builder /app/templates /app/templates

WORKDIR /app

# 暴露端口
EXPOSE 8080

# 设置环境变量
ENV RUST_LOG=info

# 运行应用
CMD ["rust_video_parser"]

