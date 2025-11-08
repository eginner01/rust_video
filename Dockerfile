# 多阶段构建 - 前端构建阶段
FROM node:18-alpine AS frontend-builder

WORKDIR /frontend

COPY frontend/package.json frontend/package-lock.json ./

RUN npm ci --prefer-offline --no-audit

COPY frontend/ ./

RUN npx vite build --outDir dist

FROM rust:alpine AS backend-builder

RUN apk add --no-cache musl-dev openssl-dev openssl-libs-static pkgconfig

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

COPY src ./src

COPY --from=frontend-builder /frontend/dist ./templates/dist

RUN cargo build --release

FROM alpine:latest

RUN apk add --no-cache ca-certificates libgcc

RUN addgroup -g 1000 appuser && \
    adduser -D -u 1000 -G appuser appuser

WORKDIR /app

COPY --from=backend-builder /app/target/release/rust_video_parser /usr/local/bin/
COPY --from=frontend-builder /frontend/dist ./templates/dist

RUN chown -R appuser:appuser /app

USER appuser

EXPOSE 8080

ENV RUST_LOG=info \
    SERVER_PORT=8080

HEALTHCHECK --interval=30s --timeout=10s --start-period=40s --retries=3 \
    CMD wget --no-verbose --tries=1 --spider http://localhost:8080/ || exit 1

CMD ["rust_video_parser"]
