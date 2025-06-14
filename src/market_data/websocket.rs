use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{mpsc, Mutex};
use tokio::time::{sleep, timeout};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use futures_util::{SinkExt, StreamExt};
use serde_json::{json, Value};
use url::Url;

use crate::{Result, Error};
use crate::constants::*;
use crate::market_data::Mode;

/// Authentication state for WebSocket connection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthenticationState {
    Pending,
    Authenticating,
    Authenticated,
    Unauthenticated,
}

/// WebSocket event types
#[derive(Debug, Clone)]
pub enum WebSocketEvent {
    Connect,
    Disconnect { code: Option<u16>, reason: String },
    Message(String),
    Error(String),
    Authenticated(Value),
    Unauthenticated(Value),
}

/// Event handler trait
pub trait EventHandler: Send + Sync {
    fn handle_event(&self, event: WebSocketEvent);
}

/// WebSocket client configuration
#[derive(Debug, Clone)]
pub struct WebSocketConfig {
    pub base_url: String,
    pub api_key: Option<String>,
    pub bearer_token: Option<String>,
    pub sdk_token: Option<String>,
}

impl WebSocketConfig {
    pub fn new() -> Self {
        Self {
            base_url: "wss://api.fubon.com/ws".to_string(),
            api_key: None,
            bearer_token: None,
            sdk_token: None,
        }
    }
    
    pub fn with_sdk_token(mut self, token: String) -> Self {
        self.sdk_token = Some(token);
        self
    }
    
    pub fn with_api_key(mut self, key: String) -> Self {
        self.api_key = Some(key);
        self
    }
    
    pub fn with_bearer_token(mut self, token: String) -> Self {
        self.bearer_token = Some(token);
        self
    }
    
    pub fn with_base_url(mut self, url: String) -> Self {
        self.base_url = url;
        self
    }
}

/// WebSocket client
pub struct WebSocketClient {
    mode: Mode,
    config: WebSocketConfig,
    auth_state: Arc<Mutex<AuthenticationState>>,
    event_handlers: Arc<Mutex<Vec<Arc<dyn EventHandler>>>>,
    missed_pongs: Arc<Mutex<u32>>,
    sender: Option<mpsc::UnboundedSender<Message>>,
}

impl WebSocketClient {
    pub fn new(mode: Mode, sdk_token: String) -> Result<Self> {
        let config = WebSocketConfig::new().with_sdk_token(sdk_token);
        
        Ok(Self {
            mode,
            config,
            auth_state: Arc::new(Mutex::new(AuthenticationState::Pending)),
            event_handlers: Arc::new(Mutex::new(Vec::new())),
            missed_pongs: Arc::new(Mutex::new(0)),
            sender: None,
        })
    }
    
    /// Add event handler
    pub async fn add_event_handler(&self, handler: Arc<dyn EventHandler>) {
        let mut handlers = self.event_handlers.lock().await;
        handlers.push(handler);
    }
    
    /// Emit event to all handlers
    async fn emit_event(&self, event: WebSocketEvent) {
        let handlers = self.event_handlers.lock().await;
        for handler in handlers.iter() {
            handler.handle_event(event.clone());
        }
    }
    
    /// Send ping message
    pub async fn ping(&self, message: &str) -> Result<()> {
        let ping_msg = json!({
            "event": "ping",
            "data": {
                "state": message
            }
        });
        
        self.send_message(ping_msg).await
    }
    
    /// Subscribe to channel
    pub async fn subscribe(&self, params: HashMap<String, Value>) -> Result<()> {
        // Check if mode supports the channel
        if self.mode == Mode::Speed {
            if let Some(channel) = params.get("channel") {
                if let Some(channel_str) = channel.as_str() {
                    if channel_str == "aggregates" || channel_str == "candles" {
                        return Err(Error::InvalidModeForChannel {
                            channel: channel_str.to_string(),
                            mode: self.mode.as_str().to_string(),
                        });
                    }
                }
            }
        }
        
        let subscribe_msg = json!({
            "event": "subscribe",
            "data": params
        });
        
        self.send_message(subscribe_msg).await
    }
    
    /// Unsubscribe from channel
    pub async fn unsubscribe(&self, params: HashMap<String, Value>) -> Result<()> {
        let unsubscribe_msg = json!({
            "event": "unsubscribe",
            "data": params
        });
        
        self.send_message(unsubscribe_msg).await
    }
    
    /// Get current subscriptions
    pub async fn subscriptions(&self) -> Result<()> {
        let subscriptions_msg = json!({
            "event": "subscriptions"
        });
        
        self.send_message(subscriptions_msg).await
    }
    
    /// Send message to WebSocket
    async fn send_message(&self, message: Value) -> Result<()> {
        if let Some(ref sender) = self.sender {
            let text = serde_json::to_string(&message)?;
            sender.send(Message::Text(text))
                .map_err(|_| Error::websocket("Failed to send message"))?;
        } else {
            return Err(Error::websocket("WebSocket not connected"));
        }
        Ok(())
    }
    
    /// Handle authentication
    async fn handle_authentication(&self) -> Result<()> {
        let auth_info = if let Some(ref api_key) = self.config.api_key {
            json!({
                "event": "auth",
                "data": {
                    "apikey": api_key
                }
            })
        } else if let Some(ref bearer_token) = self.config.bearer_token {
            json!({
                "event": "auth",
                "data": {
                    "token": bearer_token
                }
            })
        } else if let Some(ref sdk_token) = self.config.sdk_token {
            json!({
                "event": "auth",
                "data": {
                    "sdkToken": sdk_token
                }
            })
        } else {
            *self.auth_state.lock().await = AuthenticationState::Unauthenticated;
            return Err(Error::MissingCredentials);
        };
        
        self.send_message(auth_info).await?;
        *self.auth_state.lock().await = AuthenticationState::Authenticating;
        
        Ok(())
    }
    
