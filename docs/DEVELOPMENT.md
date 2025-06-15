# 開發者指南

r-fubon-neo 開發環境設置和貢獻指南

> **⚠️ P.O.C 專案提醒**: 本專案為概念驗證階段，開發指南僅供學習和研究參考。  
> **專案開發者**: Steve Lo (info@sd.idv.tw)  
> **專案性質**: Proof of Concept - 探索 Rust 在金融科技領域的應用

## 目錄

- [開發環境設置](#開發環境設置)
- [專案結構](#專案結構)
- [開發工作流程](#開發工作流程)
- [測試](#測試)
- [程式碼品質](#程式碼品質)
- [貢獻指南](#貢獻指南)
- [發布流程](#發布流程)
- [故障排除](#故障排除)

## 開發環境設置

### 系統需求

- **Rust**: 1.75+ (建議使用 rustup 安裝)
- **系統依賴**:
  - macOS: `brew install openssl pkg-config`
  - Ubuntu/Debian: `sudo apt install libssl-dev pkg-config build-essential`
  - CentOS/RHEL: `sudo yum install openssl-devel pkgconfig gcc`
- **靜態連結依賴** (可選):
  - macOS: `brew install FiloSottile/musl-cross/musl-cross`
  - Ubuntu/Debian: `sudo apt install musl-tools musl-dev`

### 安裝 Rust

```bash
# 安裝 rustup（如果尚未安裝）
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 重新載入環境
source ~/.cargo/env

# 驗證安裝
rustc --version
cargo --version

# 安裝靜態連結目標 (可選)
rustup target add x86_64-unknown-linux-musl
rustup target add aarch64-unknown-linux-musl
rustup target add x86_64-unknown-linux-gnu
rustup target add aarch64-unknown-linux-gnu
```

### 克隆專案

```bash
git clone https://github.com/SDpower/r-fubon-neo.git
cd r-fubon-neo
```

### 安裝開發工具

```bash
# 安裝有用的 Cargo 工具
cargo install cargo-watch    # 檔案監控和自動重建
cargo install cargo-expand   # 展開宏
cargo install cargo-audit    # 安全審計
cargo install cargo-outdated # 檢查過期依賴
cargo install cargo-tree     # 依賴樹視圖

# 安裝 pre-commit hooks（可選）
pip install pre-commit
pre-commit install
```

### IDE 設置

#### Visual Studio Code

推薦插件：
- **rust-analyzer**: Rust 語言支援
- **CodeLLDB**: 調試支援
- **Even Better TOML**: TOML 檔案支援
- **GitLens**: Git 增強功能

```json
// .vscode/settings.json
{
    "rust-analyzer.checkOnSave.command": "clippy",
    "rust-analyzer.cargo.features": "all",
    "editor.formatOnSave": true
}
```

```json
// .vscode/launch.json
{
    "version": "0.2.0",
    "configurations": [
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug r-fubon-neo",
            "cargo": {
                "args": ["build", "--bin=r-fubon-neo"],
                "filter": {
                    "name": "r-fubon-neo",
                    "kind": "bin"
                }
            },
            "args": ["version"],
            "cwd": "${workspaceFolder}"
        }
    ]
}
```

## 專案結構

```
r-fubon-neo/
├── src/                    # 主要源碼
│   ├── lib.rs             # 庫入口
│   ├── main.rs            # CLI 入口
│   ├── constants.rs       # 常數定義
│   ├── error.rs           # 錯誤類型
│   ├── types.rs           # 數據類型
│   ├── sdk.rs             # 核心 SDK
│   └── market_data/       # 市場數據模組
│       ├── mod.rs
│       ├── rest.rs        # REST API 客戶端
│       └── websocket.rs   # WebSocket 客戶端
├── examples/              # 使用範例
├── tests/                 # 測試檔案
├── docs/                  # 文檔
├── scripts/               # 實用腳本
├── config/                # 配置檔案
├── .cargo/                # Cargo 配置
│   └── config.toml        # 靜態連結配置
├── Dockerfile             # 生產 Docker 檔案
├── Dockerfile.dev         # 開發 Docker 檔案
├── Dockerfile.static      # 靜態連結 Docker 檔案
├── docker-compose.yml     # Docker Compose 配置
└── Cargo.toml            # 專案配置
```

### 模組說明

#### `src/lib.rs`
- 庫的主要入口點
- 公開 API 介面
- 重新導出主要類型

#### `src/sdk.rs`
- `CoreSDK` trait 定義
- `FubonSDK` 主要實現
- 交易相關功能

#### `src/market_data/`
- `rest.rs`: HTTP REST API 客戶端
- `websocket.rs`: WebSocket 即時數據客戶端
- `mod.rs`: 模組公共介面

#### `src/types.rs`
- 所有交易和市場數據相關的類型定義
- 枚舉和結構體
- Serde 序列化支援

#### `src/error.rs`
- 統一錯誤處理
- 自定義錯誤類型
- 錯誤轉換實現

## 開發工作流程

### 日常開發

```bash
# 1. 創建功能分支
git checkout -b feature/new-feature

# 2. 開發過程中持續運行
cargo watch -x "check" -x "test" -x "run -- version"

# 3. 提交前檢查
cargo fmt
cargo clippy
cargo test

# 測試靜態連結編譯
cargo build --release --target x86_64-unknown-linux-musl
cargo build --profile static

# 4. 提交更改
git add .
git commit -m "feat: add new feature"

# 5. 推送到遠端
git push origin feature/new-feature
```

### 熱重載開發

```bash
# 方法1: 使用 cargo-watch
cargo watch -x "run -- version"

# 方法2: 使用 Docker 開發環境
docker-compose --profile dev up fubon-neo-dev

# 方法3: 使用靜態連結開發
docker-compose --profile static up fubon-neo-static

# 方法4: 直接使用開發容器
docker run --rm -it \
  -v $(pwd):/app:cached \
  r-fubon-neo:dev
```

### 添加新功能

1. **設計階段**
   - 在 `docs/` 中創建設計文檔
   - 討論 API 設計
   - 確定類型定義

2. **實現階段**
   - 在適當的模組中添加代碼
   - 更新公共 API（`lib.rs`）
   - 添加錯誤處理

3. **測試階段**
   - 編寫單元測試
   - 添加集成測試
   - 更新範例程式碼

4. **文檔階段**
   - 更新 API 文檔
   - 添加使用範例
   - 更新 README

## 測試

### 運行測試

```bash
# 運行所有測試
cargo test

# 運行特定測試
cargo test test_name

# 運行特定模組的測試
cargo test market_data

# 顯示詳細輸出
cargo test -- --nocapture

# 運行文檔測試
cargo test --doc
```

### 測試結構

```
tests/
├── integration/           # 集成測試
│   ├── sdk_tests.rs
│   └── market_data_tests.rs
├── unit/                  # 單元測試
└── common/                # 測試實用工具
    └── mod.rs
```

### 編寫測試

#### 單元測試

```rust
// src/sdk.rs
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sdk_creation() {
        let sdk = FubonSDK::new();
        assert!(sdk.api_key.is_none());
    }
    
    #[tokio::test]
    async fn test_async_function() {
        // 異步測試
    }
}
```

#### 集成測試

```rust
// tests/integration/sdk_tests.rs
use r_fubon_neo::{FubonSDK, CoreSDK};

#[tokio::test]
async fn test_sdk_integration() {
    let sdk = FubonSDK::new()
        .with_credentials("test_key".to_string(), "test_secret".to_string());
    
    // 測試邏輯
}
```

### 測試覆蓋率

```bash
# 安裝 tarpaulin
cargo install cargo-tarpaulin

# 運行覆蓋率測試
cargo tarpaulin --out Html

# 查看覆蓋率報告
open tarpaulin-report.html
```

## 程式碼品質

### 格式化

```bash
# 格式化所有代碼
cargo fmt

# 檢查格式是否正確
cargo fmt -- --check
```

### Linting

```bash
# 運行 Clippy
cargo clippy

# 運行嚴格的 Clippy 檢查
cargo clippy -- -D warnings

# 修復可自動修復的問題
cargo clippy --fix
```

### 代碼審查檢查清單

- [ ] 代碼遵循 Rust 慣例
- [ ] 所有函數都有適當的文檔
- [ ] 錯誤處理正確
- [ ] 測試覆蓋新功能
- [ ] 沒有 `unwrap()` 或 `expect()` 在生產代碼中
- [ ] 異步代碼正確使用
- [ ] 安全考量已檢查

### 安全審計

```bash
# 審計依賴
cargo audit

# 檢查過期依賴
cargo outdated

# 查看依賴樹
cargo tree
```

## 貢獻指南

### 貢獻流程

1. **Fork 專案**
   ```bash
   git clone https://github.com/YOUR_USERNAME/r-fubon-neo.git
   ```

2. **創建功能分支**
   ```bash
   git checkout -b feature/amazing-feature
   ```

3. **實現功能**
   - 遵循現有代碼風格
   - 添加適當的測試
   - 更新文檔

4. **提交更改**
   ```bash
   git commit -m "feat: add amazing feature"
   ```

5. **推送分支**
   ```bash
   git push origin feature/amazing-feature
   ```

6. **創建 Pull Request**
   - 填寫 PR 模板
   - 確保 CI 通過
   - 回應代碼審查

### 提交消息格式

使用 [Conventional Commits](https://www.conventionalcommits.org/) 格式：

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

類型：
- `feat`: 新功能
- `fix`: 錯誤修復
- `docs`: 文檔更新
- `style`: 代碼格式
- `refactor`: 重構
- `test`: 測試相關
- `chore`: 構建或輔助工具

範例：
```
feat(market_data): add WebSocket reconnection logic

Implement automatic reconnection with exponential backoff
when WebSocket connection is lost.

Closes #123
```

### Pull Request 檢查清單

- [ ] 代碼通過所有測試
- [ ] 代碼通過 Clippy 檢查
- [ ] 代碼已格式化（`cargo fmt`）
- [ ] 添加了適當的測試
- [ ] 更新了相關文檔
- [ ] PR 描述清晰
- [ ] 遵循提交消息格式

## 發布流程

### 版本號規則

遵循 [Semantic Versioning](https://semver.org/)：

- `MAJOR.MINOR.PATCH`
- `MAJOR`: 不相容的 API 變更
- `MINOR`: 向後相容的功能新增
- `PATCH`: 向後相容的錯誤修復

### 發布步驟

1. **準備發布**
   ```bash
   # 確保主分支是最新的
   git checkout main
   git pull origin main
   
   # 運行完整測試
   cargo test
   cargo clippy
   cargo audit
   ```

2. **更新版本號**
   ```toml
   # Cargo.toml
   [package]
   version = "2.2.4"
   ```

3. **更新 CHANGELOG**
   ```markdown
   ## [2.2.4] - 2024-01-15
   
   ### Added
   - New feature X
   
   ### Fixed
   - Bug fix Y
   ```

4. **創建發布提交**
   ```bash
   git add .
   git commit -m "chore: bump version to 2.2.4"
   git tag v2.2.4
   git push origin main --tags
   ```

5. **發布到 Crates.io**（如果公開）
   ```bash
   cargo publish
   ```

6. **創建 GitHub Release**
   - 在 GitHub 上創建新的 Release
   - 使用 tag v2.2.4
   - 複製 CHANGELOG 內容

## 故障排除

### 編譯問題

#### OpenSSL 錯誤

```bash
# macOS
brew install openssl
export OPENSSL_DIR=/usr/local/opt/openssl

# Ubuntu/Debian
sudo apt install libssl-dev

# 使用 rustls 替代（可選）
cargo build --no-default-features --features rustls

# 靜態連結 OpenSSL
export OPENSSL_STATIC=1
cargo build --target x86_64-unknown-linux-musl
```

#### 靜態連結問題

```bash
# 安裝 musl 工具鏈
rustup target add x86_64-unknown-linux-musl

# macOS 上安裝 musl 交叉編譯工具
brew install FiloSottile/musl-cross/musl-cross

# Ubuntu/Debian 安裝 musl 工具
sudo apt install musl-tools musl-dev

# 設置靜態連結環境變數
export RUSTFLAGS="-C target-feature=+crt-static"
export PKG_CONFIG_ALL_STATIC=1
```

#### Docker 靜態連結構建問題

**問題：Cargo.lock 版本不兼容**
```bash
error: lock file version `4` was found, but this version of Cargo does not understand this lock file
```
解決方案：
- 升級 Docker 映像中的 Rust 版本至 1.82+
- 或在 Dockerfile 中重新生成 Cargo.lock

**問題：tokio-tungstenite 缺少 connect feature**
```bash
error: unresolved import `tokio_tungstenite::connect_async`
```
解決方案：
```toml
# Cargo.toml
tokio-tungstenite = { version = "0.20", features = ["connect", "rustls-tls-webpki-roots"], default-features = false }
```

**問題：交叉編譯工具鏈缺失**
```bash
error: failed to run custom build command for `ring v0.17.14`
```
解決方案：
- 使用 `--platform=linux/x86_64` 強制使用 x86_64 構建環境
- 或安裝適當的交叉編譯工具鏈

**已驗證的解決方案：**
```dockerfile
# 使用 x86_64 平台避免交叉編譯
FROM --platform=linux/x86_64 rust:1.82-alpine as builder

# 安裝必要的依賴
RUN apk add --no-cache \
    musl-dev \
    pkgconfig \
    openssl-dev \
    openssl-libs-static \
    ca-certificates \
    gcc \
    libc-dev

# 設置環境變數
ENV PKG_CONFIG_ALL_STATIC=1
ENV OPENSSL_STATIC=1
```

#### 依賴衝突

```bash
# 清理並重新構建
cargo clean
cargo build

# 更新依賴
cargo update

# 檢查依賴樹
cargo tree --duplicates
```

### 測試問題

#### 異步測試失敗

```rust
// 確保使用 tokio::test
#[tokio::test]
async fn test_async_function() {
    // 測試代碼
}

// 或使用 tokio 運行時
#[test]
fn test_with_runtime() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        // 異步測試代碼
    });
}
```

#### 網路測試問題

```rust
// 使用模擬或跳過網路測試
#[cfg(not(feature = "integration"))]
#[tokio::test]
async fn test_network_function() {
    // 網路測試
}

// 運行時跳過
cargo test --features integration
```

### Docker 問題

#### 構建失敗

```bash
# 檢查 Docker 版本
docker --version

# 清理 Docker 快取
docker system prune -a

# 使用 BuildKit
DOCKER_BUILDKIT=1 docker build .

# 建立靜態連結映像
docker build -f Dockerfile.static --target static -t r-fubon-neo:static .
```

#### 權限問題

```bash
# 檢查檔案權限
ls -la

# 修正權限
sudo chown -R $(id -u):$(id -g) .
```

## 參考資源

### Rust 學習資源

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Async Book](https://rust-lang.github.io/async-book/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)

### 工具文檔

- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [Clippy Lints](https://rust-lang.github.io/rust-clippy/master/)
- [Rustfmt](https://github.com/rust-lang/rustfmt)

### 專案特定

- [Serde Guide](https://serde.rs/)
- [Reqwest](https://docs.rs/reqwest/)
- [Tokio-tungstenite](https://docs.rs/tokio-tungstenite/)

## 聯繫方式

- **專案開發者**: Steve Lo
- **Email**: info@sd.idv.tw
- **GitHub Issues**: [問題回報](https://github.com/SDpower/r-fubon-neo/issues)
- **討論區**: [GitHub Discussions](https://github.com/SDpower/r-fubon-neo/discussions)
- **專案性質**: P.O.C (Proof of Concept) project

## P.O.C 專案特別說明

本專案作為概念驗證，主要目標包括：

1. **技術探索**: 驗證 Rust 在金融 API 開發的可行性
2. **性能研究**: 比較 Rust vs Python 在金融應用的性能差異
3. **架構設計**: 探索異步、類型安全的金融 SDK 設計模式
4. **學習資源**: 為有興趣學習 Rust 金融應用開發的開發者提供參考

### 貢獻方向

歡迎以下類型的貢獻：
- 📚 文檔改進和翻譯
- 🧪 測試用例添加
- 🔧 代碼品質改進
- 💡 新功能概念驗證
- 🐛 問題回報和修復
- 📊 性能基準測試

### 不適合的貢獻

- ❌ 生產環境優化
- ❌ 商業化功能
- ❌ 複雜的企業級功能
- ❌ 涉及真實交易的功能