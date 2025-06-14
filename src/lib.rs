//! # r-fubon-neo
//! 
//! Rust implementation of Fubon Neo SDK for trading and market data.
//! Compatible with Python 3.12+ fubon_neo package.

pub mod constants;
pub mod error;
pub mod market_data;
pub mod sdk;
pub mod types;

pub use error::{Error, Result};
pub use sdk::{FubonSDK, CoreSDK};
pub use market_data::{MarketData, RestClient, WebSocketClient, Mode};
pub use types::*;

/// Library version, matching Python package version
pub const VERSION: &str = "2.2.3";

/// Get library version
pub fn version() -> &'static str {
    VERSION
}