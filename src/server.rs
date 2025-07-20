use crate::{
    config::Config,
    error::{Error, Result},
    http::{Request, Response},
    router::Router,
    utils,
};
use bytes::Bytes;
use http::{HeaderMap, HeaderValue, Method, Uri, Version};
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tracing::{error, info};

pub struct Server {
    config: Config,
    router: Router,
}

impl Server {
    pub fn new(config: Config) -> Self {
        let mut server = Self {
            config,
            router: Router::new(),
        };
        server.setup_routes();
        server
    }

    pub async fn run(&self) -> Result<()> {
        let addr = format!("{}:{}", self.config.server.host, self.config.server.port);
        let listener = TcpListener::bind(&addr).await?;
        
        info!("Server listening on {}", addr);
        
        loop {
            match listener.accept().await {
                Ok((socket, addr)) => {
                    let config = self.config.clone();
                    let router = self.router.clone();
                    
                    tokio::spawn(async move {
                        if let Err(e) = Self::handle_connection(socket, addr, config, router).await {
                            error!("Connection error: {}", e);
                        }
                    });
                }
                Err(e) => {
                    error!("Accept error: {}", e);
                }
            }
        }
    }

    async fn handle_connection(
        socket: TcpStream,
        _addr: SocketAddr,
        config: Config,
        router: Router,
    ) -> Result<()> {
        let mut stream = socket;
        let mut buffer = Vec::new();
        let mut temp_buffer = [0; 4096];
        
        loop {
            let n = stream.read(&mut temp_buffer).await?;
            if n == 0 {
                break;
            }
            buffer.extend_from_slice(&temp_buffer[..n]);
            
            if let Some(request) = Self::parse_request(&buffer)? {
                let response = Self::process_request(request, &config, &router).await?;
                Self::send_response(&mut stream, response).await?;
                break;
            }
        }
        
        Ok(())
    }

    fn parse_request(buffer: &[u8]) -> Result<Option<Request>> {
        let mut lines = buffer.split(|&b| b == b'\n');
        
        let request_line = lines.next().ok_or_else(|| Error::Parse("No request line".to_string()))?;
        let request_line = std::str::from_utf8(request_line).map_err(|_| Error::Parse("Invalid UTF-8".to_string()))?;
        let request_line = request_line.trim_end_matches('\r');
        
        let parts: Vec<&str> = request_line.split_whitespace().collect();
        if parts.len() != 3 {
            return Err(Error::Parse("Invalid request line".to_string()));
        }
        
        let method = parts[0].parse::<Method>()?;
        let uri = parts[1].parse::<Uri>()?;
        let version = match parts[2] {
            "HTTP/1.0" => Version::HTTP_10,
            "HTTP/1.1" => Version::HTTP_11,
            _ => return Err(Error::Parse("Unsupported HTTP version".to_string())),
        };
        
        let mut request = Request::new(method, uri, version);
        let mut headers = HeaderMap::new();
        let mut content_length = None;
        
        for line in lines {
            let line = std::str::from_utf8(line).map_err(|_| Error::Parse("Invalid UTF-8 in headers".to_string()))?;
            let line = line.trim_end_matches('\r');
            
            if line.is_empty() {
                break;
            }
            
            if let Some((name, value)) = line.split_once(':') {
                let name = name.trim().to_lowercase();
                let value = value.trim();
                
                if let Ok(header_value) = HeaderValue::from_str(value) {
                    if let Ok(header_name) = http::header::HeaderName::from_lowercase(name.as_bytes()) {
                        headers.insert(header_name, header_value);
                    }
                    
                    if name == "content-length" {
                        content_length = value.parse::<usize>().ok();
                    }
                }
            }
        }
        
        request.headers = headers;
        
        if let Some(length) = content_length {
            let body_start = buffer.windows(4).position(|window| window == b"\r\n\r\n");
            if let Some(start) = body_start {
                let body_data = &buffer[start + 4..];
                if body_data.len() >= length {
                    request.body = Some(Bytes::copy_from_slice(&body_data[..length]));
                }
            }
        }
        
        Ok(Some(request))
    }

    async fn process_request(request: Request, _config: &Config, router: &Router) -> Result<Response> {
        router.handle(request)
    }

    async fn send_response(stream: &mut TcpStream, response: Response) -> Result<()> {
        let response_bytes = response.to_bytes();
        stream.write_all(&response_bytes).await?;
        stream.flush().await?;
        Ok(())
    }

