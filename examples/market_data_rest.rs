/*!
# REST API 市場數據範例 (P.O.C)

⚠️ **P.O.C 專案**: 本範例僅供概念驗證和學習使用，不會連接真實 API 端點。
👨‍💻 **開發者**: Steve Lo (info@sd.idv.tw)

本範例展示如何使用 r-fubon-neo SDK 的 REST API 功能來獲取歷史和快照市場數據。

## 功能
- 獲取股票快照數據 (模擬)
- 獲取股票即時數據 (模擬)
- 獲取股票歷史數據 (模擬)
- 錯誤處理示範

## 使用方法
```bash
# 方法1: 使用 .env 檔案 (推薦)
cp .env.example .env
# 編輯 .env 檔案，填入測試值或實際認證資訊

# 方法2: 設置環境變數
export FUBON_API_KEY=test_api_key
export FUBON_SECRET_KEY=test_secret_key

# 運行範例
cargo run --example market_data_rest
```

## ⚠️ 重要提醒
- 這是模擬 REST API 調用範例
- 不會連接真實的市場數據 API
- 僅供學習 Rust HTTP 客戶端和 JSON 處理
*/

use r_fubon_neo::{
    FubonSDK, CoreSDK, Mode,
    Error, Result
};
use std::env;
use serde_json::Value;
use tokio::time::{sleep, Duration};

/// 格式化 JSON 數據以便顯示
fn format_json_value(value: &Value, indent: usize) -> String {
    let prefix = "  ".repeat(indent);
    
    match value {
        Value::Object(map) => {
            let mut result = String::new();
            for (key, val) in map {
                result.push_str(&format!("{}{}:", prefix, key));
                if val.is_object() || val.is_array() {
                    result.push('\n');
                    result.push_str(&format_json_value(val, indent + 1));
                } else {
                    result.push(' ');
                    result.push_str(&format_json_value(val, 0));
                    result.push('\n');
                }
            }
            result
        }
        Value::Array(arr) => {
            let mut result = String::new();
            for (i, val) in arr.iter().enumerate() {
                result.push_str(&format!("{}[{}]:", prefix, i));
                if val.is_object() || val.is_array() {
                    result.push('\n');
                    result.push_str(&format_json_value(val, indent + 1));
                } else {
                    result.push(' ');
                    result.push_str(&format_json_value(val, 0));
                    result.push('\n');
                }
            }
            result
        }
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Null => "null".to_string(),
    }
}

