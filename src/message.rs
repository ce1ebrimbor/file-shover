use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};

/// Errors that can occur when parsing HTTP requests.
///
/// This enum covers various failure modes that can happen during request parsing,
/// including I/O errors, malformed request format, and missing required headers.
///
/// # Examples
///
/// ```
/// use file_shover::message::RequestError;
/// use std::io;
///
/// // Creating different types of errors
/// let io_error = RequestError::Io(io::Error::new(io::ErrorKind::UnexpectedEof, "connection closed"));
/// let format_error = RequestError::InvalidFormat;
/// let missing_header = RequestError::MissingHeader("Content-Length".to_string());
///
/// println!("IO Error: {}", io_error);
/// println!("Format Error: {}", format_error);
/// println!("Missing Header: {}", missing_header);
/// ```
#[derive(Debug)]
pub enum RequestError {
    Io(std::io::Error),
    InvalidFormat,
    MissingHeader(String),
}

impl std::fmt::Display for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RequestError::Io(err) => write!(f, "IO error: {}", err),
            RequestError::InvalidFormat => write!(f, "Invalid request format"),
            RequestError::MissingHeader(header) => write!(f, "Missing required header: {}", header),
        }
    }
}

impl std::error::Error for RequestError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            RequestError::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for RequestError {
    fn from(err: std::io::Error) -> Self {
        RequestError::Io(err)
    }
}

/// HTTP methods supported by the server.
///
/// This enum covers the basic HTTP methods that a static file server typically needs to handle.
/// Currently supports GET for retrieving resources, HEAD for metadata only, and OPTIONS for
/// CORS preflight requests.
///
/// # Examples
///
/// ```
/// use file_shover::message::HttpMethod;
/// use std::str::FromStr;
///
/// // Parse from string
/// let method = HttpMethod::from_str("GET").unwrap();
/// assert_eq!(method, HttpMethod::GET);
///
/// // Convert to string
/// assert_eq!(method.to_string(), "GET");
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum HttpMethod {
    GET,
    HEAD,
    OPTIONS,
}

impl std::str::FromStr for HttpMethod {
    type Err = RequestError;

    /// Parses an HTTP method from a string.
    ///
    /// # Examples
    ///
    /// ```
    /// use file_shover::message::HttpMethod;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(HttpMethod::from_str("GET").unwrap(), HttpMethod::GET);
    /// assert_eq!(HttpMethod::from_str("HEAD").unwrap(), HttpMethod::HEAD);
    /// assert_eq!(HttpMethod::from_str("OPTIONS").unwrap(), HttpMethod::OPTIONS);
    ///
    /// // Invalid methods return an error
    /// assert!(HttpMethod::from_str("POST").is_err());
    /// assert!(HttpMethod::from_str("get").is_err()); // case sensitive
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(HttpMethod::GET),
            "HEAD" => Ok(HttpMethod::HEAD),
            "OPTIONS" => Ok(HttpMethod::OPTIONS),
            _ => Err(RequestError::InvalidFormat),
        }
    }
}

impl std::fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let method_str = match self {
            HttpMethod::GET => "GET",
            HttpMethod::HEAD => "HEAD",
            HttpMethod::OPTIONS => "OPTIONS",
        };
        write!(f, "{}", method_str)
    }
}

/// HTTP request structure.
///
/// # Examples
///
/// ```
/// use file_shover::message::{Request, HttpMethod};
/// use std::io::Cursor;
///
/// let request_data = "GET /index.html HTTP/1.1\r\nHost: example.com\r\n\r\n";
/// let cursor = Cursor::new(request_data.as_bytes());
/// let request = Request::from_bytes(cursor).unwrap();
///
/// assert_eq!(request.method, HttpMethod::GET);
/// assert_eq!(request.path, "/index.html");
/// assert_eq!(request.http_version, "HTTP/1.1");
/// assert_eq!(request.headers.get("Host"), Some(&"example.com".to_string()));
/// ```
#[derive(Debug)]
pub struct Request {
    pub method: HttpMethod,
    pub path: String,
    pub http_version: String,
    pub headers: HashMap<String, String>,
}

