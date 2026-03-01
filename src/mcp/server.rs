//! MCP Server implementation for Keycloak integration.

use rmcp::{
    handler::server::router::tool::ToolRouter, model::*, schemars, tool_handler, ServerHandler,
};

const SERVER_NAME: &str = "keycloak-mcp-server";
const SERVER_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Clone)]
pub struct KeycloakMcpServer {
    #[allow(dead_code)]
    tool_router: ToolRouter<Self>,
}

impl KeycloakMcpServer {
    pub fn new() -> Self {
        Self {
            tool_router: ToolRouter::new(),
        }
    }
}

impl Default for KeycloakMcpServer {
    fn default() -> Self {
        Self::new()
    }
}

#[tool_handler]
impl ServerHandler for KeycloakMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation {
                name: SERVER_NAME.into(),
                version: SERVER_VERSION.into(),
                title: None,
                icons: None,
                website_url: None,
            },
            instructions: Some(
                "Keycloak MCP Server - Manage Keycloak resources via MCP protocol".into(),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_creation() {
        let server = KeycloakMcpServer::new();
        let info = server.get_info();

        assert_eq!(info.server_info.name, "keycloak-mcp-server");
        assert_eq!(info.server_info.version, env!("CARGO_PKG_VERSION"));
    }

    #[test]
    fn test_server_default() {
        let server = KeycloakMcpServer::default();
        let info = server.get_info();

        assert_eq!(info.server_info.name, "keycloak-mcp-server");
    }

    #[test]
    fn test_server_capabilities() {
        let server = KeycloakMcpServer::new();
        let info = server.get_info();

        assert!(info.capabilities.tools.is_some());
    }

    #[test]
    fn test_server_instructions() {
        let server = KeycloakMcpServer::new();
        let info = server.get_info();

        assert!(info.instructions.is_some());
        assert!(info.instructions.unwrap().contains("Keycloak"));
    }

    #[test]
    fn test_protocol_version() {
        let server = KeycloakMcpServer::new();
        let info = server.get_info();

        assert_eq!(info.protocol_version, ProtocolVersion::V_2024_11_05);
    }
}
