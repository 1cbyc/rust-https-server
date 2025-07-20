use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub files: FileConfig,
    pub security: SecurityConfig,
    pub performance: PerformanceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: usize,
    pub backlog: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileConfig {
    pub root_dir: String,
    pub max_file_size: usize,
    pub allowed_extensions: Vec<String>,
    pub enable_directory_listing: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub max_request_size: usize,
    pub allowed_origins: Vec<String>,
    pub enable_cors: bool,
    pub rate_limit_requests: usize,
    pub rate_limit_window: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    pub connection_timeout: u64,
    pub keep_alive_timeout: u64,
    pub max_connections: usize,
    pub enable_compression: bool,
    pub compression_level: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
            files: FileConfig::default(),
            security: SecurityConfig::default(),
            performance: PerformanceConfig::default(),
        }
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 4221,
            workers: num_cpus::get(),
            backlog: 1024,
        }
    }
}

impl Default for FileConfig {
    fn default() -> Self {
        Self {
            root_dir: "./files".to_string(),
            max_file_size: 100 * 1024 * 1024,
            allowed_extensions: vec![
                "txt".to_string(), "html".to_string(), "css".to_string(), "js".to_string(), 
                "json".to_string(), "xml".to_string(), "pdf".to_string(), "jpg".to_string(), 
                "jpeg".to_string(), "png".to_string(), "gif".to_string()
            ],
            enable_directory_listing: false,
        }
    }
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            max_request_size: 10 * 1024 * 1024,
            allowed_origins: vec!["*".to_string()],
            enable_cors: true,
            rate_limit_requests: 1000,
            rate_limit_window: 60,
        }
    }
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            connection_timeout: 30,
            keep_alive_timeout: 5,
            max_connections: 10000,
            enable_compression: true,
            compression_level: 6,
        }
    }
}

impl Config {
    pub fn load(config_path: Option<&str>) -> crate::Result<Self> {
        let mut config = config::Config::default();

        if let Some(path) = config_path {
            if Path::new(path).exists() {
                config.merge(config::File::with_name(path))?;
            }
        }

        config.merge(config::Environment::with_prefix("RUST_HTTP_SERVER"))?;

        let config: Config = config.try_deserialize().unwrap_or_else(|_| Config::default());
        Ok(config)
    }

    pub fn validate(&self) -> crate::Result<()> {
        if self.server.port == 0 {
            return Err(crate::Error::Config("Port cannot be 0".to_string()));
        }

        if self.server.workers == 0 {
            return Err(crate::Error::Config("Workers cannot be 0".to_string()));
        }

        if self.security.max_request_size == 0 {
            return Err(crate::Error::Config("Max request size cannot be 0".to_string()));
        }

        if self.files.max_file_size == 0 {
            return Err(crate::Error::Config("Max file size cannot be 0".to_string()));
        }

        Ok(())
    }
} 