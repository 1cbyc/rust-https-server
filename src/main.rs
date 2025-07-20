use clap::Parser;
use rust_https_server::config::Config;
use rust_https_server::server::Server;
use std::process;
use tracing::{error, info};

#[derive(Parser)]
#[command(name = "rust-https-server")]
#[command(about = "An HTTP server implementation in Rust")]
struct Args {
    #[arg(short, long, default_value = "127.0.0.1")]
    host: String,

    #[arg(short, long, default_value = "4221")]
    port: u16,

    #[arg(short, long)]
    config: Option<String>,

    #[arg(long, default_value = "info")]
    log_level: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    tracing_subscriber::fmt()
        .with_env_filter(format!("rust_https_server={}", args.log_level))
        .init();

    info!("Starting Rust HTTP Server");

    let config = match Config::load(args.config.as_deref()) {
        Ok(config) => config,
        Err(e) => {
            error!("Failed to load configuration: {}", e);
            process::exit(1);
        }
    };

    let server = Server::new(config);
    
    if let Err(e) = server.run().await {
        error!("Server error: {}", e);
        process::exit(1);
    }
}
