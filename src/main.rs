//! Keycloak MCP Server
//!
//! A Model Context Protocol (MCP) server implementation for Keycloak integration.

pub mod api;
pub mod auth;
pub mod mcp;
pub mod config;
pub mod error;

#[tokio::main]
async fn main() {
    println!("Keycloak MCP Server starting...");
}