/// HTTP status codes.
///
/// # Examples
///
/// ```
/// use file_shover::message::HttpStatus;
///
/// let status = HttpStatus::Ok;
/// assert_eq!(status.as_str(), "200 OK");
/// assert_eq!(status.to_string(), "200 OK");
/// assert_eq!(status as u16, 200);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum HttpStatus {
    Ok = 200,
    NotModified = 304,
    BadRequest = 400,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    InternalServerError = 500,
}

impl HttpStatus {
    /// Returns the status code and reason phrase as a string.
    ///
    /// # Examples
    ///
    /// ```
    /// use file_shover::message::HttpStatus;
    ///
    /// assert_eq!(HttpStatus::Ok.as_str(), "200 OK");
    /// assert_eq!(HttpStatus::NotFound.as_str(), "404 Not Found");
    /// assert_eq!(HttpStatus::InternalServerError.as_str(), "500 Internal Server Error");
    /// ```
    pub fn as_str(&self) -> &'static str {
        match self {
            HttpStatus::Ok => "200 OK",
            HttpStatus::NotModified => "304 Not Modified",
            HttpStatus::BadRequest => "400 Bad Request",
            HttpStatus::Forbidden => "403 Forbidden",
            HttpStatus::NotFound => "404 Not Found",
            HttpStatus::MethodNotAllowed => "405 Method Not Allowed",
            HttpStatus::InternalServerError => "500 Internal Server Error",
        }
    }
}

impl std::fmt::Display for HttpStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// HTTP response.
///
/// # Examples
///
/// ```
/// use file_shover::message::{Response, HttpStatus};
/// use std::io::Cursor;
///
/// let response = Response::new()
///     .status(HttpStatus::Ok)
///     .content_type("text/html")
///     .server("file-shover/1.0")
///     .body("<html><body>Hello World</body></html>".as_bytes().to_vec());
///
/// // Write to a buffer
/// let mut buffer = Vec::new();
/// response.write(&mut buffer).unwrap();
/// let response_str = String::from_utf8(buffer).unwrap();
///
/// assert!(response_str.contains("HTTP/1.1 200 OK"));
/// assert!(response_str.contains("Content-Type: text/html"));
/// assert!(response_str.contains("Hello World"));
/// ```
#[derive(Debug)]
pub struct Response {
    pub status: HttpStatus,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
}

impl Default for Response {
    fn default() -> Self {
        let df = Self {
            status: HttpStatus::Ok,
            headers: HashMap::new(),
            body: None,
        };
        df.server("file-shover/1.0").header("Connection", "close")
    }
}

impl Response {
    /// Creates a new response with default values (200 OK, no headers, no body).
    ///
    /// # Examples
    ///
    /// ```
    /// use file_shover::message::{Response, HttpStatus};
    ///
    /// let response = Response::new();
    /// assert_eq!(response.status, HttpStatus::Ok);
    /// assert!(response.body.is_none());
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the HTTP status code for this response.
    ///
    /// # Examples
    ///
    /// ```
    /// use file_shover::message::{Response, HttpStatus};
    ///
    /// let response = Response::new().status(HttpStatus::NotFound);
    /// assert_eq!(response.status, HttpStatus::NotFound);
    /// ```
    pub fn status(mut self, status: HttpStatus) -> Self {
        self.status = status;
        self
    }

