# File Shover

Painfully naive implementation of a static web server in Rust. Built for learning and experimentation.

## Architecture

### Current Design

- **FileTree**: Provides streaming file access via `get_reader()` - returns `BufReader<File>` for memory-efficient file reading
- **HTTP Message Handling**: Request parsing and response generation with convenience methods for`common HTTP operations
- **Response System**: Currently stores entire response body in memory as `String` or `Vec<u8>`

### Known Limitations

#### 🚧 Streaming Response Mismatch

**Issue**: Architectural mismatch between streaming file reading and in-memory response bodies.

- **FileTree** is designed for streaming: `get_reader()` returns a reader for memory-efficient file access
- **Response** is designed for in-memory: `body: Option<String>` forces entire content into memory

**Impact**: 
- Cannot efficiently serve large files (videos, images, archives)
- Memory usage scales with file size instead of remaining constant
- Loses the streaming benefits of the FileTree design

**Current Workaround**: 
```rust
// This loads entire file into memory - not optimal for large files
let mut reader = tree.get_reader("large_file.mp4")?;
let mut content = String::new();
reader.read_to_string(&mut content)?;
response.body(content)
```

**Future Solutions**:
1. **Enum-based ResponseBody**: Support both `String` and `Box<dyn Read>` bodies
2. **Generic Response**: `Response<R>` where R can be String or Reader
3. **Separate Streaming Method**: Add `write_with_reader()` method to Response


## Future Improvements

### Phase 1: Core Streaming
- [ ] **Add Multi-threading**
- [ ] Implement streaming response bodies
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
- The FileTree reader-based design is correct - Response needs to catch up 