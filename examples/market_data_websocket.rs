/*!
# WebSocket 市場數據範例 (P.O.C)

⚠️ **P.O.C 專案**: 本範例僅供概念驗證和學習使用，不會連接真實市場數據。
👨‍💻 **開發者**: Steve Lo (info@sd.idv.tw)

本範例展示如何使用 r-fubon-neo SDK 的 WebSocket 功能來接收即時市場數據。

## 功能
- 初始化市場數據連接 (模擬)
- WebSocket 事件處理
- 訂閱股票即時報價 (模擬)
- 處理連接斷線和重連

## 使用方法
```bash
# 方法1: 使用 .env 檔案 (推薦)
cp .env.example .env
# 編輯 .env 檔案，填入測試值或實際認證資訊

# 方法2: 設置環境變數
export FUBON_API_KEY=test_api_key
export FUBON_SECRET_KEY=test_secret_key

# 運行範例
cargo run --example market_data_websocket
```

## ⚠️ 重要提醒
- 這是模擬 WebSocket 連接範例
- 不會連接真實的市場數據源
- 僅供學習 Rust 異步編程和 WebSocket 處理
*/

use r_fubon_neo::{
    FubonSDK, CoreSDK, Mode,
    market_data::websocket::{EventHandler, WebSocketEvent},
    Error, Result
};
use std::env;
use std::sync::Arc;
use std::collections::HashMap;
use serde_json::{json, Value};
use tokio::time::{sleep, Duration};

/// 自定義事件處理器
struct MarketDataHandler {
    name: String,
}

impl MarketDataHandler {
    fn new(name: String) -> Self {
        Self { name }
    }
}

impl EventHandler for MarketDataHandler {
    fn handle_event(&self, event: WebSocketEvent) {
        match event {
            WebSocketEvent::Connect => {
                println!("🔗 [{}] WebSocket 連接成功", self.name);
            }
            
            WebSocketEvent::Disconnect { code, reason } => {
                println!("💔 [{}] WebSocket 連接斷開", self.name);
                if let Some(code) = code {
                    println!("    斷開代碼: {}", code);
                }
                if !reason.is_empty() {
                    println!("    斷開原因: {}", reason);
                }
            }
            
            WebSocketEvent::Message(msg) => {
                // 解析訊息
                match serde_json::from_str::<Value>(&msg) {
                    Ok(data) => {
                        if let Some(event_type) = data.get("event").and_then(|e| e.as_str()) {
                            match event_type {
                                "quote" => {
                                    self.handle_quote_data(&data);
                                }
                                "trade" => {
                                    self.handle_trade_data(&data);
                                }
                                "candle" => {
                                    self.handle_candle_data(&data);
                                }
                                "pong" => {
                                    println!("🏓 [{}] 收到 pong 回應", self.name);
                                }
                                "subscriptions" => {
                                    self.handle_subscriptions(&data);
                                }
                                _ => {
                                    println!("📨 [{}] 收到未知事件: {}", self.name, event_type);
                                    println!("    數據: {}", serde_json::to_string_pretty(&data).unwrap_or_default());
                                }
                            }
                        } else {
                            println!("📨 [{}] 收到原始訊息: {}", self.name, msg);
                        }
                    }
                    Err(_) => {
                        println!("📨 [{}] 收到非 JSON 訊息: {}", self.name, msg);
                    }
                }
            }
            
            WebSocketEvent::Error(err) => {
                eprintln!("❌ [{}] WebSocket 錯誤: {}", self.name, err);
            }
            
            WebSocketEvent::Authenticated(data) => {
                println!("✅ [{}] WebSocket 認證成功", self.name);
                if let Some(message) = data.get("data").and_then(|d| d.get("message")).and_then(|m| m.as_str()) {
                    println!("    認證訊息: {}", message);
                }
            }
            
            WebSocketEvent::Unauthenticated(data) => {
                eprintln!("🚫 [{}] WebSocket 認證失敗", self.name);
                if let Some(message) = data.get("data").and_then(|d| d.get("message")).and_then(|m| m.as_str()) {
                    eprintln!("    失敗原因: {}", message);
                }
            }
        }
    }
}

