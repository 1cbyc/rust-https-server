pub mod config;
pub mod error;
pub mod http;
pub mod router;
pub mod server;
pub mod utils;

pub use error::{Error, Result};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::{Request, Response};
    use http::{Method, Uri, Version};

    #[test]
    fn test_response_creation() {
        let response = Response::ok().with_text("Hello, World!");
        assert_eq!(response.status, http::StatusCode::OK);
        assert!(response.body.is_some());
    }

    #[test]
    fn test_request_creation() {
        let uri = "http://localhost:4221/test".parse::<Uri>().unwrap();
        let request = Request::new(Method::GET, uri, Version::HTTP_11);
        assert_eq!(request.method(), Method::GET);
        assert_eq!(request.path(), "/test");
    }

    #[test]
    fn test_config_default() {
        let config = config::Config::default();
        assert_eq!(config.server.port, 4221);
        assert_eq!(config.server.host, "127.0.0.1");
    }

    #[test]
    fn test_utils_sanitize_path() {
        assert!(utils::sanitize_path("test.txt").is_ok());
        assert!(utils::sanitize_path("../test.txt").is_err());
        assert!(utils::sanitize_path("test\\file.txt").is_err());
    }

    #[test]
    fn test_utils_get_mime_type() {
        assert_eq!(utils::get_mime_type("test.html"), "text/html");
        assert_eq!(utils::get_mime_type("test.css"), "text/css");
        assert_eq!(utils::get_mime_type("test.js"), "application/javascript");
        assert_eq!(utils::get_mime_type("test.json"), "application/json");
        assert_eq!(utils::get_mime_type("test.txt"), "text/plain");
        assert_eq!(utils::get_mime_type("test.jpg"), "image/jpeg");
        assert_eq!(utils::get_mime_type("test.png"), "image/png");
        assert_eq!(utils::get_mime_type("test.unknown"), "application/octet-stream");
    }
} 