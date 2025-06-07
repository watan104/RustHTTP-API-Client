use anyhow::{Context, Result};
use reqwest::{Client, Response};
use serde_json::Value;
use std::collections::HashMap;
use std::time::Instant;

use crate::models::{ApiResponse, RequestConfig};

pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    pub fn new() -> Self {
        let client = Client::builder()
            .user_agent("RustHttpClient/0.1.0")
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");
        
        Self { client }
    }

    pub async fn get(&self, url: &str, config: RequestConfig) -> Result<ApiResponse> {
        let start_time = Instant::now();
        
        let mut request = self.client.get(url);
        
        for (key, value) in &config.headers {
            request = request.header(key, value);
        }
        
        let response = request
            .send()
            .await
            .with_context(|| format!("Failed to send GET request to {}", url))?;
        
        self.process_response(response, start_time).await
    }

    pub async fn post(&self, url: &str, data: &str, config: RequestConfig) -> Result<ApiResponse> {
        let start_time = Instant::now();
        
        let json_value: Value = serde_json::from_str(data)
            .with_context(|| "Invalid JSON data provided")?;
        
        let mut request = self.client
            .post(url)
            .json(&json_value);
        
        for (key, value) in &config.headers {
            request = request.header(key, value);
        }
        
        let response = request
            .send()
            .await
            .with_context(|| format!("Failed to send POST request to {}", url))?;
        
        self.process_response(response, start_time).await
    }

    pub async fn put(&self, url: &str, data: &str, config: RequestConfig) -> Result<ApiResponse> {
        let start_time = Instant::now();
        
        let json_value: Value = serde_json::from_str(data)
            .with_context(|| "Invalid JSON data provided")?;
        
        let mut request = self.client
            .put(url)
            .json(&json_value);
        
        for (key, value) in &config.headers {
            request = request.header(key, value);
        }
        
        let response = request
            .send()
            .await
            .with_context(|| format!("Failed to send PUT request to {}", url))?;
        
        self.process_response(response, start_time).await
    }

    pub async fn delete(&self, url: &str, config: RequestConfig) -> Result<ApiResponse> {
        let start_time = Instant::now();
        
        let mut request = self.client.delete(url);
        
        for (key, value) in &config.headers {
            request = request.header(key, value);
        }
        
        let response = request
            .send()
            .await
            .with_context(|| format!("Failed to send DELETE request to {}", url))?;
        
        self.process_response(response, start_time).await
    }

    async fn process_response(&self, response: Response, start_time: Instant) -> Result<ApiResponse> {
        let status = response.status().as_u16();
        let status_text = response.status().canonical_reason().unwrap_or("Unknown").to_string();
        
        let mut headers = HashMap::new();
        for (key, value) in response.headers() {
            if let Ok(value_str) = value.to_str() {
                headers.insert(key.to_string(), value_str.to_string());
            }
        }
        
        let content_type = response
            .headers()
            .get("content-type")
            .and_then(|ct| ct.to_str().ok())
            .unwrap_or("text/plain")
            .to_string();
        
        let body = response
            .text()
            .await
            .with_context(|| "Failed to read response body")?;
        
        let response_time_ms = start_time.elapsed().as_millis() as u64;
        
        Ok(ApiResponse {
            status,
            status_text,
            headers,
            body,
            content_type,
            response_time_ms,
        })
    }

    pub fn validate_url(url: &str) -> Result<()> {
        url::Url::parse(url)
            .with_context(|| format!("Invalid URL format: {}", url))?;
        Ok(())
    }

    pub fn with_timeout(timeout_secs: u64) -> Result<Self> {
        let client = Client::builder()
            .user_agent("RustHttpClient/0.1.0")
            .timeout(std::time::Duration::from_secs(timeout_secs))
            .build()
            .with_context(|| "Failed to create HTTP client with custom timeout")?;
        
        Ok(Self { client })
    }
}