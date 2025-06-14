use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;
use crate::{Result, Error};

/// Configuration for REST client
#[derive(Debug, Clone)]
pub struct RestConfig {
    pub base_url: String,
    pub api_key: Option<String>,
    pub bearer_token: Option<String>,
    pub sdk_token: Option<String>,
}

impl RestConfig {
    pub fn new() -> Self {
        Self {
            base_url: String::new(),
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
    
    pub fn validate(&self) -> Result<()> {
        let token_count = [&self.api_key, &self.bearer_token, &self.sdk_token]
            .iter()
            .filter(|token| token.is_some())
            .count();
            
        if token_count == 0 {
            return Err(Error::MissingCredentials);
        }
        
        if token_count > 1 {
            return Err(Error::general("Only one of the \"apiKey\", \"bearerToken\", or \"sdkToken\" options must be specified"));
        }
        
        Ok(())
    }
}

/// Stock intraday data client
pub struct Intraday {
    client: Client,
    config: RestConfig,
}

impl Intraday {
    pub fn new(config: RestConfig) -> Result<Self> {
        config.validate()?;
        Ok(Self {
            client: Client::new(),
            config,
        })
    }
    
    pub async fn get_data(&self, symbol: &str) -> Result<Value> {
        let url = format!("{}/intraday/{}", self.config.base_url, symbol);
        let mut headers = reqwest::header::HeaderMap::new();
        
        if let Some(ref api_key) = self.config.api_key {
            headers.insert("X-API-Key", api_key.parse().unwrap());
        } else if let Some(ref bearer_token) = self.config.bearer_token {
            headers.insert("Authorization", format!("Bearer {}", bearer_token).parse().unwrap());
        } else if let Some(ref sdk_token) = self.config.sdk_token {
            headers.insert("X-SDK-Token", sdk_token.parse().unwrap());
        }
        
        let response = self.client
            .get(&url)
            .headers(headers)
            .send()
            .await?;
            
        let json = response.json::<Value>().await?;
        Ok(json)
    }
}

/// Stock historical data client
pub struct Historical {
    client: Client,
    config: RestConfig,
}

impl Historical {
    pub fn new(config: RestConfig) -> Result<Self> {
        config.validate()?;
        Ok(Self {
            client: Client::new(),
            config,
        })
    }
    
    pub async fn get_data(&self, symbol: &str, from: &str, to: &str) -> Result<Value> {
        let url = format!("{}/historical/{}", self.config.base_url, symbol);
        let mut params = HashMap::new();
        params.insert("from", from);
        params.insert("to", to);
        
        let mut headers = reqwest::header::HeaderMap::new();
        
        if let Some(ref api_key) = self.config.api_key {
            headers.insert("X-API-Key", api_key.parse().unwrap());
        } else if let Some(ref bearer_token) = self.config.bearer_token {
            headers.insert("Authorization", format!("Bearer {}", bearer_token).parse().unwrap());
        } else if let Some(ref sdk_token) = self.config.sdk_token {
            headers.insert("X-SDK-Token", sdk_token.parse().unwrap());
        }
        
        let response = self.client
            .get(&url)
            .headers(headers)
            .query(&params)
            .send()
            .await?;
            
        let json = response.json::<Value>().await?;
        Ok(json)
    }
}

/// Stock snapshot data client
pub struct Snapshot {
    client: Client,
    config: RestConfig,
}

impl Snapshot {
    pub fn new(config: RestConfig) -> Result<Self> {
        config.validate()?;
        Ok(Self {
            client: Client::new(),
            config,
        })
    }
    
    pub async fn get_data(&self, symbol: &str) -> Result<Value> {
        let url = format!("{}/snapshot/{}", self.config.base_url, symbol);
        let mut headers = reqwest::header::HeaderMap::new();
        
        if let Some(ref api_key) = self.config.api_key {
            headers.insert("X-API-Key", api_key.parse().unwrap());
        } else if let Some(ref bearer_token) = self.config.bearer_token {
            headers.insert("Authorization", format!("Bearer {}", bearer_token).parse().unwrap());
        } else if let Some(ref sdk_token) = self.config.sdk_token {
            headers.insert("X-SDK-Token", sdk_token.parse().unwrap());
        }
        
        let response = self.client
            .get(&url)
            .headers(headers)
            .send()
            .await?;
            
        let json = response.json::<Value>().await?;
        Ok(json)
    }
}

/// Stock REST client
pub struct RestStockClient {
    config: RestConfig,
}

impl RestStockClient {
    pub fn new(config: RestConfig) -> Result<Self> {
        config.validate()?;
        Ok(Self { config })
    }
    
    pub fn intraday(&self) -> Result<Intraday> {
        Intraday::new(self.config.clone())
    }
    
    pub fn historical(&self) -> Result<Historical> {
        Historical::new(self.config.clone())
    }
    
    pub fn snapshot(&self) -> Result<Snapshot> {
        Snapshot::new(self.config.clone())
    }
}

/// Main REST client
pub struct RestClient {
    sdk_token: String,
}

impl RestClient {
    pub fn new(sdk_token: String) -> Result<Self> {
        Ok(Self { sdk_token })
    }
    
    pub fn stock(&self) -> Result<RestStockClient> {
        let config = RestConfig::new()
            .with_sdk_token(self.sdk_token.clone())
            .with_base_url("https://api.fubon.com".to_string());
            
        RestStockClient::new(config)
    }
}