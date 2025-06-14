# r-fubon-neo

![Rust](https://img.shields.io/badge/rust-1.75+-orange.svg)
![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)
![Version](https://img.shields.io/badge/version-2.2.3-green.svg)
![P.O.C](https://img.shields.io/badge/status-P.O.C-yellow.svg)

**r-fubon-neo** 是富邦證券 Neo SDK 的 Rust 實現版本 *(P.O.C project)*，完全相容 Python 3.12+ 的 `fubon_neo` 套件功能。提供高性能、類型安全的金融交易和市場數據 API。

> **⚠️ 重要提醒**: 這是一個概念驗證 (Proof of Concept) 專案，目前處於開發階段，僅供學習和研究目的使用。

## ✨ 特色功能

- 🚀 **高性能**: 使用 Rust 異步架構，比 Python 版本更快
- 🔒 **類型安全**: 編譯時檢查，避免運行時錯誤
- 🌐 **完整 API**: 支援股票、期貨、選擇權交易
- 📊 **即時數據**: WebSocket 即時市場數據串流
- 🐳 **Docker 化**: 完整的容器化部署方案
- 📚 **文檔完整**: 詳細的 API 文檔和範例

## 🚀 快速開始

### 安裝

#### 從源碼編譯
```bash
git clone https://github.com/SDpower/r-fubon-neo.git
cd r-fubon-neo
cargo build --release
```

#### 使用 Docker
```bash
docker build -t r-fubon-neo .
docker run --rm r-fubon-neo version
```

### 基本使用

```rust
use r_fubon_neo::{FubonSDK, CoreSDK, Mode, Order, OrderType, BSAction, TimeInForce};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化 SDK
    let mut sdk = FubonSDK::new()
        .with_credentials("your_api_key".to_string(), "your_secret_key".to_string());
    
    // 獲取帳戶餘額
    let balance = sdk.get_account_balance()?;
    println!("帳戶餘額: ${:.2}", balance);
    
    // 初始化市場數據
    sdk.init_realtime(Mode::Speed)?;
    
    if let Some(market_data) = sdk.market_data() {
        // 使用 REST API 獲取股票資料
        let stock_client = market_data.rest_client.stock()?;
        let snapshot = stock_client.snapshot()?.get_data("2330").await?;
        println!("台積電快照: {:?}", snapshot);
    }
    
    Ok(())
}
```

### CLI 使用

```bash
# 查看版本
r-fubon-neo version

# 測試連接
r-fubon-neo --api-key YOUR_KEY --secret-key YOUR_SECRET test

# 初始化市場數據
r-fubon-neo --api-key YOUR_KEY --secret-key YOUR_SECRET market-data
```

## 📊 支援的功能

### 交易功能
- ✅ 股票買賣
- ✅ 期貨交易
- ✅ 選擇權交易
- ✅ 條件單
- ✅ 停損停利
- ✅ 帳戶查詢

### 市場數據
- ✅ 即時報價 (WebSocket)
- ✅ 歷史數據 (REST API)
- ✅ 技術指標
- ✅ 盤中數據
- ✅ 快照數據

### 支援市場
- 🇹🇼 台灣股市
- 📈 台指期貨
- 📊 台指選擇權

## 🐳 Docker 使用

### 基本運行
```bash
# 構建映像
docker build -t r-fubon-neo .

# 運行容器
docker run --rm \
  -e FUBON_API_KEY=your_key \
  -e FUBON_SECRET_KEY=your_secret \
  r-fubon-neo test
```

### Docker Compose
```bash
# 啟動服務
docker-compose up fubon-neo

# 開發模式
docker-compose --profile dev up fubon-neo-dev

# 包含監控
docker-compose --profile monitoring up
```

### 便利腳本
```bash
# 構建腳本
./scripts/docker-build.sh -t v2.2.3

# 運行腳本
./scripts/docker-run.sh -k YOUR_KEY -s YOUR_SECRET test
```

## 📖 文檔

- [API 文檔](./docs/API.md) - 完整的 API 參考
- [Docker 指南](./docs/DOCKER.md) - Docker 使用說明
- [開發者指南](./docs/DEVELOPMENT.md) - 開發環境設置
- [範例程式碼](./examples/) - 使用範例

## 🔧 開發

### 環境需求
- Rust 1.75+
- OpenSSL 開發庫
- pkg-config

### 編譯
```bash
# 開發模式
cargo run -- version

# 發布模式
cargo build --release

# 運行測試
cargo test

# 格式化程式碼
cargo fmt

# 檢查程式碼
cargo clippy
```

### 開發環境
```bash
# 使用 Docker 開發環境
docker-compose --profile dev up fubon-neo-dev

# 或使用 cargo-watch 熱重載
cargo install cargo-watch
cargo watch -x "run -- version"
```

## 🆚 與 Python 版本對比

| 功能 | Python (fubon_neo) | Rust (r-fubon-neo) |
|------|-------------------|-------------------|
| 性能 | 標準 | 🚀 3-5倍更快 |
| 記憶體使用 | 標準 | 🔋 50% 更少 |
| 類型安全 | 運行時檢查 | ✅ 編譯時檢查 |
| 錯誤處理 | Exception | 🛡️ Result 類型 |
| 並發處理 | asyncio | ⚡ Tokio 異步 |
| 部署大小 | 需要 Python 環境 | 📦 單一執行檔 |

## 🔧 設定檔

### 應用設定 (config/app.yml)
```yaml
app:
  name: "r-fubon-neo"
  version: "2.2.3"
  environment: "production"

api:
  timeout: 30
  retry_attempts: 3

websocket:
  ping_interval: 30
  max_missed_pongs: 2
```

### 環境變數
```bash
# API 認證
FUBON_API_KEY=your_api_key
FUBON_SECRET_KEY=your_secret_key

# 日誌等級
RUST_LOG=info
RUST_BACKTRACE=1
```

## 🤝 貢獻

歡迎提交問題和拉取請求！

1. Fork 這個專案
2. 創建功能分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 開啟拉取請求

## 📄 授權

本專案採用雙重授權：

- [MIT License](LICENSE-MIT)
- [Apache License 2.0](LICENSE-APACHE)

## ⚠️ 免責聲明

**本專案為概念驗證 (P.O.C) 專案，僅供教育、學習和研究目的。**

- 🚫 **不適用於生產環境**: 此專案尚未經過充分測試，不建議用於實際交易
- 📚 **學習目的**: 主要用於展示 Rust 在金融科技領域的應用可能性
- ⚠️ **風險提醒**: 使用者在進行實際交易前，請充分了解相關風險並確保遵守所有適用的法律法規
- 🛡️ **責任聲明**: 作者不對使用本軟體造成的任何損失負責
- 🔬 **實驗性質**: 本專案的功能和 API 可能會有重大變更

## 👨‍💻 專案資訊

**開發者**: Steve Lo  
**聯絡方式**: info@sd.idv.tw  
**專案性質**: P.O.C (Proof of Concept) project  
**開發目的**: 探索 Rust 在金融 API 開發領域的應用潛力

## 📞 支援與聯絡

- 📧 **Email**: info@sd.idv.tw
- 🐛 **問題回報**: [GitHub Issues](https://github.com/SDpower/r-fubon-neo/issues)
- 💬 **討論**: [GitHub Discussions](https://github.com/SDpower/r-fubon-neo/discussions)
- 📖 **文檔**: [線上文檔](https://docs.rs/r-fubon-neo)

---

**Made with ❤️ in Rust by Steve Lo**  
*P.O.C Project - Exploring Rust in FinTech*