    /// Adds a header to the response.
    ///
    /// # Examples
    ///
    /// ```
    /// use file_shover::message::Response;
    ///
    /// let response = Response::new()
    ///     .header("Content-Type", "application/json")
    ///     .header("Cache-Control", "no-cache");
    ///
    /// assert_eq!(response.headers.get("Content-Type"), Some(&"application/json".to_string()));
    /// assert_eq!(response.headers.get("Cache-Control"), Some(&"no-cache".to_string()));
    /// ```
    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(name.into(), value.into());
        self
    }

    /// Sets the response body.
    ///
    /// # Examples
    ///
    /// ```
    /// use file_shover::message::Response;
    ///
    /// let response = Response::new().body("Hello, World!".as_bytes().to_vec());
    /// assert_eq!(response.body, Some("Hello, World!".as_bytes().to_vec()));
    /// ```
    pub fn body(mut self, body: Vec<u8>) -> Self {
        self.body = Some(body);
        self
    }

    /// Sets the Content-Type header.
    ///
    /// This is a convenience method for setting the Content-Type header,
    /// commonly used by static file servers.
    ///
    /// # Examples
    ///
    /// ```
    /// use file_shover::message::Response;
    ///
    /// let response = Response::new().content_type("text/html; charset=utf-8");
    /// assert_eq!(response.headers.get("Content-Type"), Some(&"text/html; charset=utf-8".to_string()));
    /// ```
    pub fn content_type(self, mime_type: &str) -> Self {
        self.header("Content-Type", mime_type)
    }

    /// Sets the Server header.
    /// # Examples
    ///
    /// ```
    /// use file_shover::message::Response;
    ///
    /// let response = Response::new().server("file-shover/1.0");
    /// assert_eq!(response.headers.get("Server"), Some(&"file-shover/1.0".to_string()));
    /// ```
    pub fn server(self, name: &str) -> Self {
        self.header("Server", name)
    }

    /// Writes the HTTP response to the provided writer.
    ///
    /// # Examples
    ///
    /// ```
    /// use file_shover::message::{Response, HttpStatus};
    /// use std::io::Cursor;
    ///
    /// let response = Response::new()
    ///     .status(HttpStatus::Ok)
    ///     .content_type("text/plain")
    ///     .body("Hello, World!".as_bytes().to_vec());
    ///
    /// let mut buffer = Vec::new();
    /// response.write(&mut buffer).unwrap();
    ///
    /// let response_str = String::from_utf8(buffer).unwrap();
    /// assert!(response_str.starts_with("HTTP/1.1 200 OK"));
    /// assert!(response_str.contains("Content-Type: text/plain"));
    /// assert!(response_str.ends_with("Hello, World!"));
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an `std::io::Error` if writing to the stream fails.
    pub fn write<W: Write>(&self, stream: &mut W) -> std::io::Result<()> {
        // Status line
        writeln!(stream, "HTTP/1.1 {}", self.status.as_str())?;

        // Headers
        for (name, value) in &self.headers {
            writeln!(stream, "{}: {}", name, value)?;
        }

        // Empty line to separate headers from body
        writeln!(stream)?;

        // Body (if present)
        if let Some(ref body) = self.body {
            stream.write_all(body)?;
        }

        Ok(())
    }
}

