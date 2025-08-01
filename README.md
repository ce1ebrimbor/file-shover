# File Shover

A static web server implementation in Rust focused on RFC 2616 HTTP/1.1 compliance. Built for learning HTTP fundamentals and exploring Rust's networking capabilities.

**How to run:**
```bash
RUST_LOG=debug cargo run -- --root test-sites/simple-portfolio -p 7878
```

## Current Features

✅ **Multi-threaded**: Handles concurrent requests using Rayon thread pool  
✅ **Streaming**: Memory-efficient file serving with `BufReader<File>`  
✅ **Security**: Path traversal protection and input validation  
✅ **MIME Types**: Basic content type detection for common file types  
✅ **Error Handling**: Proper HTTP status codes (400, 404, 500)  
✅ **Logging**: Configurable logging with `env_logger`  

## Architecture

### Core Components

- **FileTree**: Safe file access within root directory with streaming readers
- **HTTP Message System**: RFC 2616 compliant request parsing and response generation
- **Thread Pool**: Concurrent request handling with configurable pool size
- **MIME Detection**: File extension-based content type identification

### Current HTTP Support

- **Methods**: GET (implicit), basic error responses
- **Status Codes**: 200, 400, 404, 500
- **Headers**: Content-Type, Content-Length, Server, Connection
- **Security**: Path traversal prevention, input sanitization

## RFC 2616 Compliance Roadmap

### 🎯 Priority 1: Core HTTP Methods (Required)
- [ ] **Explicit Method Handling**: Properly handle GET, HEAD, OPTIONS
- [ ] **405 Method Not Allowed**: Return correct status for unsupported methods
- [ ] **HEAD Support**: Same headers as GET but no response body
- [ ] **OPTIONS Support**: Return allowed methods and CORS headers

### 🎯 Priority 2: Required HTTP Headers (Section 14 RFC 2616)
- [ ] **Date Header**: RFC 2616 formatted timestamp on all responses
- [ ] **Last-Modified**: File modification time for caching
- [ ] **Accept-Ranges**: Indicate partial content support capability
- [ ] **ETag**: Entity tags for cache validation

### 🎯 Priority 3: Conditional Requests (Caching)
- [ ] **If-Modified-Since**: Return 304 Not Modified when appropriate
- [ ] **If-None-Match**: ETag-based conditional requests
- [ ] **Cache-Control**: Proper cache directives
- [ ] **Expires**: Cache expiration headers

### 🎯 Priority 4: Advanced Features
- [ ] **Range Requests**: HTTP/1.1 partial content (206 responses)
- [ ] **Content-Encoding**: Gzip compression for text files
- [ ] **Directory Index**: Serve index.html for directory requests
- [ ] **Persistent Connections**: Keep-Alive support

## Service & Connection Improvements

### Connection Handling
- [ ] **HTTP Keep-Alive**: Reuse connections for multiple requests
- [ ] **Connection Timeouts**: Configurable read/write timeouts
- [ ] **Graceful Shutdown**: Clean connection termination on SIGTERM
- [ ] **Connection Limits**: Max concurrent connections per client

### Performance Enhancements
- [ ] **File Caching**: In-memory cache for frequently accessed files
- [ ] **Static Compression**: Pre-compressed gzip files (.gz)
- [ ] **Async I/O**: Consider tokio for higher concurrency
- [ ] **Zero-Copy**: Investigate sendfile() for large file transfers

### Operational Features
- [ ] **Configuration File**: YAML/TOML config instead of CLI only
- [ ] **Access Logging**: Common Log Format (CLF) support
- [ ] **Metrics**: Prometheus metrics endpoint
- [ ] **Health Checks**: `/health` endpoint for monitoring
- [ ] **Hot Reload**: Reload configuration without restart

### Security Enhancements
- [ ] **HTTPS Support**: TLS/SSL with rustls
- [ ] **Rate Limiting**: Per-IP request throttling
- [ ] **Security Headers**: HSTS, X-Frame-Options, CSP
- [ ] **IP Filtering**: Allow/deny lists for client IPs

## Implementation Examples

### RFC 2616 Method Handling
```rust
match req.method {
    HttpMethod::GET => serve_file(&req.path),
    HttpMethod::HEAD => serve_headers_only(&req.path),
    HttpMethod::OPTIONS => serve_options(&req.path),
    _ => Response::new().status(HttpStatus::MethodNotAllowed)
        .header("Allow", "GET, HEAD, OPTIONS")
}
```

### Connection Keep-Alive
```rust
// Check Connection header and HTTP version
let keep_alive = req.headers.get("Connection")
    .map(|h| h.to_lowercase() == "keep-alive")
    .unwrap_or(req.http_version == "HTTP/1.1");

response.header("Connection", if keep_alive { "keep-alive" } else { "close" })
```

### Conditional Requests
```rust
// Check If-Modified-Since header
if let Some(since) = req.headers.get("If-Modified-Since") {
    if !file_modified_since(since, &metadata) {
        return Response::new().status(HttpStatus::NotModified)
            .header("Last-Modified", &format_http_date(metadata.modified()?));
    }
}
```

## Development Philosophy

1. **RFC Compliance First**: Implement HTTP/1.1 specification correctly
2. **Security by Design**: Path traversal protection, input validation
3. **Performance Awareness**: Stream files, avoid unnecessary copies
4. **Observability**: Comprehensive logging and metrics
5. **Operational Readiness**: Configuration, monitoring, graceful shutdown

## Dependencies to Add

```toml
[dependencies]
httpdate = "1.0"        # RFC 2616 date formatting
flate2 = "1.0"          # Gzip compression
rustls = "0.23"         # TLS support (optional)
tokio = "1.0"           # Async I/O (future consideration)
```