mod proxy;
mod config;
mod error;

use axum::{Router, routing::any};
use crate::proxy::proxy_handler;
use crate::config::LISTEN_ADDR;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📋 Initializing reverse proxy server");
    println!("📋 Upstream server: {}", crate::config::UPSTREAM_BASE);
    
    // Create our application with a route that matches any path
   let app = Router::new()
    .route("/", any(proxy_handler))         // catch `/`
    .route("/*path", any(proxy_handler));
    
    println!("🚀 Reverse Proxy running on http://{LISTEN_ADDR}");
    println!("🔌 Ready to handle requests...");
    let addr: SocketAddr = LISTEN_ADDR.parse()?;
    
    // In Axum 0.7, we use a TcpListener
    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("📡 Server bound to {}", addr);
    
    // Serve the application
    println!("🔄 Starting server loop");
    axum::serve(listener, app).await?;
    
    println!("👋 Server shutting down gracefully");
    Ok(())
}

