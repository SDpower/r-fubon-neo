# 範例程式碼

這個目錄包含了 r-fubon-neo SDK 的各種使用範例，幫助您快速上手並了解如何使用不同的功能。

> **⚠️ P.O.C 專案提醒**: 這些範例僅供學習和概念驗證使用，請勿用於實際交易。  
> **開發者**: Steve Lo (info@sd.idv.tw)

## 📁 範例列表

### 1. 基本交易 (`basic_trading.rs`)
展示基本的股票交易功能。

**功能:**
- 初始化 SDK
- 檢查帳戶餘額和持倉
- 下限價買單
- 查詢訂單狀態
- 取消訂單

**運行方式:**
```bash
# 方法1: 使用 .env 檔案 (推薦)
cp .env.example .env
# 編輯 .env 檔案，填入您的認證資訊
cargo run --example basic_trading

# 方法2: 使用環境變數
export FUBON_API_KEY=your_api_key
export FUBON_SECRET_KEY=your_secret_key
cargo run --example basic_trading
```

### 2. WebSocket 市場數據 (`market_data_websocket.rs`)
展示如何使用 WebSocket 接收即時市場數據。

**功能:**
- WebSocket 連接和認證
- 事件處理器實現
- 訂閱/取消訂閱即時報價
- 處理各種市場數據事件
- 優雅關閉連接

**運行方式:**
```bash
# 使用 .env 檔案或環境變數 (同上)
cargo run --example market_data_websocket
```

### 3. REST API 市場數據 (`market_data_rest.rs`)
展示如何使用 REST API 獲取歷史和快照數據。

**功能:**
- 獲取股票快照數據
- 獲取股票即時數據
- 獲取股票歷史數據
- 數據格式化和分析
- 錯誤處理示範

**運行方式:**
```bash
# 使用 .env 檔案或環境變數 (同上)
cargo run --example market_data_rest
```

## 🚀 快速開始

### 環境設置

1. **使用 .env 檔案 (推薦):**
   ```bash
   # 複製範例檔案
   cp .env.example .env
   
   # 編輯 .env 檔案，填入您的認證資訊
   # FUBON_API_KEY=your_actual_api_key
   # FUBON_SECRET_KEY=your_actual_secret_key
   ```

2. **或使用環境變數:**
   ```bash
   export FUBON_API_KEY=your_api_key
   export FUBON_SECRET_KEY=your_secret_key
   ```

### 運行範例

```bash
# 查看所有可用範例
cargo run --example

# 運行特定範例
cargo run --example basic_trading

# 使用除錯模式運行
RUST_LOG=debug cargo run --example market_data_websocket
```

## 📊 範例詳細說明

### basic_trading.rs

這個範例展示了完整的交易流程：

```rust
// 1. 初始化 SDK
let sdk = FubonSDK::new()
    .with_credentials(api_key, secret_key);

// 2. 檢查餘額
let balance = sdk.get_account_balance()?;

// 3. 下單
let order = Order {
    symbol: "2330".to_string(),
    quantity: 1000,
    price: Some(500.0),
    order_type: OrderType::Limit,
    time_in_force: TimeInForce::Day,
    action: BSAction::Buy,
};
let order_id = sdk.place_order(&order)?;

// 4. 查詢狀態
let status = sdk.get_order_status(&order_id)?;
```

### market_data_websocket.rs

這個範例展示了 WebSocket 即時數據處理：

```rust
// 1. 創建事件處理器
struct MarketDataHandler;

impl EventHandler for MarketDataHandler {
    fn handle_event(&self, event: WebSocketEvent) {
        match event {
            WebSocketEvent::Message(msg) => {
                // 處理即時數據
            }
            WebSocketEvent::Authenticated(_) => {
                // 認證成功
            }
            // ... 其他事件
        }
    }
}

// 2. 連接和訂閱
let mut ws_client = WebSocketClient::new(Mode::Speed, sdk_token)?;
ws_client.add_event_handler(Arc::new(MarketDataHandler)).await;
ws_client.connect().await?;

// 3. 訂閱報價
let mut params = HashMap::new();
params.insert("channel".to_string(), json!("quote"));
params.insert("symbol".to_string(), json!("2330"));
ws_client.subscribe(params).await?;
```

### market_data_rest.rs

這個範例展示了 REST API 數據獲取：

```rust
// 1. 獲取股票客戶端
let stock_client = market_data.rest_client.stock()?;

// 2. 獲取快照數據
let snapshot = stock_client.snapshot()?.get_data("2330").await?;

// 3. 獲取歷史數據
let historical = stock_client.historical()?
    .get_data("2330", "2024-01-01", "2024-01-31").await?;

// 4. 獲取即時數據
let intraday = stock_client.intraday()?.get_data("2330").await?;
```

## 🔧 自定義範例

### 創建新範例

1. **在 `examples/` 目錄創建新檔案:**
   ```bash
   touch examples/my_example.rs
   ```

