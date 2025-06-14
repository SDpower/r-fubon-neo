pub mod rest;
pub mod websocket;

pub use rest::RestClient;
pub use websocket::WebSocketClient;

use crate::Result;

/// Market data mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Speed,
    Standard,
}

impl Mode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Mode::Speed => "speed",
            Mode::Standard => "standard",
        }
    }
}

/// Market data wrapper combining REST and WebSocket clients
pub struct MarketData {
    pub websocket_client: WebSocketClient,
    pub rest_client: RestClient,
}

impl MarketData {
    pub fn new(sdk_token: String, mode: Mode) -> Result<Self> {
        let websocket_client = WebSocketClient::new(mode, sdk_token.clone())?;
        let rest_client = RestClient::new(sdk_token)?;
        
        Ok(Self {
            websocket_client,
            rest_client,
        })
    }
}