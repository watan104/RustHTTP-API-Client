use anyhow::Result;
use RustHTTP::{HttpClient, RequestConfig, pretty_print_json, format_duration, status_indicator};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<()> {
    println!("RustHTTP API Client Demo");
    println!("{}", "=".repeat(40));

    let client = HttpClient::new();

    println!("\nGET Request Demo");
    let config = RequestConfig::new();
    
    match client.get("https://jsonplaceholder.typicode.com/posts/1", config).await {
        Ok(response) => {
            println!("Status: {} {}", 
                status_indicator(response.status), 
                response.status_text
            );
            println!("Response Time: {}", format_duration(response.response_time_ms));
            
            if response.is_json() {
                match pretty_print_json(&response.body) {
                    Ok(pretty_json) => println!("Response:\n{}", pretty_json),
                    Err(_) => println!("Response: {}", response.body),
                }
            } else {
                println!("Response: {}", response.body);
            }
        }
        Err(e) => println!("❌ Error: {}", e),
    }

    println!("\nPOST Request Demo");
    let post_data = r#"{
        "title": "watan love cats",
        "body": "i love cats",
        "userId": 1
    }"#;

    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "application/json".to_string());
    headers.insert("User-Agent".to_string(), "RustHTTP-Client/0.1.0".to_string());
    
    let config = RequestConfig::new().with_headers(headers);

    match client.post("https://jsonplaceholder.typicode.com/posts", post_data, config).await {
        Ok(response) => {
            println!("Status: {} {}", 
                status_indicator(response.status), 
                response.status_text
            );
            
            if response.is_success() {
                println!("POST Done!");
                if let Ok(json_value) = response.as_json_value() {
                    if let Some(id) = json_value.get("id") {
                        println!("New POST ID: {}", id);
                    }
                }
            }
        }
        Err(e) => println!("POST Error: {}", e),
    }

    println!("\nAuthenticated Request Demo");
    let config = RequestConfig::new()
        .with_bearer_token("test-token-12345")
        .add_header("Accept", "application/json");

    match client.get("https://jsonplaceholder.typicode.com/users", config).await {
        Ok(response) => {
            println!("Status: {} {}", 
                status_indicator(response.status), 
                response.status_text
            );
            println!("Headers count: {}", response.headers.len());
            
            if let Some(content_type) = response.headers.get("content-type") {
                println!("Content-Type: {}", content_type);
            }
        }
        Err(e) => println!("Error: {}", e),
    }

    println!("\nPUT Request Demo");
    let put_data = r#"{"id": 1, "name": "cat", "email": "cat@andrewatan.com"}"#;
    let config = RequestConfig::new();

    match client.put("https://jsonplaceholder.typicode.com/users/1", put_data, config).await {
        Ok(response) => {
            println!("Status: {} {}", 
                status_indicator(response.status), 
                response.status_text
            );
            println!("PUT Req Done");
        }
        Err(e) => println!("PUT Error: {}", e),
    }

    println!("\nDELETE Request Demo");
    let config = RequestConfig::new();

    match client.delete("https://jsonplaceholder.typicode.com/posts/1", config).await {
        Ok(response) => {
            println!("Status: {} {}", 
                status_indicator(response.status), 
                response.status_text
            );
            println!("DELETE Req Done");
        }
        Err(e) => println!("DELETE Error: {}", e),
    }

    println!("\nCustom Timeout Demo");
    match HttpClient::with_timeout(5) {
        Ok(timeout_client) => {
            let config = RequestConfig::new();
            match timeout_client.get("https://jsonplaceholder.typicode.com/posts", config).await {
                Ok(response) => {
                    println!("Timeout Req Done with 5 Seconds");
                    println!("Status: {}", status_indicator(response.status));
                }
                Err(e) => println!("Timeout Error: {}", e),
            }
        }
        Err(e) => println!("Client creation error: {}", e),
    }

    println!("\nAll Demos Work!");
    Ok(())
}