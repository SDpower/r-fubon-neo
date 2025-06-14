# API 文檔

r-fubon-neo API 完整參考文檔

> **⚠️ P.O.C 專案提醒**: 本專案為概念驗證階段，API 可能會有重大變更。請勿用於生產環境。  
> **開發者**: Steve Lo (info@sd.idv.tw)

## 目錄

- [核心 SDK](#核心-sdk)
- [市場數據](#市場數據)
- [交易功能](#交易功能)
- [錯誤處理](#錯誤處理)
- [類型定義](#類型定義)

## 核心 SDK

### FubonSDK

主要 SDK 類別，提供所有交易和市場數據功能。

#### 初始化

```rust
use r_fubon_neo::{FubonSDK, CoreSDK};

// 創建新的 SDK 實例
let sdk = FubonSDK::new();

// 設置認證資訊
let sdk = FubonSDK::new()
    .with_credentials("api_key".to_string(), "secret_key".to_string());
```

#### 方法

##### `new() -> Self`
創建新的 SDK 實例。

##### `with_credentials(api_key: String, secret_key: String) -> Self`
設置 API 認證資訊。

**參數:**
- `api_key`: API 金鑰
- `secret_key`: 秘密金鑰

##### `init_realtime(mode: Mode) -> Result<()>`
初始化即時市場數據。

**參數:**
- `mode`: 市場數據模式 (`Mode::Speed` 或 `Mode::Standard`)

**錯誤:**
- `Error::MissingCredentials`: 缺少認證資訊
- `Error::Authentication`: 認證失敗

## 市場數據

### Mode

市場數據模式枚舉。

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Speed,    // 速度模式，不支援某些頻道
    Standard, // 標準模式，支援所有功能
}
```

### MarketData

市場數據容器，包含 REST 和 WebSocket 客戶端。

```rust
let market_data = sdk.market_data().unwrap();
```

#### 屬性

- `websocket_client: WebSocketClient` - WebSocket 客戶端
- `rest_client: RestClient` - REST 客戶端

### RestClient

REST API 客戶端，用於獲取歷史和快照數據。

#### 股票數據

```rust
let stock_client = market_data.rest_client.stock()?;

// 獲取即時數據
let intraday = stock_client.intraday()?.get_data("2330").await?;

// 獲取歷史數據
let historical = stock_client.historical()?
    .get_data("2330", "2024-01-01", "2024-01-31").await?;

// 獲取快照數據
let snapshot = stock_client.snapshot()?.get_data("2330").await?;
```

### WebSocketClient

WebSocket 客戶端，用於即時數據串流。

#### 連接和認證

```rust
let mut ws_client = WebSocketClient::new(Mode::Speed, "sdk_token".to_string())?;
ws_client.connect().await?;
```

#### 事件處理

```rust
use std::sync::Arc;
use r_fubon_neo::market_data::websocket::{EventHandler, WebSocketEvent};

struct MyEventHandler;

impl EventHandler for MyEventHandler {
    fn handle_event(&self, event: WebSocketEvent) {
        match event {
            WebSocketEvent::Message(msg) => {
                println!("收到消息: {}", msg);
            }
            WebSocketEvent::Authenticated(_) => {
                println!("認證成功");
            }
            WebSocketEvent::Error(err) => {
                eprintln!("錯誤: {}", err);
            }
            _ => {}
        }
    }
}

ws_client.add_event_handler(Arc::new(MyEventHandler)).await;
```

#### 訂閱數據

```rust
use std::collections::HashMap;
use serde_json::json;

// 訂閱股票報價
let mut params = HashMap::new();
params.insert("channel".to_string(), json!("quote"));
params.insert("symbol".to_string(), json!("2330"));

ws_client.subscribe(params).await?;
```

## 交易功能

### CoreSDK Trait

所有交易功能的核心介面。

#### 帳戶功能

##### `get_account_balance() -> Result<f64>`
獲取帳戶餘額。

```rust
let balance = sdk.get_account_balance()?;
println!("餘額: ${:.2}", balance);
```

##### `get_positions() -> Result<Vec<String>>`
獲取持倉列表。

```rust
let positions = sdk.get_positions()?;
for symbol in positions {
    println!("持倉: {}", symbol);
}
```

#### 訂單管理

##### `place_order(order: &Order) -> Result<String>`
下股票單。

```rust
use r_fubon_neo::{Order, OrderType, BSAction, TimeInForce};

let order = Order {
    symbol: "2330".to_string(),
    quantity: 1000,
    price: Some(500.0),
    order_type: OrderType::Limit,
    time_in_force: TimeInForce::Day,
    action: BSAction::Buy,
};

let order_id = sdk.place_order(&order)?;
println!("訂單 ID: {}", order_id);
```

##### `place_condition_order(condition_order: &ConditionOrder) -> Result<String>`
下條件單。

```rust
use r_fubon_neo::{ConditionOrder, Condition, Operator, TriggerContent};

let condition = Condition {
    symbol: "2330".to_string(),
    operator: Operator::GreaterThan,
    value: 550.0,
    trigger_content: TriggerContent::Price,
};

let condition_order = ConditionOrder {
    condition,
    order,
    order_type: ConditionOrderType::Stop,
};

let order_id = sdk.place_condition_order(&condition_order)?;
```

##### `cancel_order(order_id: &str) -> Result<()>`
取消訂單。

```rust
sdk.cancel_order("order_123")?;
```

##### `get_order_status(order_id: &str) -> Result<String>`
查詢訂單狀態。

```rust
let status = sdk.get_order_status("order_123")?;
println!("訂單狀態: {}", status);
```

#### 期貨選擇權

##### `place_futopt_order(order: &FutOptOrder) -> Result<String>`
下期貨/選擇權單。

```rust
use r_fubon_neo::{FutOptOrder, FutOptOrderType};

let futopt_order = FutOptOrder {
    symbol: "TXF202412".to_string(),
    quantity: 1,
    price: Some(18000.0),
    order_type: FutOptOrderType::Limit,
    action: BSAction::Buy,
};

let order_id = sdk.place_futopt_order(&futopt_order)?;
```

## 錯誤處理

### Error 枚舉

```rust
#[derive(Error, Debug)]
pub enum Error {
    #[error("HTTP request failed: {0}")]
    Http(reqwest::Error),
    
    #[error("WebSocket error: {0}")]
    WebSocket(String),
    
    #[error("JSON serialization error: {0}")]
    Json(serde_json::Error),
    
    #[error("Authentication failed: {0}")]
    Authentication(String),
    
    #[error("Authentication timeout")]
    AuthenticationTimeout,
    
    #[error("Missing credentials")]
    MissingCredentials,
    
    #[error("Unauthenticated")]
    Unauthenticated,
    
    #[error("Invalid mode for channel: {channel} not supported in {mode} mode")]
    InvalidModeForChannel { channel: String, mode: String },
    
    #[error("Connection error: {0}")]
    Connection(String),
    
    #[error("General error: {0}")]
    General(String),
}
```

### 錯誤處理範例

```rust
use r_fubon_neo::{Error, Result};

match sdk.get_account_balance() {
    Ok(balance) => println!("餘額: ${:.2}", balance),
    Err(Error::MissingCredentials) => {
        eprintln!("請設置 API 認證資訊");
    }
    Err(Error::Authentication(msg)) => {
        eprintln!("認證失敗: {}", msg);
    }
    Err(e) => {
        eprintln!("發生錯誤: {}", e);
    }
}
```

## 類型定義

### 交易相關

#### TimeInForce
```rust
pub enum TimeInForce {
    Day,  // 當日有效
    Ioc,  // 立即成交或取消
    Fok,  // 全部成交或取消
    Gtc,  // 有效直至取消
}
```

#### OrderType
```rust
pub enum OrderType {
    Market,     // 市價單
    Limit,      // 限價單
    Stop,       // 停損單
    StopLimit,  // 停損限價單
}
```

#### BSAction
```rust
pub enum BSAction {
    Buy,   // 買入
    Sell,  // 賣出
}
```

#### MarketType
```rust
pub enum MarketType {
    Stock,   // 股票
    Future,  // 期貨
    Option,  // 選擇權
}
```

### 條件單相關

#### Operator
```rust
pub enum Operator {
    GreaterThan,    // 大於
    LessThan,       // 小於
    GreaterEqual,   // 大於等於
    LessEqual,      // 小於等於
    Equal,          // 等於
}
```

#### TriggerContent
```rust
pub enum TriggerContent {
    Price,   // 價格觸發
    Volume,  // 成交量觸發
}
```

### 結構體

#### Order
```rust
pub struct Order {
    pub symbol: String,              // 商品代號
    pub quantity: u32,               // 數量
    pub price: Option<f64>,          // 價格 (市價單為 None)
    pub order_type: OrderType,       // 訂單類型
    pub time_in_force: TimeInForce, // 時效性
    pub action: BSAction,            // 買賣別
}
```

#### Condition
```rust
pub struct Condition {
    pub symbol: String,                 // 觸發商品代號
    pub operator: Operator,             // 比較運算子
    pub value: f64,                     // 觸發值
    pub trigger_content: TriggerContent, // 觸發內容
}
```

#### ConditionOrder
```rust
pub struct ConditionOrder {
    pub condition: Condition,                    // 觸發條件
    pub order: Order,                           // 要執行的訂單
    pub order_type: ConditionOrderType,         // 條件單類型
}
```

## 範例程式碼

完整的使用範例請參考 [examples](../examples/) 目錄。

### 基本交易範例

```rust
use r_fubon_neo::{FubonSDK, CoreSDK, Order, OrderType, BSAction, TimeInForce};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sdk = FubonSDK::new()
        .with_credentials("your_api_key".to_string(), "your_secret_key".to_string());
    
    // 檢查餘額
    let balance = sdk.get_account_balance()?;
    println!("可用餘額: ${:.2}", balance);
    
    // 下限價買單
    let order = Order {
        symbol: "2330".to_string(),
        quantity: 1000,
        price: Some(500.0),
        order_type: OrderType::Limit,
        time_in_force: TimeInForce::Day,
        action: BSAction::Buy,
    };
    
    let order_id = sdk.place_order(&order)?;
    println!("訂單已提交，ID: {}", order_id);
    
    // 查詢訂單狀態
    let status = sdk.get_order_status(&order_id)?;
    println!("訂單狀態: {}", status);
    
    Ok(())
}
```

### WebSocket 即時數據範例

```rust
use r_fubon_neo::{FubonSDK, Mode, market_data::websocket::{EventHandler, WebSocketEvent}};
use std::sync::Arc;
use std::collections::HashMap;
use serde_json::json;

struct QuoteHandler;

impl EventHandler for QuoteHandler {
    fn handle_event(&self, event: WebSocketEvent) {
        match event {
            WebSocketEvent::Message(msg) => {
                println!("即時報價: {}", msg);
            }
            WebSocketEvent::Authenticated(_) => {
                println!("WebSocket 認證成功");
            }
            WebSocketEvent::Error(err) => {
                eprintln!("WebSocket 錯誤: {}", err);
            }
            _ => {}
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut sdk = FubonSDK::new()
        .with_credentials("your_api_key".to_string(), "your_secret_key".to_string());
    
    // 初始化市場數據
    sdk.init_realtime(Mode::Speed)?;
    
    if let Some(market_data) = sdk.market_data_mut() {
        // 添加事件處理器
        market_data.websocket_client
            .add_event_handler(Arc::new(QuoteHandler)).await;
        
        // 連接 WebSocket
        market_data.websocket_client.connect().await?;
        
        // 訂閱台積電報價
        let mut params = HashMap::new();
        params.insert("channel".to_string(), json!("quote"));
        params.insert("symbol".to_string(), json!("2330"));
        
        market_data.websocket_client.subscribe(params).await?;
        
        // 保持連接
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }
    
    Ok(())
}
```