use axum::{routing::get, Json, Router};
use keycloak_mcp_server::config::Config;
use keycloak_mcp_server::mcp::KeycloakMcpServer;
use rmcp::transport::streamable_http_server::{
    session::local::LocalSessionManager, StreamableHttpService,
};
use serde_json::{json, Value};
use std::sync::Arc;

async fn health_check() -> Json<Value> {
    Json(json!({"status": "ok"}))
}

#[tokio::main]
async fn main() {
    let config = match Config::from_env() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("❌ Configuration Error: {}", e);
            eprintln!("\nPlease ensure all required environment variables are set:");
            eprintln!("  - KEYCLOAK_URL: URL of your Keycloak server (required)");
            eprintln!("\nOptional environment variables (with defaults):");
            eprintln!("  - KEYCLOAK_REALM: Keycloak realm name (default: master)");
            eprintln!("  - MCP_PORT: Server port (default: 3000)");
            eprintln!("  - LOG_LEVEL: Logging level (default: info)");
            eprintln!("  - JWKS_CACHE_TTL: JWKS cache TTL in seconds (default: 3600)");
            std::process::exit(1);
        }
    };

    let bind_address = config.bind_address();

    let mcp_service = StreamableHttpService::new(
        || Ok(KeycloakMcpServer::default()),
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