2. **添加基本結構:**
   ```rust
   /*!
   # 我的範例
   
   描述這個範例的功能
   */
   
   use r_fubon_neo::{FubonSDK, CoreSDK, Result};
   
   #[tokio::main]
   async fn main() -> Result<()> {
       // 您的程式碼
       Ok(())
   }
   ```

3. **運行新範例:**
   ```bash
   cargo run --example my_example
   ```

### 範例模板

```rust
/*!
# 範例標題

簡要描述這個範例的功能和用途。

## 功能
- 功能 1
- 功能 2

## 使用方法
```bash
export FUBON_API_KEY=your_key
export FUBON_SECRET_KEY=your_secret
cargo run --example example_name
```
*/

use r_fubon_neo::{FubonSDK, CoreSDK, Error, Result};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日誌
    tracing_subscriber::fmt::init();
    
    // 獲取認證資訊
    let api_key = env::var("FUBON_API_KEY")
        .map_err(|_| Error::general("請設置 FUBON_API_KEY 環境變數"))?;
    let secret_key = env::var("FUBON_SECRET_KEY")
        .map_err(|_| Error::general("請設置 FUBON_SECRET_KEY 環境變數"))?;
    
    // 初始化 SDK
    let sdk = FubonSDK::new()
        .with_credentials(api_key, secret_key);
    
    // 您的程式碼邏輯
    
    println!("範例執行完成!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_example() {
        // 您的測試程式碼
    }
}
```

## 🐛 故障排除

### 常見問題

#### 1. 認證錯誤
```
Error: Missing credentials
```

**解決方案:**
```bash
# 確保設置了環境變數
export FUBON_API_KEY=your_actual_api_key
export FUBON_SECRET_KEY=your_actual_secret_key

# 或檢查 .env 檔案是否正確
cat .env
```

#### 2. 網路連接問題
```
Error: Connection error
```

**解決方案:**
- 檢查網路連接
- 確認防火牆設置
- 檢查 API 端點是否可訪問

#### 3. 編譯錯誤
```
Error: could not compile
```

**解決方案:**
```bash
# 更新依賴
cargo update

# 清理並重新編譯
cargo clean
cargo build
```

### 除錯技巧

#### 啟用詳細日誌
```bash
RUST_LOG=debug cargo run --example basic_trading
```

#### 使用除錯模式
```bash
cargo run --example basic_trading --debug
```

#### 檢查環境變數
```bash
env | grep FUBON
```

## 📚 進階主題

### 自定義事件處理

```rust
use std::sync::Arc;
use r_fubon_neo::market_data::websocket::{EventHandler, WebSocketEvent};

struct CustomHandler {
    name: String,
}

impl EventHandler for CustomHandler {
    fn handle_event(&self, event: WebSocketEvent) {
        // 自定義處理邏輯
    }
}
```

### 錯誤處理最佳實踐

```rust
use r_fubon_neo::{Error, Result};

async fn handle_api_call() -> Result<()> {
    match some_api_call().await {
        Ok(result) => {
            // 處理成功結果
            Ok(())
        }
        Err(Error::Authentication(msg)) => {
            eprintln!("認證錯誤: {}", msg);
            Err(Error::Authentication(msg))
        }
        Err(Error::Network(msg)) => {
            eprintln!("網路錯誤: {}", msg);
            // 可能重試邏輯
            Err(Error::Network(msg))
        }
        Err(e) => {
            eprintln!("其他錯誤: {}", e);
            Err(e)
        }
    }
}
```

### 異步編程技巧

```rust
use tokio::time::{sleep, Duration};
use futures::future::join_all;

async fn concurrent_requests() -> Result<()> {
    let symbols = vec!["2330", "2317", "2454"];
    
    // 並發請求
    let futures = symbols.iter().map(|symbol| {
        async move {
            // 每個 symbol 的處理邏輯
        }
    });
    
    let results = join_all(futures).await;
    
    Ok(())
}
```

## 🤝 貢獻範例

歡迎貢獻新的範例！請遵循以下指南：

1. **添加清晰的文檔註釋**
2. **包含錯誤處理**
3. **添加適當的測試**
4. **遵循現有的程式碼風格**
5. **更新這個 README**

## 📞 幫助

如果您在運行範例時遇到問題：

- 查看 [API 文檔](../docs/API.md)
- 查看 [開發者指南](../docs/DEVELOPMENT.md)
- 提交 [GitHub Issue](https://github.com/SDpower/r-fubon-neo/issues)
- 聯繫開發者: Steve Lo (info@sd.idv.tw)

## ⚠️ P.O.C 專案免責聲明

本專案為概念驗證 (Proof of Concept) 階段：

- 🚫 **不適用於生產環境**
- 📚 **僅供教育和學習使用**
- 🔬 **實驗性質，API 可能變更**
- 💡 **展示 Rust 在金融科技的應用潛力**