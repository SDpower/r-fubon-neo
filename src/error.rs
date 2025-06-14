use thiserror::Error;

/// Result type alias for this crate
pub type Result<T> = std::result::Result<T, Error>;

/// Error types for Fubon Neo SDK
#[derive(Error, Debug)]
pub enum Error {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
    
    #[error("WebSocket error: {0}")]
    WebSocket(String),
    
    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),
    
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
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("URL parse error: {0}")]
    UrlParse(#[from] url::ParseError),
    
    #[error("General error: {0}")]
    General(String),
}

impl Error {
    pub fn websocket<T: Into<String>>(msg: T) -> Self {
        Error::WebSocket(msg.into())
    }
    
    pub fn authentication<T: Into<String>>(msg: T) -> Self {
        Error::Authentication(msg.into())
    }
    
    pub fn connection<T: Into<String>>(msg: T) -> Self {
        Error::Connection(msg.into())
    }
    
    pub fn general<T: Into<String>>(msg: T) -> Self {
        Error::General(msg.into())
    }
}