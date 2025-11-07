#!/bin/bash

###############################################################################
# Rust Video Parser - 一键部署脚本
# 用途：快速部署 Rust Video Parser Docker 容器
###############################################################################

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# 配置
IMAGE_NAME="eginner01/rust_video_parser:latest"
CONTAINER_NAME="rust_video_parser"
PORT="8080"

echo -e "${CYAN}======================================${NC}"
echo -e "${CYAN}  Rust Video Parser - 一键部署${NC}"
echo -e "${CYAN}======================================${NC}"
echo ""

# 检查 Docker 是否安装
echo -e "${YELLOW}[1/5] 检查 Docker...${NC}"
if ! command -v docker &> /dev/null; then
    echo -e "${RED}❌ Docker 未安装！${NC}"
    echo -e "${YELLOW}请访问 https://docs.docker.com/get-docker/ 安装 Docker${NC}"
    exit 1
fi
echo -e "${GREEN}✅ Docker 已安装: $(docker --version)${NC}"
echo ""

# 停止并删除旧容器
echo -e "${YELLOW}[2/5] 清理旧容器...${NC}"
if docker ps -a --format '{{.Names}}' | grep -q "^${CONTAINER_NAME}$"; then
    echo -e "${YELLOW}停止旧容器...${NC}"
    docker stop ${CONTAINER_NAME} > /dev/null 2>&1 || true
    echo -e "${YELLOW}删除旧容器...${NC}"
    docker rm ${CONTAINER_NAME} > /dev/null 2>&1 || true
    echo -e "${GREEN}✅ 旧容器已清理${NC}"
else
    echo -e "${GREEN}✅ 无需清理${NC}"
fi
echo ""

# 拉取最新镜像
echo -e "${YELLOW}[3/5] 拉取最新镜像...${NC}"
echo -e "${CYAN}镜像: ${IMAGE_NAME}${NC}"
docker pull ${IMAGE_NAME}
echo -e "${GREEN}✅ 镜像拉取成功${NC}"
echo ""

# 启动容器
echo -e "${YELLOW}[4/5] 启动容器...${NC}"
docker run -d \
  --name ${CONTAINER_NAME} \
  -p ${PORT}:8080 \
  -e RUST_LOG=info \
  --restart unless-stopped \
  ${IMAGE_NAME}

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ 容器启动成功${NC}"
else
    echo -e "${RED}❌ 容器启动失败${NC}"
    exit 1
fi
echo ""

# 等待服务就绪
echo -e "${YELLOW}[5/5] 等待服务就绪...${NC}"
sleep 3

# 验证服务
if curl -s http://localhost:${PORT}/ > /dev/null; then
    echo -e "${GREEN}✅ 服务运行正常${NC}"
else
    echo -e "${YELLOW}⚠️  服务可能需要更多时间启动${NC}"
fi
echo ""

# 显示结果
echo -e "${GREEN}======================================${NC}"
echo -e "${GREEN}  ✅ 部署完成！${NC}"
echo -e "${GREEN}======================================${NC}"
echo ""
echo -e "${CYAN}📦 容器名称:${NC} ${CONTAINER_NAME}"
echo -e "${CYAN}🌐 访问地址:${NC} http://localhost:${PORT}"
echo -e "${CYAN}📊 查看日志:${NC} docker logs -f ${CONTAINER_NAME}"
echo -e "${CYAN}🔄 重启服务:${NC} docker restart ${CONTAINER_NAME}"
echo -e "${CYAN}🛑 停止服务:${NC} docker stop ${CONTAINER_NAME}"
echo ""
echo -e "${YELLOW}提示: 在浏览器中打开 http://localhost:${PORT} 开始使用！${NC}"
echo ""

# 询问是否打开浏览器
read -p "是否打开浏览器？(y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    if command -v xdg-open &> /dev/null; then
        xdg-open "http://localhost:${PORT}"
    elif command -v open &> /dev/null; then
        open "http://localhost:${PORT}"
    else
        echo -e "${YELLOW}请手动在浏览器中打开: http://localhost:${PORT}${NC}"
    fi
fi