    /// Start ping timer
    async fn start_ping_timer(&self) {
        let sender = self.sender.clone();
        let missed_pongs = Arc::clone(&self.missed_pongs);
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(PING_INTERVAL));
            
            loop {
                interval.tick().await;
                
                if let Some(ref sender) = sender {
                    let ping_msg = json!({
                        "event": "ping",
                        "data": {
                            "state": ""
                        }
                    });
                    
                    let text = serde_json::to_string(&ping_msg).unwrap();
                    if sender.send(Message::Text(text)).is_err() {
                        break;
                    }
                    
                    // Increment missed pongs
                    let mut pongs = missed_pongs.lock().await;
                    *pongs += 1;
                    
                    if *pongs > MAX_MISSED_PONGS {
                        // Disconnect due to missed pongs
                        break;
                    }
                } else {
                    break;
                }
            }
        });
    }
    
    /// Connect to WebSocket
    pub async fn connect(&mut self) -> Result<()> {
        let url = Url::parse(&self.config.base_url)?;
        let (ws_stream, _) = connect_async(url).await
            .map_err(|e| Error::websocket(format!("Failed to connect: {}", e)))?;
        
        let (mut ws_sender, mut ws_receiver) = ws_stream.split();
        let (tx, mut rx) = mpsc::unbounded_channel();
        self.sender = Some(tx);
        
        // Emit connect event
        self.emit_event(WebSocketEvent::Connect).await;
        
        // Handle authentication
        self.handle_authentication().await?;
        
        // Start ping timer
        self.start_ping_timer().await;
        
        let auth_state = Arc::clone(&self.auth_state);
        let event_handlers = Arc::clone(&self.event_handlers);
        let missed_pongs = Arc::clone(&self.missed_pongs);
        
        // Sender task
        tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                if ws_sender.send(message).await.is_err() {
                    break;
                }
            }
        });
        
        // Receiver task
        tokio::spawn(async move {
            while let Some(message) = ws_receiver.next().await {
                match message {
                    Ok(Message::Text(text)) => {
                        // Emit message event
                        let handlers = event_handlers.lock().await;
                        for handler in handlers.iter() {
                            handler.handle_event(WebSocketEvent::Message(text.clone()));
                        }
                        
                        // Parse and handle specific events
                        if let Ok(msg) = serde_json::from_str::<Value>(&text) {
                            if let Some(event) = msg.get("event").and_then(|e| e.as_str()) {
                                match event {
                                    AUTHENTICATED_EVENT => {
                                        *auth_state.lock().await = AuthenticationState::Authenticated;
                                        for handler in handlers.iter() {
                                            handler.handle_event(WebSocketEvent::Authenticated(msg.clone()));
                                        }
                                    }
                                    ERROR_EVENT => {
                                        if let Some(data) = msg.get("data") {
                                            if let Some(message) = data.get("message").and_then(|m| m.as_str()) {
                                                if message == UNAUTHENTICATED_MESSAGE {
                                                    *auth_state.lock().await = AuthenticationState::Unauthenticated;
                                                    for handler in handlers.iter() {
                                                        handler.handle_event(WebSocketEvent::Unauthenticated(msg.clone()));
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    "pong" => {
                                        // Reset missed pongs counter
                                        *missed_pongs.lock().await = 0;
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    Ok(Message::Close(frame)) => {
                        let code = frame.as_ref().map(|f| f.code.into());
                        let reason = frame.as_ref()
                            .map(|f| f.reason.to_string())
                            .unwrap_or_default();
                        
                        for handler in event_handlers.lock().await.iter() {
                            handler.handle_event(WebSocketEvent::Disconnect { code, reason: reason.clone() });
                        }
                        break;
                    }
                    Err(e) => {
                        for handler in event_handlers.lock().await.iter() {
                            handler.handle_event(WebSocketEvent::Error(e.to_string()));
                        }
                        break;
                    }
                    _ => {}
                }
            }
        });
        
        // Wait for authentication with timeout
        let auth_timeout = timeout(
            Duration::from_secs(AUTH_TIMEOUT),
            async {
                loop {
                    let state = *self.auth_state.lock().await;
                    match state {
                        AuthenticationState::Authenticated => return Ok(()),
                        AuthenticationState::Unauthenticated => {
                            return Err(Error::Unauthenticated);
                        }
                        _ => {
                            sleep(Duration::from_millis(100)).await;
                        }
                    }
                }
            }
        ).await;
        
        match auth_timeout {
            Ok(result) => result,
            Err(_) => {
                *self.auth_state.lock().await = AuthenticationState::Unauthenticated;
                Err(Error::AuthenticationTimeout)
            }
        }
    }
    
    /// Disconnect from WebSocket
    pub async fn disconnect(&mut self) {
        if let Some(sender) = self.sender.take() {
            let _ = sender.send(Message::Close(None));
        }
        *self.auth_state.lock().await = AuthenticationState::Pending;
        *self.missed_pongs.lock().await = 0;
    }
    
    /// Get current authentication state
    pub async fn auth_state(&self) -> AuthenticationState {
        *self.auth_state.lock().await
    }
}