/// 處理股票快照數據
async fn handle_snapshot_data(stock_client: &r_fubon_neo::market_data::rest::RestStockClient, symbol: &str) -> Result<()> {
    println!("\n📸 獲取 {} 快照數據...", symbol);
    
    match stock_client.snapshot() {
        Ok(snapshot_client) => {
            match snapshot_client.get_data(symbol).await {
                Ok(data) => {
                    println!("✅ {} 快照數據獲取成功:", symbol);
                    
                    // 解析並顯示關鍵數據
                    if let Some(price) = data.get("price").and_then(|p| p.as_f64()) {
                        println!("  📊 目前價格: ${:.2}", price);
                    }
                    
                    if let Some(volume) = data.get("volume").and_then(|v| v.as_u64()) {
                        println!("  📈 成交量: {}", volume);
                    }
                    
                    if let Some(change) = data.get("change").and_then(|c| c.as_f64()) {
                        println!("  📉 漲跌: ${:.2}", change);
                    }
                    
                    if let Some(change_percent) = data.get("changePercent").and_then(|cp| cp.as_f64()) {
                        println!("  📊 漲跌幅: {:.2}%", change_percent);
                    }
                    
                    // 顯示完整數據（限制輸出長度）
                    let formatted = format_json_value(&data, 1);
                    let lines: Vec<&str> = formatted.lines().collect();
                    if lines.len() > 10 {
                        println!("  完整數據 (前10行):");
                        for line in lines.iter().take(10) {
                            println!("  {}", line);
                        }
                        println!("  ... (還有 {} 行)", lines.len() - 10);
                    } else {
                        println!("  完整數據:");
                        println!("{}", formatted);
                    }
                }
                Err(e) => {
                    eprintln!("❌ {} 快照數據獲取失敗: {}", symbol, e);
                    return Err(e);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ 創建快照客戶端失敗: {}", e);
            return Err(e);
        }
    }
    
    Ok(())
}

/// 處理股票即時數據
async fn handle_intraday_data(stock_client: &r_fubon_neo::market_data::rest::RestStockClient, symbol: &str) -> Result<()> {
    println!("\n⚡ 獲取 {} 即時數據...", symbol);
    
    match stock_client.intraday() {
        Ok(intraday_client) => {
            match intraday_client.get_data(symbol).await {
                Ok(data) => {
                    println!("✅ {} 即時數據獲取成功:", symbol);
                    
                    // 顯示數據摘要
                    if let Some(arr) = data.as_array() {
                        println!("  📊 數據點數量: {}", arr.len());
                        
                        // 顯示最新的幾個數據點
                        let show_count = std::cmp::min(3, arr.len());
                        println!("  📈 最新 {} 個數據點:", show_count);
                        
                        for (i, item) in arr.iter().rev().take(show_count).enumerate() {
                            if let Some(obj) = item.as_object() {
                                let time = obj.get("time")
                                    .and_then(|t| t.as_str())
                                    .unwrap_or("未知時間");
                                let price = obj.get("price")
                                    .and_then(|p| p.as_f64())
                                    .unwrap_or(0.0);
                                let volume = obj.get("volume")
                                    .and_then(|v| v.as_u64())
                                    .unwrap_or(0);
                                
                                println!("    {}. {} - ${:.2} (量: {})", 
                                        show_count - i, time, price, volume);
                            }
                        }
                    } else {
                        println!("  數據格式:");
                        println!("{}", format_json_value(&data, 1));
                    }
                }
                Err(e) => {
                    eprintln!("❌ {} 即時數據獲取失敗: {}", symbol, e);
                    return Err(e);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ 創建即時數據客戶端失敗: {}", e);
            return Err(e);
        }
    }
    
    Ok(())
}

/// 處理股票歷史數據
async fn handle_historical_data(stock_client: &r_fubon_neo::market_data::rest::RestStockClient, symbol: &str) -> Result<()> {
    println!("\n📅 獲取 {} 歷史數據...", symbol);
    
    // 設置日期範圍（最近一個月）
    let to_date = chrono::Utc::now().format("%Y-%m-%d").to_string();
    let from_date = (chrono::Utc::now() - chrono::Duration::days(30))
        .format("%Y-%m-%d").to_string();
    
    println!("  📆 日期範圍: {} 到 {}", from_date, to_date);
    
    match stock_client.historical() {
        Ok(historical_client) => {
            match historical_client.get_data(symbol, &from_date, &to_date).await {
                Ok(data) => {
                    println!("✅ {} 歷史數據獲取成功:", symbol);
                    
                    // 顯示數據摘要
                    if let Some(arr) = data.as_array() {
                        println!("  📊 歷史數據點數量: {}", arr.len());
                        
                        // 顯示最新和最舊的數據點
                        if let (Some(first), Some(last)) = (arr.first(), arr.last()) {
                            println!("  📈 數據範圍:");
                            
                            // 最舊的數據
                            if let Some(obj) = first.as_object() {
                                let date = obj.get("date")
                                    .and_then(|d| d.as_str())
                                    .unwrap_or("未知日期");
                                let close = obj.get("close")
                                    .and_then(|c| c.as_f64())
                                    .unwrap_or(0.0);
                                println!("    開始: {} - 收盤價 ${:.2}", date, close);
                            }
                            
                            // 最新的數據
                            if let Some(obj) = last.as_object() {
                                let date = obj.get("date")
                                    .and_then(|d| d.as_str())
                                    .unwrap_or("未知日期");
                                let close = obj.get("close")
                                    .and_then(|c| c.as_f64())
                                    .unwrap_or(0.0);
                                println!("    結束: {} - 收盤價 ${:.2}", date, close);
                            }
                        }
                        
                        // 計算統計資訊
                        let mut prices = Vec::new();
                        for item in arr {
                            if let Some(close) = item.get("close").and_then(|c| c.as_f64()) {
                                prices.push(close);
                            }
                        }
                        
                        if !prices.is_empty() {
                            let max_price = prices.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
                            let min_price = prices.iter().fold(f64::INFINITY, |a, &b| a.min(b));
                            let avg_price = prices.iter().sum::<f64>() / prices.len() as f64;
                            
                            println!("  📊 統計資訊:");
                            println!("    最高價: ${:.2}", max_price);
                            println!("    最低價: ${:.2}", min_price);
                            println!("    平均價: ${:.2}", avg_price);
                        }
                    } else {
                        println!("  數據格式:");
                        println!("{}", format_json_value(&data, 1));
                    }
                }
                Err(e) => {
                    eprintln!("❌ {} 歷史數據獲取失敗: {}", symbol, e);
                    return Err(e);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ 創建歷史數據客戶端失敗: {}", e);
            return Err(e);
        }
    }
    
    Ok(())
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
    
    println!("🚀 初始化 Fubon Neo SDK REST API (P.O.C 版本)...");
    println!("📝 專案開發者: Steve Lo (info@sd.idv.tw)");
    println!("⚠️  注意: 這是概念驗證專案，REST API 調用為模擬");
    
    // 創建 SDK 實例
    let mut sdk = FubonSDK::new()
        .with_credentials(api_key, secret_key);
    
    // 初始化市場數據
    println!("\n⚡ 初始化市場數據...");
    sdk.init_realtime(Mode::Standard)?;
    
    if let Some(market_data) = sdk.market_data() {
        // 獲取股票 REST 客戶端
        match market_data.rest_client.stock() {
            Ok(stock_client) => {
                println!("✅ 股票 REST 客戶端初始化成功");
                
                // 要查詢的股票列表
                let symbols = vec!["2330", "2317", "2454"];
                
                for symbol in &symbols {
                    println!("\n" + &"=".repeat(50));
                    println!("🏢 處理股票: {} ", symbol);
                    
                    // 獲取快照數據
                    if let Err(e) = handle_snapshot_data(&stock_client, symbol).await {
                        eprintln!("⚠️  快照數據處理失敗，繼續下一步: {}", e);
                    }
                    
                    // 等待一秒避免 API 限制
                    sleep(Duration::from_millis(500)).await;
                    
                    // 獲取即時數據
                    if let Err(e) = handle_intraday_data(&stock_client, symbol).await {
                        eprintln!("⚠️  即時數據處理失敗，繼續下一步: {}", e);
                    }
                    
                    // 等待一秒避免 API 限制
                    sleep(Duration::from_millis(500)).await;
                    
                    // 獲取歷史數據
                    if let Err(e) = handle_historical_data(&stock_client, symbol).await {
                        eprintln!("⚠️  歷史數據處理失敗，繼續下一步: {}", e);
                    }
                    
                    // 等待一秒避免 API 限制
                    sleep(Duration::from_secs(1)).await;
                }
                
                println!("\n" + &"=".repeat(50));
                println!("📊 數據獲取摘要完成");
                
            }
            Err(e) => {
                eprintln!("❌ 股票 REST 客戶端初始化失敗: {}", e);
                return Err(e);
            }
        }
    } else {
        return Err(Error::general("無法獲取市場數據實例"));
    }
    
    println!("\n🎉 REST API 市場數據範例完成!");
    println!("📚 這是 P.O.C (概念驗證) 專案的 REST API 示範");
    println!("👨‍💻 專案開發者: Steve Lo (info@sd.idv.tw)");
    println!("💡 目的: 展示 Rust HTTP 客戶端在金融數據獲取的應用");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    
    #[test]
    fn test_format_json_value() {
        let test_data = json!({
            "symbol": "2330",
            "price": 500.0,
            "volume": 1000
        });
        
        let formatted = format_json_value(&test_data, 0);
        assert!(formatted.contains("symbol"));
        assert!(formatted.contains("2330"));
    }
    
    #[tokio::test]
    async fn test_market_data_initialization() {
        let mut sdk = FubonSDK::new()
            .with_credentials("test_key".to_string(), "test_secret".to_string());
        
        // 這在實際測試中會失敗，因為沒有真實的認證
        let result = sdk.init_realtime(Mode::Standard);
        
        // 在模擬環境中，這應該會返回錯誤
        assert!(result.is_err());
    }
}