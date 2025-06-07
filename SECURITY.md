# Security Policy 🔒

## Supported Versions

We take security seriously and provide security updates for the following versions:

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |
| < 0.1   | :x:                |

## Security Best Practices

### 🔐 Authentication & Authorization

#### Bearer Token Security
```rust
// ✅ SECURE: Use environment variables for tokens
let token = std::env::var("API_TOKEN").expect("API_TOKEN not set");
let config = RequestConfig::new().with_bearer_token(token);

// ❌ INSECURE: Never hardcode tokens
let config = RequestConfig::new().with_bearer_token("hardcoded-token-123");
```

#### Basic Authentication
```rust
// ✅ SECURE: Load credentials from secure storage
let username = std::env::var("API_USERNAME").expect("Username not set");
let password = std::env::var("API_PASSWORD").expect("Password not set");
let config = RequestConfig::new().with_basic_auth(username, password);

// ❌ INSECURE: Avoid hardcoded credentials
let config = RequestConfig::new().with_basic_auth("admin", "password123");
```

### 🔗 URL Validation

The library includes built-in URL validation to prevent injection attacks:

```rust
// URL validation is automatically performed
match HttpClient::validate_url("https://api.example.com/data") {
    Ok(()) => println!("Valid URL"),
    Err(e) => println!("Invalid URL: {}", e),
}

// ❌ These URLs will be rejected:
// - "javascript:alert('xss')"
// - "file:///etc/passwd"
// - "ftp://malicious.com/payload"
```

### 🛡️ SSL/TLS Security

#### Default Secure Configuration
```rust
// ✅ SSL verification is enabled by default
let config = RequestConfig::new(); // SSL verification: ON

// ⚠️ Only disable SSL verification for development/testing
let config = RequestConfig::new().with_ssl_verification(false);
```

#### Certificate Validation
- All HTTPS requests validate server certificates by default
- Certificate chain validation is performed
- Hostname verification is enforced
- Only disable SSL verification in controlled environments

### 🔒 Request Security

#### Header Injection Prevention
```rust
// ✅ SECURE: Headers are properly escaped
let config = RequestConfig::new()
    .add_header("User-Agent", "MyApp/1.0")
    .add_header("Accept", "application/json");

// ❌ AVOID: Untrusted user input in headers
// Always validate and sanitize user input before adding to headers
```

#### Request Size Limits
```rust
// ✅ Use reasonable timeouts to prevent resource exhaustion
let client = HttpClient::with_timeout(30)?; // 30 seconds max

// ⚠️ Very long timeouts can lead to resource exhaustion
let client = HttpClient::with_timeout(3600)?; // 1 hour - potentially dangerous
```

### 📝 JSON Security

#### Safe JSON Parsing
```rust
// ✅ SECURE: Built-in protection against malicious JSON
match response.parse_json::<MyStruct>() {
    Ok(data) => {
        // Process safely parsed data
        println!("Parsed: {:?}", data);
    }
    Err(e) => {
        // Handle parsing errors gracefully
        eprintln!("JSON parsing failed: {}", e);
    }
}
```

#### JSON Injection Prevention
- All JSON data is properly escaped during serialization
- Input validation is performed before JSON parsing
- Memory limits prevent JSON bomb attacks

### 🌐 Network Security

#### Connection Security
- Connections use secure protocols (HTTPS/TLS)
- Connection pooling with reasonable limits
- Automatic retry logic with exponential backoff
- DNS resolution security

#### Proxy Configuration
```rust
// If using behind corporate proxy, ensure proper configuration
// The library respects system proxy settings automatically
```

## 🚨 Vulnerability Reporting

### Reporting Security Issues

If you discover a security vulnerability, please follow these steps:

1. **DO NOT** create a public GitHub issue
2. Send an email to: `security@andrewatan.com`
3. Include the following information:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if any)

### Response Timeline

