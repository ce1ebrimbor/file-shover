# File Shover

Painfully naive implementation of a static web server in Rust. Built for learning and experimentation.

**How to run:**
```
RUST_LOG=debug cargo run -- --root test-sites/simple-portfolio -p 7878
```

## Architecture

### Current Design

- **FileTree**: Provides streaming file access via `get_reader()` - returns `BufReader<File>` for memory-efficient file reading
- **HTTP Message Handling**: Request parsing and response generation with convenience methods for`common HTTP operations
- **Response System**: Currently stores entire response body in memory as `String` or `Vec<u8>`

### Known Limitations

#### 🚧 Streaming Response Mismatch

**Issue**: Architectural mismatch between streaming file reading and in-memory response bodies.

- **FileTree** is designed for streaming: `get_reader()` returns a reader for memory-efficient file access

**Impact**: 
- Cannot efficiently serve large files (videos, images, archives)
- Memory usage scales with file size instead of remaining constant
- Loses the streaming benefits of the FileTree design

**Current Workaround**: 

## Future Improvements

### Phase 1: Core Streaming
- [ ] **Add Multi-threading**
- [x] Implement streaming response bodies
- [x] Add binary file support

### Phase 2: Performance
- [ ] Add Content-Length header calculation
- [ ] Implement chunked transfer encoding
- [ ] Add compression support (gzip, brotli)

### Phase 3: Features
- [ ] HTTP caching headers (ETag, Last-Modified)
- [ ] Range requests for partial content
- [x] MIME type detection
    - Basic MimeType
- [ ] Directory listing

## Development Notes

- Current focus is on getting basic HTTP functionality working
- Streaming optimizations are deliberately deferred to avoid premature optimization