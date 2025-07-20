use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("HTTP error: {0}")]
    Http(#[from] http::Error),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Route not found: {0}")]
    RouteNotFound(String),

    #[error("Method not allowed: {0}")]
    MethodNotAllowed(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Invalid path: {0}")]
    InvalidPath(String),

    #[error("Content too large: {0}")]
    ContentTooLarge(usize),

    #[error("Unsupported encoding: {0}")]
    UnsupportedEncoding(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("URL parse error: {0}")]
    UrlParse(#[from] url::ParseError),

    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),

    #[error("Config error: {0}")]
    ConfigError(#[from] config::ConfigError),

    #[error("Invalid method: {0}")]
    InvalidMethod(#[from] http::method::InvalidMethod),

    #[error("Invalid URI: {0}")]
    InvalidUri(#[from] http::uri::InvalidUri),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<Error> for http::StatusCode {
    fn from(err: Error) -> Self {
        match err {
            Error::RouteNotFound(_) => http::StatusCode::NOT_FOUND,
            Error::MethodNotAllowed(_) => http::StatusCode::METHOD_NOT_ALLOWED,
            Error::BadRequest(_) => http::StatusCode::BAD_REQUEST,
            Error::FileNotFound(_) => http::StatusCode::NOT_FOUND,
            Error::PermissionDenied(_) => http::StatusCode::FORBIDDEN,
            Error::InvalidPath(_) => http::StatusCode::BAD_REQUEST,
            Error::ContentTooLarge(_) => http::StatusCode::PAYLOAD_TOO_LARGE,
            Error::UnsupportedEncoding(_) => http::StatusCode::UNSUPPORTED_MEDIA_TYPE,
            _ => http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
} 