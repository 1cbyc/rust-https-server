use bytes::Bytes;
use http::{HeaderMap, HeaderValue, Method, StatusCode, Uri, Version};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Request {
    pub method: Method,
    pub uri: Uri,
    pub version: Version,
    pub headers: HeaderMap,
    pub body: Option<Bytes>,
    pub params: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub status: StatusCode,
    pub headers: HeaderMap,
    pub body: Option<Bytes>,
}

impl Request {
    pub fn new(method: Method, uri: Uri, version: Version) -> Self {
        Self {
            method,
            uri,
            version,
            headers: HeaderMap::new(),
            body: None,
            params: HashMap::new(),
        }
    }

    pub fn path(&self) -> &str {
        self.uri.path()
    }

    pub fn query(&self) -> Option<&str> {
        self.uri.query()
    }

    pub fn header(&self, name: &str) -> Option<&HeaderValue> {
        self.headers.get(name)
    }

    pub fn content_length(&self) -> Option<usize> {
        self.headers
            .get("content-length")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse().ok())
    }

    pub fn content_type(&self) -> Option<&str> {
        self.headers
            .get("content-type")
            .and_then(|v| v.to_str().ok())
    }

    pub fn user_agent(&self) -> Option<&str> {
        self.headers
            .get("user-agent")
            .and_then(|v| v.to_str().ok())
    }

    pub fn accept_encoding(&self) -> Option<&str> {
        self.headers
            .get("accept-encoding")
            .and_then(|v| v.to_str().ok())
    }

    pub fn supports_gzip(&self) -> bool {
        self.accept_encoding()
            .map(|encoding| encoding.contains("gzip"))
            .unwrap_or(false)
    }

    pub fn body_as_string(&self) -> Option<String> {
        self.body.as_ref().map(|b| String::from_utf8_lossy(b).to_string())
    }

    pub fn body_as_json<T>(&self) -> crate::Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let body = self.body_as_string().ok_or_else(|| {
            crate::Error::BadRequest("Request body is required for JSON parsing".to_string())
        })?;
        let value: T = serde_json::from_str(&body)?;
        Ok(value)
    }
}

impl Response {
    pub fn new(status: StatusCode) -> Self {
        Self {
            status,
            headers: HeaderMap::new(),
            body: None,
        }
    }

    pub fn ok() -> Self {
        Self::new(StatusCode::OK)
    }

    pub fn not_found() -> Self {
        Self::new(StatusCode::NOT_FOUND)
    }

    pub fn bad_request() -> Self {
        Self::new(StatusCode::BAD_REQUEST)
    }

    pub fn internal_server_error() -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR)
    }

    pub fn created() -> Self {
        Self::new(StatusCode::CREATED)
    }

    pub fn method_not_allowed() -> Self {
        Self::new(StatusCode::METHOD_NOT_ALLOWED)
    }

    pub fn with_header(mut self, name: &str, value: &str) -> Self {
        if let Ok(header_value) = HeaderValue::from_str(value) {
            if let Ok(header_name) = http::header::HeaderName::from_lowercase(name.as_bytes()) {
                self.headers.insert(header_name, header_value);
            }
        }
        self
    }

    pub fn with_content_type(mut self, content_type: &str) -> Self {
        self.headers.insert("content-type", HeaderValue::from_str(content_type).unwrap_or_else(|_| HeaderValue::from_static("text/plain")));
        self
    }

    pub fn with_content_length(mut self, length: usize) -> Self {
        self.headers.insert("content-length", HeaderValue::from(length));
        self
    }

    pub fn with_body(mut self, body: impl Into<Bytes>) -> Self {
        let body_bytes = body.into();
        self.headers.insert("content-length", HeaderValue::from(body_bytes.len()));
        self.body = Some(body_bytes);
        self
    }

    pub fn with_json<T>(mut self, data: &T) -> crate::Result<Self>
    where
        T: Serialize,
    {
        let json = serde_json::to_string(data)?;
        let body_bytes = Bytes::from(json);
        self.headers.insert("content-type", HeaderValue::from_static("application/json"));
        self.headers.insert("content-length", HeaderValue::from(body_bytes.len()));
        self.body = Some(body_bytes);
        Ok(self)
    }

    pub fn with_text(mut self, text: &str) -> Self {
        let body_bytes = Bytes::from(text.to_string());
        self.headers.insert("content-type", HeaderValue::from_static("text/plain"));
        self.headers.insert("content-length", HeaderValue::from(body_bytes.len()));
        self.body = Some(body_bytes);
        self
    }

    pub fn with_html(mut self, html: &str) -> Self {
        let body_bytes = Bytes::from(html.to_string());
        self.headers.insert("content-type", HeaderValue::from_static("text/html"));
        self.headers.insert("content-length", HeaderValue::from(body_bytes.len()));
        self.body = Some(body_bytes);
        self
    }

    pub fn with_cors(mut self, origin: &str) -> Self {
        self.headers.insert("access-control-allow-origin", HeaderValue::from_str(origin).unwrap_or_else(|_| HeaderValue::from_static("*")));
        self.headers.insert("access-control-allow-methods", HeaderValue::from_static("GET, POST, PUT, DELETE, OPTIONS"));
        self.headers.insert("access-control-allow-headers", HeaderValue::from_static("Content-Type, Authorization"));
        self
    }

    pub fn with_compression(mut self, encoding: &str) -> Self {
        self.headers.insert("content-encoding", HeaderValue::from_str(encoding).unwrap_or_else(|_| HeaderValue::from_static("identity")));
        self
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut response = Vec::new();
        
        let status_line = format!("HTTP/1.1 {} {}\r\n", self.status.as_u16(), self.status.canonical_reason().unwrap_or("Unknown"));
        response.extend_from_slice(status_line.as_bytes());
        
        for (name, value) in &self.headers {
            let header_line = format!("{}: {}\r\n", name.as_str(), value.to_str().unwrap_or(""));
            response.extend_from_slice(header_line.as_bytes());
        }
        
        response.extend_from_slice(b"\r\n");
        
        if let Some(body) = &self.body {
            response.extend_from_slice(body);
        }
        
        response
    }
}

impl Default for Response {
    fn default() -> Self {
        Self::ok()
    }
} 