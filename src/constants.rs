//! Constants used throughout the SDK, matching Python version

pub const AUTHENTICATION_TIMEOUT_MESSAGE: &str = "Authentication timeout";
pub const CONNECT_EVENT: &str = "connect";
pub const DISCONNECT_EVENT: &str = "disconnect";
pub const MESSAGE_EVENT: &str = "message";
pub const ERROR_EVENT: &str = "error";
pub const AUTHENTICATED_EVENT: &str = "authenticated";
pub const MISSING_CREDENTIALS_MESSAGE: &str = "One of the \"apiKey\", \"bearerToken\", or \"sdkToken\" options must be specified";
pub const UNAUTHENTICATED_EVENT: &str = "unauthenticated";
pub const UNAUTHENTICATED_MESSAGE: &str = "Unauthenticated";

/// WebSocket connection timeout in seconds
pub const WEBSOCKET_TIMEOUT: u64 = 5;

/// Authentication timeout in seconds  
pub const AUTH_TIMEOUT: u64 = 5;

/// Ping interval in seconds
pub const PING_INTERVAL: u64 = 30;

/// Maximum missed pongs before disconnect
pub const MAX_MISSED_PONGS: u32 = 2;