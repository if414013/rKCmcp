//! Model Context Protocol (MCP) implementation and handlers

pub mod server;
pub mod tools;

pub use server::KeycloakMcpServer;
pub use tools::KeycloakToolHandler;