impl Request {
    /// Parses an HTTP request from a byte stream.
    ///
    /// This method reads and parses an HTTP request from any type that implements
    /// the `Read` trait. It expects the request to be in standard HTTP/1.1 format
    /// with CRLF line endings.
    ///
    /// # Examples
    ///
    /// ```
    /// use file_shover::message::{Request, HttpMethod};
    /// use std::io::Cursor;
    ///
    /// let request_data = "GET /path HTTP/1.1\r\nHost: example.com\r\nUser-Agent: test\r\n\r\n";
    /// let cursor = Cursor::new(request_data.as_bytes());
    /// let request = Request::from_bytes(cursor).unwrap();
    ///
    /// assert_eq!(request.method, HttpMethod::GET);
    /// assert_eq!(request.path, "/path");
    /// assert_eq!(request.http_version, "HTTP/1.1");
    /// assert_eq!(request.headers.get("Host"), Some(&"example.com".to_string()));
    /// assert_eq!(request.headers.get("User-Agent"), Some(&"test".to_string()));
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a `RequestError` if:
    /// - An I/O error occurs while reading
    /// - The request format is invalid
    /// - Required components are missing
    /// - Headers are malformed
    pub fn from_bytes<R: Read>(stream: R) -> Result<Self, RequestError> {
        let mut reader = BufReader::new(stream);
        let mut request_line = String::new();
        reader.read_line(&mut request_line)?;

        // Parse request line
        let mut parts = request_line.trim().split_ascii_whitespace();
        let method = parts
            .next()
            .ok_or(RequestError::InvalidFormat)?
            .parse::<HttpMethod>()?;
        let path = parts.next().ok_or(RequestError::InvalidFormat)?.to_string();
        let http_version = parts.next().ok_or(RequestError::InvalidFormat)?.to_string();

        // Parse headers
        let headers: Result<HashMap<String, String>, RequestError> = reader
            .lines()
            .take_while(|line_result| line_result.as_ref().map_or(false, |line| !line.is_empty()))
            .map(|line_result| {
                let line = line_result?;
                let mut parts = line.splitn(2, ": ");
                let key = parts.next().ok_or(RequestError::InvalidFormat)?.to_string();
                let value = parts.next().ok_or(RequestError::InvalidFormat)?.to_string();
                Ok((key, value))
            })
            .collect();

        let headers = headers?;

        Ok(Request {
            method,
            path,
            http_version,
            headers,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_http_method_from_str_valid_cases() {
        // Test all valid HTTP methods
        assert_eq!(HttpMethod::from_str("GET").unwrap(), HttpMethod::GET);
        assert_eq!(HttpMethod::from_str("HEAD").unwrap(), HttpMethod::HEAD);
        assert_eq!(HttpMethod::from_str("OPTIONS").unwrap(), HttpMethod::OPTIONS);
    }

    #[test]
    fn test_http_method_from_str_invalid_cases() {
        // Test invalid methods return errors
        assert!(HttpMethod::from_str("POST").is_err());
        assert!(HttpMethod::from_str("PUT").is_err());
        assert!(HttpMethod::from_str("DELETE").is_err());
        assert!(HttpMethod::from_str("PATCH").is_err());
        assert!(HttpMethod::from_str("").is_err());
        assert!(HttpMethod::from_str("get").is_err()); // lowercase
        assert!(HttpMethod::from_str("Get").is_err()); // mixed case
        assert!(HttpMethod::from_str("INVALID").is_err());
    }

    #[test]
    fn test_http_method_from_str_error_type() {
        // Test that invalid methods return the correct error type
        match HttpMethod::from_str("POST") {
            Err(RequestError::InvalidFormat) => (), // Expected
            Err(other) => panic!("Expected InvalidFormat, got {:?}", other),
            Ok(method) => panic!("Expected error, got {:?}", method),
        }
    }

    #[test]
    fn test_http_method_display() {
        // Test the Display implementation
        assert_eq!(HttpMethod::GET.to_string(), "GET");
        assert_eq!(HttpMethod::HEAD.to_string(), "HEAD");
        assert_eq!(HttpMethod::OPTIONS.to_string(), "OPTIONS");
    }

    #[test]
    fn test_http_status_display() {
        // Test HTTP status display
        assert_eq!(HttpStatus::Ok.to_string(), "200 OK");
        assert_eq!(HttpStatus::NotFound.to_string(), "404 Not Found");
        assert_eq!(HttpStatus::InternalServerError.to_string(), "500 Internal Server Error");
    }

    #[test]
    fn test_response_builder() {
        // Test the response builder pattern
        let response = Response::new()
            .status(HttpStatus::Ok)
            .content_type("text/html")
            .server("test-server")
            .body("Hello World".as_bytes().to_vec());

        assert_eq!(response.status, HttpStatus::Ok);
        assert_eq!(response.headers.get("Content-Type"), Some(&"text/html".to_string()));
        assert_eq!(response.headers.get("Server"), Some(&"test-server".to_string()));
        assert_eq!(response.body, Some("Hello World".as_bytes().to_vec()));
    }
}
