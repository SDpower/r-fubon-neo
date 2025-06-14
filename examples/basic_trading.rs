/*!
# 基本交易範例 (P.O.C)

⚠️ **P.O.C 專案**: 本範例僅供概念驗證和學習使用，不適用於實際交易。
👨‍💻 **開發者**: Steve Lo (info@sd.idv.tw)

本範例展示如何使用 r-fubon-neo SDK 進行基本的股票交易操作。

## 功能
- 初始化 SDK
- 檢查帳戶餘額 (模擬)
- 下限價買單 (模擬)
- 查詢訂單狀態 (模擬)
- 取消訂單 (模擬)

## 使用方法
```bash
# 方法1: 使用 .env 檔案 (推薦)
cp .env.example .env
# 編輯 .env 檔案，填入測試值或實際認證資訊

# 方法2: 設置環境變數
export FUBON_API_KEY=test_api_key
export FUBON_SECRET_KEY=test_secret_key

# 運行範例
cargo run --example basic_trading
```

## ⚠️ 重要提醒
- 這是模擬交易範例，不會執行真實交易
- 僅供學習 Rust 金融 API 開發使用
- 請勿用於實際交易環境
*/

use r_fubon_neo::{
    FubonSDK, CoreSDK, Order, OrderType, BSAction, TimeInForce,
    Error, Result
};
use std::env;

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
    
    println!("🚀 初始化 Fubon Neo SDK (P.O.C 版本)...");
    println!("📝 專案開發者: Steve Lo (info@sd.idv.tw)");
    println!("⚠️  注意: 這是概念驗證專案，所有操作均為模擬");
    
    // 創建 SDK 實例
    let sdk = FubonSDK::new()
        .with_credentials(api_key, secret_key);
    
    // 1. 檢查帳戶餘額
    println!("\n💰 檢查帳戶餘額...");
    match sdk.get_account_balance() {
        Ok(balance) => {
            println!("帳戶餘額: ${:.2}", balance);
            
            if balance < 100000.0 {
                println!("⚠️  餘額可能不足以進行交易");
            }
        }
        Err(e) => {
            eprintln!("❌ 獲取餘額失敗: {}", e);
            return Err(e);
        }
    }
    
    // 2. 查看目前持倉
    println!("\n📊 查看目前持倉...");
    match sdk.get_positions() {
        Ok(positions) => {
            if positions.is_empty() {
                println!("目前無持倉");
            } else {
                println!("目前持倉:");
                for symbol in &positions {
                    println!("  - {}", symbol);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ 獲取持倉失敗: {}", e);
        }
    }
    
    // 3. 下限價買單 (台積電)
    println!("\n📈 準備下台積電限價買單...");
    
    let order = Order {
        symbol: "2330".to_string(),    // 台積電
        quantity: 1000,                // 1張 (1000股)
        price: Some(500.0),            // 限價 500 元
        order_type: OrderType::Limit,  // 限價單
        time_in_force: TimeInForce::Day, // 當日有效
        action: BSAction::Buy,         // 買入
    };
    
    println!("訂單詳情:");
    println!("  股票代號: {}", order.symbol);
    println!("  數量: {} 股", order.quantity);
    println!("  價格: ${:.2}", order.price.unwrap());
    println!("  訂單類型: {:?}", order.order_type);
    println!("  有效期: {:?}", order.time_in_force);
    println!("  買賣別: {:?}", order.action);
    
    // 確認是否要送出訂單
    println!("\n⚠️  這是模擬訂單，實際上不會送出真實交易");
    
    match sdk.place_order(&order) {
        Ok(order_id) => {
            println!("✅ 訂單送出成功!");
            println!("訂單 ID: {}", order_id);
            
            // 4. 查詢訂單狀態
            println!("\n🔍 查詢訂單狀態...");
            match sdk.get_order_status(&order_id) {
                Ok(status) => {
                    println!("訂單狀態: {}", status);
                }
                Err(e) => {
                    eprintln!("❌ 查詢訂單狀態失敗: {}", e);
                }
            }
            
            // 5. 模擬取消訂單 (僅作示範)
            println!("\n❌ 模擬取消訂單...");
            match sdk.cancel_order(&order_id) {
                Ok(_) => {
                    println!("✅ 訂單取消成功");
                }
                Err(e) => {
                    eprintln!("❌ 取消訂單失敗: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ 下單失敗: {}", e);
            return Err(e);
        }
    }
    
    println!("\n🎉 基本交易範例完成!");
    println!("📚 這是 P.O.C (概念驗證) 專案的模擬示範");
    println!("👨‍💻 專案開發者: Steve Lo (info@sd.idv.tw)");
    println!("💡 目的: 探索 Rust 在金融科技領域的應用潛力");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_order_creation() {
        let order = Order {
            symbol: "2330".to_string(),
            quantity: 1000,
            price: Some(500.0),
            order_type: OrderType::Limit,
            time_in_force: TimeInForce::Day,
            action: BSAction::Buy,
        };
        
        assert_eq!(order.symbol, "2330");
        assert_eq!(order.quantity, 1000);
        assert_eq!(order.price, Some(500.0));
    }
    
    #[test]
    fn test_sdk_creation() {
        let sdk = FubonSDK::new()
            .with_credentials("test_key".to_string(), "test_secret".to_string());
        
        // SDK 應該能夠正常創建
        // 在實際測試中，這裡會有更多的驗證邏輯
    }
}