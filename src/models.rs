use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse {
    pub status: u16,
    pub status_text: String,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub content_type: String,
    pub response_time_ms: u64,
}

impl ApiResponse {
    pub fn is_success(&self) -> bool {
        (200..300).contains(&self.status)
    }

    pub fn is_json(&self) -> bool {
        self.content_type.contains("application/json")
    }

    pub fn parse_json<T>(&self) -> anyhow::Result<T>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        serde_json::from_str(&self.body)
            .map_err(|e| anyhow::anyhow!("Failed to parse JSON: {}", e))
    }

    pub fn as_json_value(&self) -> anyhow::Result<serde_json::Value> {
        serde_json::from_str(&self.body)
            .map_err(|e| anyhow::anyhow!("Failed to parse JSON: {}", e))
    }
}

#[derive(Debug, Clone, Default)]
pub struct RequestConfig {
    pub headers: HashMap<String, String>,
    pub pretty_print: bool,
    pub follow_redirects: bool,
    pub verify_ssl: bool,
}

impl RequestConfig {
    pub fn new() -> Self {
        Self {
            headers: HashMap::new(),
            pretty_print: false,
            follow_redirects: true,
            verify_ssl: true,
        }
    }

    pub fn with_headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers = headers;
        self
    }

    pub fn with_pretty_print(mut self, pretty: bool) -> Self {
        self.pretty_print = pretty;
        self
    }

    pub fn with_redirects(mut self, follow: bool) -> Self {
        self.follow_redirects = follow;
        self
    }

    pub fn with_ssl_verification(mut self, verify: bool) -> Self {
        self.verify_ssl = verify;
        self
    }

    pub fn add_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }

    pub fn with_bearer_token(self, token: impl Into<String>) -> Self {
        self.add_header("Authorization", format!("Bearer {}", token.into()))
    }

    pub fn with_basic_auth(self, username: impl Into<String>, password: impl Into<String>) -> Self {
        let credentials = format!("{}:{}", username.into(), password.into());
        let encoded = base64_encode(&credentials);
        self.add_header("Authorization", format!("Basic {}", encoded))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub code: Option<String>,
    pub message: String,
    pub details: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Head,
    Options,
}

impl std::fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpMethod::Get => write!(f, "GET"),
            HttpMethod::Post => write!(f, "POST"),
            HttpMethod::Put => write!(f, "PUT"),
            HttpMethod::Delete => write!(f, "DELETE"),
            HttpMethod::Patch => write!(f, "PATCH"),
            HttpMethod::Head => write!(f, "HEAD"),
            HttpMethod::Options => write!(f, "OPTIONS"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RequestStats {
    pub method: HttpMethod,
    pub url: String,
    pub status_code: u16,
    pub response_time_ms: u64,
    pub response_size_bytes: usize,
    pub timestamp: std::time::SystemTime,
}

fn base64_encode(input: &str) -> String {
    let mut result = String::new();
    let bytes = input.as_bytes();
    
    for chunk in bytes.chunks(3) {
        let mut buf = [0u8; 3];
        for (i, &byte) in chunk.iter().enumerate() {
            buf[i] = byte;
        }
        
        let b64_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let chars: Vec<char> = b64_chars.chars().collect();
        
        let n = (buf[0] as u32) << 16 | (buf[1] as u32) << 8 | (buf[2] as u32);
        
        result.push(chars[((n >> 18) & 63) as usize]);
        result.push(chars[((n >> 12) & 63) as usize]);
        result.push(if chunk.len() > 1 { chars[((n >> 6) & 63) as usize] } else { '=' });
        result.push(if chunk.len() > 2 { chars[(n & 63) as usize] } else { '=' });
    }
    
    result
}