impl MarketDataHandler {
    /// 處理即時報價數據
    fn handle_quote_data(&self, data: &Value) {
        if let Some(quote_data) = data.get("data") {
            let symbol = quote_data.get("symbol")
                .and_then(|s| s.as_str())
                .unwrap_or("未知");
            let price = quote_data.get("price")
                .and_then(|p| p.as_f64())
                .unwrap_or(0.0);
            let volume = quote_data.get("volume")
                .and_then(|v| v.as_u64())
                .unwrap_or(0);
            
            println!("📊 [{}] 即時報價 - {}: ${:.2} (成交量: {})", 
                    self.name, symbol, price, volume);
        }
    }
    
    /// 處理交易數據
    fn handle_trade_data(&self, data: &Value) {
        if let Some(trade_data) = data.get("data") {
            let symbol = trade_data.get("symbol")
                .and_then(|s| s.as_str())
                .unwrap_or("未知");
            let price = trade_data.get("price")
                .and_then(|p| p.as_f64())
                .unwrap_or(0.0);
            let size = trade_data.get("size")
                .and_then(|s| s.as_u64())
                .unwrap_or(0);
            
            println!("💰 [{}] 成交紀錄 - {}: ${:.2} x {}", 
                    self.name, symbol, price, size);
        }
    }
    
    /// 處理 K 線數據
    fn handle_candle_data(&self, data: &Value) {
        if let Some(candle_data) = data.get("data") {
            let symbol = candle_data.get("symbol")
                .and_then(|s| s.as_str())
                .unwrap_or("未知");
            let open = candle_data.get("open")
                .and_then(|o| o.as_f64())
                .unwrap_or(0.0);
            let high = candle_data.get("high")
                .and_then(|h| h.as_f64())
                .unwrap_or(0.0);
            let low = candle_data.get("low")
                .and_then(|l| l.as_f64())
                .unwrap_or(0.0);
            let close = candle_data.get("close")
                .and_then(|c| c.as_f64())
                .unwrap_or(0.0);
            
            println!("📈 [{}] K線數據 - {}: O:{:.2} H:{:.2} L:{:.2} C:{:.2}", 
                    self.name, symbol, open, high, low, close);
        }
    }
    