    fn setup_routes(&mut self) {
        let config = self.config.clone();
        
        self.router
            .get("/", move |_| {
                Ok(Response::ok().with_text("Welcome to Rust HTTP Server"))
            })
            .get("/user-agent", move |request| {
                if let Some(user_agent) = request.user_agent() {
                    Ok(Response::ok().with_text(user_agent))
                } else {
                    Ok(Response::bad_request().with_text("User-Agent header not found"))
                }
            })
            .get("/echo/{param}", move |request| {
                let empty = String::new();
                let param = request.params.get("param").unwrap_or(&empty);
                Ok(Response::ok().with_text(param))
            })
            .post("/echo/{param}", move |request| {
                let empty = String::new();
                let param = request.params.get("param").unwrap_or(&empty);
                Ok(Response::ok().with_text(param))
            })
            .get("/files/{filename}", {
                let config = config.clone();
                move |request| {
                    let empty = String::new();
                    let filename = request.params.get("filename").unwrap_or(&empty);
                    Self::handle_file_get(filename, &config)
                }
            })
            .post("/files/{filename}", {
                let config = config.clone();
                move |request| {
                    let empty = String::new();
                    let filename = request.params.get("filename").unwrap_or(&empty);
                    let body = request.body_as_string().unwrap_or_default();
                    Self::handle_file_post(filename, &body, &config)
                }
            })
            .delete("/files/{filename}", {
                let config = config.clone();
                move |request| {
                    let empty = String::new();
                    let filename = request.params.get("filename").unwrap_or(&empty);
                    Self::handle_file_delete(filename, &config)
                }
            });
    }

    fn handle_file_get(filename: &str, config: &Config) -> Result<Response> {
        let sanitized_path = utils::sanitize_path(filename)?;
        utils::validate_file_extension(&sanitized_path, &config.files.allowed_extensions)?;
        
        let file_path = std::path::Path::new(&config.files.root_dir).join(&sanitized_path);
        
        if !file_path.exists() {
            return Ok(Response::not_found().with_text("File not found"));
        }
        
        if !file_path.is_file() {
            if file_path.is_dir() && config.files.enable_directory_listing {
                return Self::handle_directory_listing(&file_path, &sanitized_path);
            }
            return Ok(Response::not_found().with_text("Not a file"));
        }
        
        let content = std::fs::read(&file_path)?;
        let mime_type = utils::get_mime_type(&sanitized_path);
        
        Ok(Response::ok()
            .with_content_type(mime_type)
            .with_body(content))
    }

    fn handle_file_post(filename: &str, content: &str, config: &Config) -> Result<Response> {
        let sanitized_path = utils::sanitize_path(filename)?;
        utils::validate_file_extension(&sanitized_path, &config.files.allowed_extensions)?;
        
        if content.len() > config.files.max_file_size {
            return Err(Error::ContentTooLarge(content.len()));
        }
        
        let file_path = std::path::Path::new(&config.files.root_dir).join(&sanitized_path);
        
        std::fs::create_dir_all(file_path.parent().unwrap_or_else(|| std::path::Path::new("")))?;
        std::fs::write(&file_path, content)?;
        
        Ok(Response::created().with_text("File created successfully"))
    }

    fn handle_file_delete(filename: &str, config: &Config) -> Result<Response> {
        let sanitized_path = utils::sanitize_path(filename)?;
        utils::validate_file_extension(&sanitized_path, &config.files.allowed_extensions)?;
        
        let file_path = std::path::Path::new(&config.files.root_dir).join(&sanitized_path);
        
        if !file_path.exists() {
            return Ok(Response::not_found().with_text("File not found"));
        }
        
        if !file_path.is_file() {
            return Ok(Response::bad_request().with_text("Not a file"));
        }
        
        std::fs::remove_file(&file_path)?;
        
        Ok(Response::ok().with_text("File deleted successfully"))
    }

    fn handle_directory_listing(dir_path: &std::path::Path, path: &str) -> Result<Response> {
        let entries: Vec<std::fs::DirEntry> = std::fs::read_dir(dir_path)?
            .filter_map(|entry| entry.ok())
            .collect();
        
        let html = utils::generate_directory_listing(path, &entries);
        
        Ok(Response::ok()
            .with_content_type("text/html")
            .with_body(html))
    }
}

 