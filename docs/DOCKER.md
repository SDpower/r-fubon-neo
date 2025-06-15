# Docker 使用指南

r-fubon-neo 完整的 Docker 化部署指南

> **⚠️ P.O.C 專案提醒**: 本專案為概念驗證階段，Docker 配置僅供學習和測試使用。  
> **開發者**: Steve Lo (info@sd.idv.tw)

## 目錄

- [快速開始](#快速開始)
- [構建映像](#構建映像)
- [運行容器](#運行容器)
- [Docker Compose](#docker-compose)
- [開發環境](#開發環境)
- [生產部署](#生產部署)
- [監控和日誌](#監控和日誌)
- [故障排除](#故障排除)

## 快速開始

### 最簡單的使用方式

```bash
# 構建映像
docker build -t r-fubon-neo .

# 查看版本
docker run --rm r-fubon-neo version

# 使用 API 金鑰測試
docker run --rm \
  -e FUBON_API_KEY=your_api_key \
  -e FUBON_SECRET_KEY=your_secret_key \
  r-fubon-neo test
```

## 構建映像

### 使用 Dockerfile

#### 生產映像（默認）
```bash
# 基本構建
docker build -t r-fubon-neo .

# 指定標籤
docker build -t r-fubon-neo:v2.2.3 .

# 多平台構建
docker buildx build --platform linux/amd64,linux/arm64 -t r-fubon-neo .
```

#### 開發映像
```bash
# 構建開發映像
docker build -f Dockerfile.dev --target development -t r-fubon-neo:dev .

# 構建生產映像（從開發 Dockerfile）
docker build -f Dockerfile.dev --target production -t r-fubon-neo:prod .
```

#### 靜態連結映像（已驗證可用）
```bash
# 構建靜態連結映像（極小體積 - 僅 2.16MB）
docker build -f Dockerfile.static --target static -t r-fubon-neo:static .

# 構建 Distroless 靜態映像（3.94MB，更好相容性）
docker build -f Dockerfile.static --target distroless -t r-fubon-neo:distroless .

# 驗證映像大小
docker images r-fubon-neo --format "table {{.Repository}}:{{.Tag}}\t{{.Size}}"
# 輸出：
# REPOSITORY:TAG           SIZE
# r-fubon-neo:distroless   3.94MB
# r-fubon-neo:static       2.16MB
```

### 使用構建腳本

我們提供了便利的構建腳本：

```bash
# 基本構建
./scripts/docker-build.sh

# 指定標籤
./scripts/docker-build.sh -t v2.2.3

# 構建開發映像
./scripts/docker-build.sh -d -t dev

# 構建靜態連結映像（2.16MB）
./scripts/docker-build.sh -s -t static

# 構建 Distroless 映像（3.94MB）
./scripts/docker-build.sh --distroless -t distroless

# 構建並推送到註冊表
./scripts/docker-build.sh -t v2.2.3 -p -r your-registry.com

# 查看所有選項
./scripts/docker-build.sh -h
```

### 構建選項

| 選項 | 描述 | 範例 |
|------|------|------|
| `-t, --tag` | 指定映像標籤 | `-t v2.2.3` |
| `-d, --dev` | 構建開發映像 | `-d` |
| `-s, --static` | 構建靜態連結映像 | `-s` |
| `--distroless` | 構建 Distroless 靜態映像 | `--distroless` |
| `-p, --push` | 構建後推送到註冊表 | `-p` |
| `-r, --registry` | 指定註冊表 URL | `-r registry.example.com` |

## 運行容器

### 基本運行

```bash
# 查看版本
docker run --rm r-fubon-neo version

# 測試連接（需要 API 金鑰）
docker run --rm \
  -e FUBON_API_KEY=your_key \
  -e FUBON_SECRET_KEY=your_secret \
  r-fubon-neo test

# 使用靜態連結映像（極小體積 - 2.16MB）
docker run --rm \
  -e FUBON_API_KEY=your_key \
  -e FUBON_SECRET_KEY=your_secret \
  r-fubon-neo:static test

# 使用 Distroless 映像（小體積 - 3.94MB，更好相容性）
docker run --rm \
  -e FUBON_API_KEY=your_key \
  -e FUBON_SECRET_KEY=your_secret \
  r-fubon-neo:distroless test

# 初始化市場數據
docker run --rm \
  -e FUBON_API_KEY=your_key \
  -e FUBON_SECRET_KEY=your_secret \
  r-fubon-neo market-data
```

### 進階運行選項

```bash
# 掛載配置和日誌目錄
docker run --rm \
  -v $(pwd)/config:/app/config:ro \
  -v $(pwd)/logs:/app/logs \
  -e RUST_LOG=debug \
  r-fubon-neo version

# 交互模式
docker run --rm -it \
  --name fubon-interactive \
  r-fubon-neo version

# 後台運行
docker run -d \
  --name fubon-service \
  --restart unless-stopped \
  -e FUBON_API_KEY=your_key \
  -e FUBON_SECRET_KEY=your_secret \
  r-fubon-neo market-data
```

### 使用運行腳本

```bash
# 基本使用
./scripts/docker-run.sh version

# 測試連接
./scripts/docker-run.sh -k YOUR_API_KEY -s YOUR_SECRET test

# 交互模式
./scripts/docker-run.sh -t -n fubon-dev version

# 後台運行
./scripts/docker-run.sh -d -n fubon-service market-data

# 查看所有選項
./scripts/docker-run.sh -h
```

### 運行腳本選項

| 選項 | 描述 | 範例 |
|------|------|------|
| `-i, --image` | 指定映像 | `-i r-fubon-neo:dev` |
| `-k, --api-key` | API 金鑰 | `-k your_api_key` |
| `-s, --secret-key` | 秘密金鑰 | `-s your_secret` |
| `-t, --interactive` | 交互模式 | `-t` |
| `-d, --detach` | 後台運行 | `-d` |
| `-n, --name` | 容器名稱 | `-n fubon-service` |

## Docker Compose

### 基本使用

```bash
# 啟動主服務
docker-compose up fubon-neo

# 後台啟動
docker-compose up -d fubon-neo

# 查看日誌
docker-compose logs -f fubon-neo

# 停止服務
docker-compose down
```

### 開發模式

```bash
# 啟動開發環境
docker-compose --profile dev up fubon-neo-dev

# 啟動靜態連結版本
docker-compose --profile static up fubon-neo-static

# 啟動 Distroless 版本
docker-compose --profile distroless up fubon-neo-distroless

# 使用覆蓋文件（自動熱重載）
docker-compose up fubon-neo  # 自動使用 docker-compose.override.yml
```

### 完整環境（包含監控）

```bash
# 啟動所有服務
docker-compose --profile monitoring --profile cache up

# 只啟動監控服務
docker-compose --profile monitoring up prometheus

# 只啟動快取服務
docker-compose --profile cache up redis
```

### 服務說明

| 服務名稱 | 描述 | Profile | 映像大小 | 端口 |
|----------|------|---------|----------|------|
| `fubon-neo` | 主要服務 | - | ~50MB+ | - |
| `fubon-neo-dev` | 開發服務 | `dev` | ~1.3GB | 3000, 8080 |
| `fubon-neo-static` | 靜態連結服務 | `static` | **2.16MB** | - |
| `fubon-neo-distroless` | Distroless 服務 | `distroless` | **3.94MB** | - |
| `redis` | 快取服務 | `cache` | ~32MB | 6379 |
| `prometheus` | 監控服務 | `monitoring` | ~200MB | 9090 |

## 開發環境

### 開發容器特點

- 支援熱重載（`cargo-watch`）
- 預裝開發工具
- 掛載源碼目錄
- 暴露調試端口

### 啟動開發環境

```bash
# 方法1: 使用 Docker Compose
docker-compose --profile dev up fubon-neo-dev

# 方法2: 直接運行開發映像
docker run --rm -it \
  -v $(pwd):/app:cached \
  -v cargo-cache:/usr/local/cargo/registry \
  -v target-cache:/app/target \
  -p 3000:3000 -p 8080:8080 \
  r-fubon-neo:dev

# 方法3: 使用覆蓋配置
docker-compose up fubon-neo  # 自動使用熱重載
```

### 開發工作流程

1. **修改程式碼** - 在主機上編輯
2. **自動重新編譯** - `cargo-watch` 監控檔案變化
3. **即時測試** - 容器內自動重啟應用
4. **調試** - 使用暴露的端口連接調試器

### 開發環境變數

```bash
# 日誌等級
RUST_LOG=debug
RUST_BACKTRACE=full

# 開發模式標誌
CARGO_WATCH_IGNORE_GLOB=target/*
```

## 生產部署

### 生產映像特點

- 多階段構建，映像小
- 非 root 用戶運行
- 包含健康檢查
- 優化的依賴管理
- 支援靜態連結（無外部依賴）

### 靜態連結映像優勢

**r-fubon-neo:static (2.16MB)**
- ✅ 極小體積，快速下載和啟動
- ✅ 基於 scratch，最高安全性（無攻擊面）
- ✅ 完全靜態連結，無外部依賴
- ✅ 適合微服務和邊緣計算
- ⚠️ 調試工具有限

**r-fubon-neo:distroless (3.94MB)**  
- ✅ 小體積，兼顧安全性和相容性
- ✅ 包含基本的 CA 證書和時區資料
- ✅ 非 root 用戶執行
- ✅ 更好的故障排除支援
- ✅ 推薦用於生產環境

### 部署配置

```yaml
# docker-compose.prod.yml
version: '3.8'

services:
  fubon-neo:
    image: r-fubon-neo:latest
    container_name: fubon-neo-prod
    environment:
      - RUST_LOG=info
      - FUBON_API_KEY=${FUBON_API_KEY}
      - FUBON_SECRET_KEY=${FUBON_SECRET_KEY}
    volumes:
      - ./config:/app/config:ro
      - ./logs:/app/logs
    restart: unless-stopped
    deploy:
      resources:
        limits:
          memory: 256M
          cpus: '0.5'
        reservations:
          memory: 128M
          cpus: '0.25'
    healthcheck:
      test: ["CMD", "r-fubon-neo", "version"]
      interval: 30s
      timeout: 10s
      retries: 3
      start_period: 40s
```

### 部署腳本

```bash
#!/bin/bash
# deploy.sh

set -e

# 拉取最新映像
docker pull r-fubon-neo:latest

# 停止舊容器
docker-compose -f docker-compose.prod.yml down

# 啟動新容器
docker-compose -f docker-compose.prod.yml up -d

# 檢查健康狀態
sleep 30
docker-compose -f docker-compose.prod.yml ps
```

## 監控和日誌

### Prometheus 監控

```bash
# 啟動監控
docker-compose --profile monitoring up prometheus

# 訪問 Prometheus UI
open http://localhost:9090
```

### 日誌管理

```bash
# 查看容器日誌
docker logs fubon-neo-prod

# 實時日誌
docker logs -f fubon-neo-prod

# 使用 docker-compose 查看日誌
docker-compose logs -f fubon-neo

# 限制日誌輸出行數
docker logs --tail 100 fubon-neo-prod
```

### 日誌配置

```yaml
# docker-compose.yml 中的日誌配置
services:
  fubon-neo:
    logging:
      driver: "json-file"
      options:
        max-size: "10m"
        max-file: "3"
```

### 健康檢查

```bash
# 手動檢查健康狀態
docker exec fubon-neo-prod r-fubon-neo version

# 查看健康檢查歷史
docker inspect fubon-neo-prod | jq '.[0].State.Health'
```

## 故障排除

### 常見問題

#### 1. 容器啟動失敗

```bash
# 檢查容器狀態
docker ps -a

# 查看錯誤日誌
docker logs container_name

# 檢查映像
docker images r-fubon-neo
```

#### 2. 權限問題

```bash
# 檢查檔案權限
ls -la config/ logs/

# 修正權限
sudo chown -R $(id -u):$(id -g) config/ logs/
```

#### 3. 網路問題

```bash
# 檢查容器網路
docker network ls

# 檢查容器 IP
docker inspect container_name | grep IPAddress

# 測試網路連通性
docker exec container_name ping google.com
```

#### 4. 記憶體問題

```bash
# 檢查容器資源使用
docker stats container_name

# 檢查系統資源
free -h
df -h
```

### 調試技巧

#### 進入容器

```bash
# 進入運行中的容器
docker exec -it container_name /bin/bash

# 以 root 身份進入
docker exec -it --user root container_name /bin/bash

# 運行臨時調試容器
docker run --rm -it r-fubon-neo /bin/bash
```

#### 檢查環境

```bash
# 檢查環境變數
docker exec container_name env

# 檢查掛載點
docker exec container_name mount

# 檢查網路配置
docker exec container_name ip addr show
```

### 效能調優

#### 映像優化

```dockerfile
# 標準多階段構建
FROM rust:1.75-slim as builder
# ... 構建階段

FROM debian:bookworm-slim as runtime
# ... 運行時階段

# 靜態連結構建（最小體積）
FROM rust:1.75-alpine as static-builder
# ... 靜態構建階段

FROM scratch as static-runtime
# ... 僅包含靜態二進位檔
```

#### 資源限制

```yaml
services:
  fubon-neo:
    deploy:
      resources:
        limits:
          memory: 256M
          cpus: '0.5'
```

#### 快取策略

```bash
# 使用 BuildKit 快取
DOCKER_BUILDKIT=1 docker build --cache-from r-fubon-neo:cache .

# 快取 Cargo 依賴
docker run -v cargo-cache:/usr/local/cargo/registry r-fubon-neo
```

## 安全考量

### 映像安全

- 使用非 root 用戶運行
- 定期更新基礎映像
- 掃描漏洞

```bash
# 使用 trivy 掃描漏洞
trivy image r-fubon-neo:latest
```

### 秘密管理

```bash
# 使用 Docker secrets（Swarm 模式）
echo "your_api_key" | docker secret create fubon_api_key -

# 使用環境檔案
docker run --env-file .env r-fubon-neo
```

### 網路安全

```yaml
# 限制網路訪問
services:
  fubon-neo:
    networks:
      - internal
    
networks:
  internal:
    internal: true
```

## 最佳實踐

1. **使用 .dockerignore** - 排除不必要的檔案
2. **多階段構建** - 減少映像大小
3. **靜態連結** - 最小化依賴和映像體積（推薦 distroless 用於生產）
4. **映像選擇**：
   - 開發測試：標準映像
   - 生產部署：`r-fubon-neo:distroless` (3.94MB)
   - 微服務/邊緣：`r-fubon-neo:static` (2.16MB)
5. **健康檢查** - 確保服務可用性
6. **資源限制** - 防止資源耗盡
7. **日誌管理** - 配置適當的日誌策略
8. **秘密管理** - 安全處理敏感資訊
9. **定期更新** - 保持映像和依賴最新

### 映像大小對比總結

| 用途 | 推薦映像 | 大小 | 安全性 | 相容性 |
|------|----------|------|--------|--------|
| 生產部署 | `distroless` | 3.94MB | 很高 | 良好 |
| 微服務 | `static` | 2.16MB | 極高 | 基本 |
| 開發測試 | `latest` | ~50MB+ | 中等 | 最佳 |

## 參考資源

- [Docker 官方文檔](https://docs.docker.com/)
- [Docker Compose 文檔](https://docs.docker.com/compose/)
- [Rust Docker 最佳實踐](https://docs.docker.com/language/rust/)
- [專案 GitHub](https://github.com/SDpower/r-fubon-neo)