    /// 處理訂閱列表
    fn handle_subscriptions(&self, data: &Value) {
        if let Some(subs) = data.get("data").and_then(|d| d.as_array()) {
            println!("📋 [{}] 目前訂閱列表:", self.name);
            for sub in subs {
                if let Some(sub_str) = sub.as_str() {
                    println!("    - {}", sub_str);
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // 載入 .env 檔案 (如果存在)
    let _ = dotenvy::dotenv();
    
    // 初始化日誌
    tracing_subscriber::fmt::init();
    
    // 從環境變數獲取 API 認證資訊 (支援多種變數名稱)
    let api_key = env::var("FUBON_API_KEY")
        .or_else(|_| env::var("API_KEY"))
        .map_err(|_| Error::general("請設置 FUBON_API_KEY 或 API_KEY 環境變數"))?;
    let secret_key = env::var("FUBON_SECRET_KEY")
        .or_else(|_| env::var("SECRET_KEY"))
        .map_err(|_| Error::general("請設置 FUBON_SECRET_KEY 或 SECRET_KEY 環境變數"))?;
    
    println!("🚀 初始化 Fubon Neo SDK WebSocket 連接 (P.O.C 版本)...");
    println!("📝 專案開發者: Steve Lo (info@sd.idv.tw)");
    println!("⚠️  注意: 這是概念驗證專案，WebSocket 連接為模擬");
    
    // 創建 SDK 實例
    let mut sdk = FubonSDK::new()
        .with_credentials(api_key, secret_key);
    
    // 初始化市場數據 (使用速度模式)
    println!("\n⚡ 初始化市場數據 (速度模式)...");
    sdk.init_realtime(Mode::Speed)?;
    
    if let Some(market_data) = sdk.market_data_mut() {
        // 添加事件處理器
        let handler = Arc::new(MarketDataHandler::new("市場數據".to_string()));
        market_data.websocket_client.add_event_handler(handler).await;
        
        // 連接 WebSocket
        println!("🔌 連接 WebSocket...");
        market_data.websocket_client.connect().await?;
        
        // 等待連接穩定
        sleep(Duration::from_secs(2)).await;
        
        // 訂閱台積電即時報價
        println!("\n📊 訂閱台積電 (2330) 即時報價...");
        let mut quote_params = HashMap::new();
        quote_params.insert("channel".to_string(), json!("quote"));
        quote_params.insert("symbol".to_string(), json!("2330"));
        
        match market_data.websocket_client.subscribe(quote_params).await {
            Ok(_) => println!("✅ 台積電報價訂閱成功"),
            Err(e) => eprintln!("❌ 台積電報價訂閱失敗: {}", e),
        }
        
        // 等待一段時間
        sleep(Duration::from_secs(3)).await;
        
        // 訂閱鴻海即時報價
        println!("\n📊 訂閱鴻海 (2317) 即時報價...");
        let mut quote_params = HashMap::new();
        quote_params.insert("channel".to_string(), json!("quote"));
        quote_params.insert("symbol".to_string(), json!("2317"));
        
        match market_data.websocket_client.subscribe(quote_params).await {
            Ok(_) => println!("✅ 鴻海報價訂閱成功"),
            Err(e) => eprintln!("❌ 鴻海報價訂閱失敗: {}", e),
        }
        
        // 等待一段時間
        sleep(Duration::from_secs(3)).await;
        
        // 查詢目前訂閱
        println!("\n📋 查詢目前訂閱...");
        match market_data.websocket_client.subscriptions().await {
            Ok(_) => println!("✅ 訂閱查詢請求已送出"),
            Err(e) => eprintln!("❌ 訂閱查詢失敗: {}", e),
        }
        
        // 等待一段時間
        sleep(Duration::from_secs(3)).await;
        
        // 發送 ping
        println!("\n🏓 發送 ping...");
        match market_data.websocket_client.ping("test ping").await {
            Ok(_) => println!("✅ Ping 已送出"),
            Err(e) => eprintln!("❌ Ping 送出失敗: {}", e),
        }
        
        // 持續接收數據
        println!("\n📡 持續接收市場數據 (60秒)...");
        println!("按 Ctrl+C 結束程式");
        
        // 設置訊號處理（優雅關閉）
        let running = Arc::new(std::sync::atomic::AtomicBool::new(true));
        let r = running.clone();
        
        ctrlc::set_handler(move || {
            println!("\n🛑 收到關閉訊號，正在優雅關閉...");
            r.store(false, std::sync::atomic::Ordering::SeqCst);
        })?;
        
        // 主循環
        let mut counter = 0;
        while running.load(std::sync::atomic::Ordering::SeqCst) && counter < 60 {
            sleep(Duration::from_secs(1)).await;
            counter += 1;
            
            // 每 10 秒顯示狀態
            if counter % 10 == 0 {
                println!("⏰ 運行中... ({}/60 秒)", counter);
                
                // 檢查連接狀態
                let auth_state = market_data.websocket_client.auth_state().await;
                println!("🔐 認證狀態: {:?}", auth_state);
            }
        }
        
        // 取消訂閱
        println!("\n❌ 取消台積電訂閱...");
        let mut unsubscribe_params = HashMap::new();
        unsubscribe_params.insert("channel".to_string(), json!("quote"));
        unsubscribe_params.insert("symbol".to_string(), json!("2330"));
        
        match market_data.websocket_client.unsubscribe(unsubscribe_params).await {
            Ok(_) => println!("✅ 台積電訂閱已取消"),
            Err(e) => eprintln!("❌ 取消台積電訂閱失敗: {}", e),
        }
        
        // 斷開連接
        println!("\n🔌 斷開 WebSocket 連接...");
        market_data.websocket_client.disconnect().await;
        
        sleep(Duration::from_secs(1)).await;
        
    } else {
        return Err(Error::general("無法獲取市場數據實例"));
    }
    
    println!("\n🎉 WebSocket 市場數據範例完成!");
    println!("📚 這是 P.O.C (概念驗證) 專案的 WebSocket 示範");
    println!("👨‍💻 專案開發者: Steve Lo (info@sd.idv.tw)");
    println!("💡 目的: 展示 Rust 異步 WebSocket 在金融數據處理的應用");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_event_handler_creation() {
        let handler = MarketDataHandler::new("測試".to_string());
        assert_eq!(handler.name, "測試");
    }
    
    #[tokio::test]
    async fn test_market_data_initialization() {
        let mut sdk = FubonSDK::new()
            .with_credentials("test_key".to_string(), "test_secret".to_string());
        
        // 這在實際測試中會失敗，因為沒有真實的認證
        // 但可以測試代碼結構
        let result = sdk.init_realtime(Mode::Speed);
        
        // 在模擬環境中，這應該會返回錯誤
        assert!(result.is_err());
    }
}