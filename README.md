# RustHTTP-API-Client - Async HTTP Client Library 🦀

A modern, feature-rich HTTP client library built with Rust, designed for simplicity and performance.

## ✨ Features

- **Async/Await Support** - Built on `tokio` and `reqwest` for high-performance async operations
- **Full HTTP Methods** - GET, POST, PUT, DELETE support with more coming
- **JSON Handling** - Automatic JSON parsing and pretty-printing
- **Authentication** - Bearer token and Basic auth support
- **Flexible Configuration** - Customizable headers, timeouts, and SSL settings
- **Response Analysis** - Built-in response time tracking and content type detection
- **Colorized Output** - Beautiful colored JSON output for better readability
- **Error Handling** - Comprehensive error handling with detailed context

## 🚀 Quick Start

### Basic Usage

```rust
use RustHTTP::{HttpClient, RequestConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = HttpClient::new();
    let config = RequestConfig::new();
    
    // Simple GET request
    let response = client.get("https://api.example.com/data", config).await?;
    
    if response.is_success() {
        println!("Status: {}", response.status);
        println!("Body: {}", response.body);
    }
    
    Ok(())
}
```

## 📖 API Documentation

### HttpClient

The main client for making HTTP requests.

#### Methods

- `new()` - Create a new client with default settings
- `with_timeout(seconds)` - Create a client with custom timeout
- `get(url, config)` - Send GET request
- `post(url, data, config)` - Send POST request with JSON data
- `put(url, data, config)` - Send PUT request with JSON data
- `delete(url, config)` - Send DELETE request

### RequestConfig

Configuration object for customizing requests.

```rust
let config = RequestConfig::new()
    .with_bearer_token("your-token")
    .add_header("Accept", "application/json")
    .with_pretty_print(true);
```

#### Methods

- `with_headers(HashMap)` - Set multiple headers at once
- `add_header(key, value)` - Add a single header
- `with_bearer_token(token)` - Add Bearer authentication
- `with_basic_auth(username, password)` - Add Basic authentication
- `with_pretty_print(bool)` - Enable/disable pretty printing
- `with_redirects(bool)` - Enable/disable following redirects
- `with_ssl_verification(bool)` - Enable/disable SSL verification

### ApiResponse

Response object containing all response data.

#### Properties

- `status: u16` - HTTP status code
- `status_text: String` - Status text description
- `headers: HashMap<String, String>` - Response headers
- `body: String` - Response body
- `content_type: String` - Content type header value
- `response_time_ms: u64` - Response time in milliseconds

#### Methods

- `is_success()` - Check if status is 2xx
- `is_json()` - Check if response is JSON
- `parse_json<T>()` - Parse JSON into custom type
- `as_json_value()` - Parse as serde_json::Value

## 🔥 Examples

### GET Request with Headers

```rust
use std::collections::HashMap;

let mut headers = HashMap::new();
headers.insert("User-Agent".to_string(), "MyApp/1.0".to_string());
headers.insert("Accept".to_string(), "application/json".to_string());

let config = RequestConfig::new().with_headers(headers);
let response = client.get("https://api.github.com/users/octocat", config).await?;

println!("User data: {}", response.body);
```

### POST Request with JSON

```rust
let post_data = r#"{
    "title": "watan",
    "body": "i love cats",
    "userId": 1
}"#;

let config = RequestConfig::new()
    .add_header("Content-Type", "application/json");

let response = client.post("https://jsonplaceholder.typicode.com/posts", post_data, config).await?;

if response.is_success() {
    println!("Post created successfully!");
    if let Ok(json) = response.as_json_value() {
        println!("New post ID: {}", json["id"]);
    }
}
```

### Authenticated Request

```rust
let config = RequestConfig::new()
    .with_bearer_token("your-api-token")
    .add_header("Accept", "application/json");

let response = client.get("https://api.example.com/protected", config).await?;
```

### Custom Timeout

```rust
// Create client with 10 second timeout
let client = HttpClient::with_timeout(10)?;
let config = RequestConfig::new();

let response = client.get("https://slow-api.example.com/data", config).await?;
```

### Response Analysis

```rust
let response = client.get("https://api.example.com/data", config).await?;

println!("Status: {} {}", response.status, response.status_text);
println!("Response time: {}ms", response.response_time_ms);
println!("Content type: {}", response.content_type);
println!("Headers count: {}", response.headers.len());

// Check specific headers
if let Some(rate_limit) = response.headers.get("x-ratelimit-remaining") {
    println!("Rate limit remaining: {}", rate_limit);
}
```

## 🛠️ Utility Functions

The library includes several utility functions for common tasks:

```rust
use RustHTTP::{pretty_print_json, format_duration, status_indicator, is_valid_json};

// Pretty print JSON with colors
let formatted = pretty_print_json(&json_string)?;
println!("{}", formatted);

// Format response time
let duration = format_duration(1500); // "1.50s"

// Get colored status indicator
let indicator = status_indicator(200); // Green "200"

// Validate JSON
if is_valid_json(&data) {
    println!("Valid JSON!");
}
```

## 🧪 Running the Demo

Clone the repository and run the demo:

```bash
git clone https://github.com/yourusername/rust-http-client.git
cd rust-http-client
cargo run
```

The demo will show examples of:
- GET requests with JSON parsing
- POST requests with custom data
- Authenticated requests
- PUT and DELETE operations
- Custom timeout handling

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

### Development Setup

1. Clone the repository
2. Install Rust (if not already installed)
3. Run `cargo build` to build the project
4. Run `cargo test` to run tests
5. Run `cargo run` to see the demo

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🌟 Acknowledgments

- Built with [reqwest](https://github.com/seanmonstar/reqwest) for HTTP client functionality
- Uses [tokio](https://tokio.rs/) for async runtime
- JSON handling powered by [serde](https://serde.rs/)
- Colorized output using [colored](https://github.com/mackwic/colored)

## 📊 Project Structure

```
src/
├── lib.rs          # Library exports and module declarations
├── client.rs       # Main HTTP client implementation
├── models.rs       # Data structures and models
├── utils.rs        # Utility functions and helpers
└── main.rs         # Demo application
```