- **Initial Response**: Within 48 hours
- **Confirmation**: Within 72 hours
- **Fix Development**: 1-2 weeks (depending on severity)
- **Public Disclosure**: After fix is released

### Security Contact

- **Email**: security@andrewatan.com
- **PGP Key**: Available upon request
- **Preferred Language**: English

## 🔍 Security Auditing

### Regular Security Practices

1. **Dependency Scanning**
   ```bash
   cargo audit
   ```

2. **Static Analysis**
   ```bash
   cargo clippy -- -D warnings
   ```

3. **Memory Safety**
   ```bash
   cargo test --release
   ```

### Third-Party Dependencies

We regularly audit our dependencies for security vulnerabilities:

| Dependency | Purpose | Last Audit |
|------------|---------|------------|
| reqwest    | HTTP client | 2024-01 |
| tokio      | Async runtime | 2024-01 |
| serde      | Serialization | 2024-01 |
| anyhow     | Error handling | 2024-01 |

## ⚠️ Known Security Considerations

### Memory Usage
- Large response bodies are loaded into memory
- Consider streaming for very large responses
- Set appropriate timeout values

### Error Information
- Error messages may contain sensitive information
- Review error handling in production code
- Avoid logging sensitive data

### Logging Security
```rust
// ✅ SECURE: Don't log sensitive data
println!("Request to: {}", sanitize_url(&url));

// ❌ INSECURE: Logging authentication headers
println!("Headers: {:?}", request.headers()); // May contain tokens
```

## 🛠️ Secure Configuration Guide

### Production Configuration
```rust
use std::time::Duration;

// Recommended production settings
let client = HttpClient::new()
    .with_timeout(30)? // 30 second timeout
    .with_ssl_verification(true) // Always verify SSL
    .with_redirects(true); // Follow redirects securely

let config = RequestConfig::new()
    .with_bearer_token(env::var("API_TOKEN")?)
    .add_header("User-Agent", "YourApp/1.0.0")
    .add_header("Accept", "application/json");
```

### Development Configuration
```rust
// Development settings (less restrictive)
let client = HttpClient::new()
    .with_timeout(60)? // Longer timeout for debugging
    .with_ssl_verification(false); // Only for local testing

// ⚠️ Never use development config in production!
```

## 📋 Security Checklist

Before deploying to production, ensure:

- [ ] All API tokens are stored in environment variables
- [ ] SSL verification is enabled
- [ ] Reasonable timeout values are set
- [ ] Input validation is implemented
- [ ] Error handling doesn't leak sensitive information
- [ ] Logging doesn't include sensitive data
- [ ] Dependencies are up-to-date and audited
- [ ] Network policies are configured appropriately

## 🔄 Security Updates

### Automatic Updates
We recommend using `cargo update` regularly to get security patches:

```bash
# Update all dependencies
cargo update

# Check for security advisories
cargo audit
```

### Manual Review
Review the changelog for security-related updates:
- Authentication improvements
- SSL/TLS enhancements
- Input validation fixes
- Error handling improvements

## 📚 Additional Resources

### External Security Resources
- [OWASP API Security Top 10](https://owasp.org/www-project-api-security/)
- [Rust Security Guidelines](https://anssi-fr.github.io/rust-guide/)
- [Secure Coding in Rust](https://doc.rust-lang.org/nomicon/)

### Rust Security Tools
- `cargo audit` - Vulnerability scanner
- `cargo clippy` - Linter with security checks
- `cargo deny` - Dependency policy enforcement

---

## 📄 Disclosure Policy

We follow responsible disclosure practices:

1. **Private Reporting**: Security issues reported privately first
2. **Coordinated Disclosure**: Work with reporter to understand impact
3. **Fix Development**: Develop and test security patches
4. **Public Disclosure**: Announce fix with appropriate details
5. **Recognition**: Credit security researchers (with permission)

---

**Last Updated**: January 2025  

For questions about this security policy, contact: security@andrewatan.com
