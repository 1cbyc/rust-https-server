# Rust HTTP Server

A sophisticated, production-ready HTTP server implementation in Rust with modern features and comprehensive error handling.

## Features

- **Async I/O**: Built on Tokio for high-performance async operations
- **Routing**: Dynamic route matching with parameter extraction
- **Middleware**: Extensible middleware system for request processing
- **File Operations**: Complete file management with security validation
- **Compression**: Automatic gzip compression for text-based responses
- **CORS Support**: Cross-Origin Resource Sharing headers
- **Configuration**: Flexible configuration management
- **Logging**: Structured logging with tracing
- **Error Handling**: Comprehensive error management with custom types
- **Security**: Path traversal protection and input validation

## Quick Start

### Prerequisites

- Rust 1.70 or later
- Cargo

### Installation

1. Clone the repository:
```bash
git clone https://github.com/1cbyc/rust-https-server.git
cd rust-https-server
```

2. Build the project:
```bash
cargo build --release
```

3. Run the server:
```bash
cargo run --release
```

The server will start on `127.0.0.1:4221` by default.

## Usage

### Command Line Options

```bash
cargo run --release -- --help
```

Available options:
- `-h, --host`: Server host (default: 127.0.0.1)
- `-p, --port`: Server port (default: 4221)
- `-c, --config`: Configuration file path
- `--log-level`: Logging level (default: info)

### Configuration

The server supports configuration via:
- Environment variables (prefixed with `RUST_HTTP_SERVER_`)
- Configuration files (TOML format)
- Command line arguments

Example configuration file (`config.toml`):
```toml
[server]
host = "127.0.0.1"
port = 4221
workers = 4
backlog = 1024

[files]
root_dir = "./files"
max_file_size = 104857600
allowed_extensions = ["txt", "html", "css", "js", "json", "xml", "pdf", "jpg", "jpeg", "png", "gif"]
enable_directory_listing = false

[security]
max_request_size = 10485760
allowed_origins = ["*"]
enable_cors = true
rate_limit_requests = 1000
rate_limit_window = 60

[performance]
connection_timeout = 30
keep_alive_timeout = 5
max_connections = 10000
enable_compression = true
compression_level = 6
```

## API Endpoints

### Basic Routes

- `GET /` - Welcome message
- `GET /user-agent` - Returns the User-Agent header
- `GET /echo/{param}` - Echoes the parameter value
- `POST /echo/{param}` - Echoes the parameter value

### File Operations

- `GET /files/{filename}` - Retrieve file contents
- `POST /files/{filename}` - Create or update file
- `DELETE /files/{filename}` - Delete file

### Examples

```bash
# Get user agent
curl http://localhost:4221/user-agent

# Echo a parameter
curl http://localhost:4221/echo/hello-world

# Create a file
curl -X POST -d "Hello, World!" http://localhost:4221/files/test.txt

# Get file contents
curl http://localhost:4221/files/test.txt

# Delete a file
curl -X DELETE http://localhost:4221/files/test.txt
```

## Development

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

### Testing

```bash
# Run tests
cargo test

# Run tests with output
cargo test -- --nocapture
```

### Benchmarking

```bash
# Run benchmarks
cargo bench
```

## Architecture

The server is built with a modular architecture:

- **Server**: Main server instance and connection handling
- **Router**: HTTP request routing with parameter extraction
- **HTTP**: Request and response handling
- **Middleware**: Extensible middleware system
- **Config**: Configuration management
- **Error**: Comprehensive error handling
- **Utils**: Utility functions and helpers

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Run the test suite
6. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Roadmap

See [docs/whats-next.md](docs/whats-next.md) for the development roadmap and planned features.
