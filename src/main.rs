//! Keycloak MCP Server
//!
//! A Model Context Protocol (MCP) server implementation for Keycloak integration.

pub mod api;
pub mod auth;
pub mod config;
pub mod error;
pub mod mcp;

use axum::{Json, Router, routing::get};
use rmcp::transport::streamable_http_server::{
    StreamableHttpService, session::local::LocalSessionManager,
};
use serde_json::{Value, json};
use std::sync::Arc;

use crate::config::Config;
use crate::mcp::KeycloakMcpServer;

async fn health_check() -> Json<Value> {
    Json(json!({"status": "ok"}))
}

#[tokio::main]
async fn main() {
    let config = Config::from_env();
    let bind_address = config.bind_address();

    let mcp_service = StreamableHttpService::new(
        || Ok(KeycloakMcpServer::new()),
        Arc::new(LocalSessionManager::default()),
        Default::default(),
    );

    let router = Router::new()
        .route("/health", get(health_check))
        .nest_service("/mcp", mcp_service);

    let listener = tokio::net::TcpListener::bind(&bind_address)
        .await
        .expect("Failed to bind to address");

    println!("Keycloak MCP Server starting on {}", bind_address);
    println!("Health check: http://{}/health", bind_address);
    println!("MCP endpoint: http://{}/mcp", bind_address);

    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("Server failed");
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => println!("\nReceived Ctrl+C, shutting down..."),
        _ = terminate => println!("\nReceived SIGTERM, shutting down..."),
    }
}
