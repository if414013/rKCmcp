//! MCP Server implementation for Keycloak integration.

use rmcp::{handler::server::tool::ToolCallContext, model::*, RoleServer, ServerHandler};

use super::tools::KeycloakToolHandler;
use crate::api::KeycloakClient;
use crate::config::Config;

const SERVER_NAME: &str = "keycloak-mcp-server";
const SERVER_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Clone)]
pub struct KeycloakMcpServer {
    tool_handler: KeycloakToolHandler,
}

impl KeycloakMcpServer {
    pub fn new(client: KeycloakClient, token: String) -> Self {
        Self {
            tool_handler: KeycloakToolHandler::new(client, token),
        }
    }

    pub fn from_config(config: &Config) -> Result<Self, crate::api::error::ApiError> {
        let client = KeycloakClient::new(&config.keycloak_url)?;
        Ok(Self::new(client, String::new()))
    }

    pub async fn set_token(&self, token: String) {
        self.tool_handler.set_token(token).await;
    }

    pub fn tool_handler(&self) -> &KeycloakToolHandler {
        &self.tool_handler
    }
}

impl Default for KeycloakMcpServer {
    fn default() -> Self {
        let config = Config::from_env().unwrap_or_default();
        Self::from_config(&config).expect("Failed to create KeycloakMcpServer from config")
    }
}

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

    async fn list_tools(
        &self,
        _request: Option<PaginatedRequestParam>,
        _context: rmcp::service::RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, rmcp::ErrorData> {
        let items = self.tool_handler.router().list_all();
        Ok(ListToolsResult::with_all_items(items))
    }

    async fn call_tool(
        &self,
        request: CallToolRequestParam,
        context: rmcp::service::RequestContext<RoleServer>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let tcc = ToolCallContext::new(&self.tool_handler, request, context);
        self.tool_handler.router().call(tcc).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_server() -> KeycloakMcpServer {
        let client = KeycloakClient::new("http://localhost:8080").unwrap();
        KeycloakMcpServer::new(client, "test-token".to_string())
    }

    #[test]
    fn test_server_creation() {
        let server = create_test_server();
        let info = server.get_info();

        assert_eq!(info.server_info.name, "keycloak-mcp-server");
        assert_eq!(info.server_info.version, env!("CARGO_PKG_VERSION"));
    }

    #[test]
    fn test_server_capabilities() {
        let server = create_test_server();
        let info = server.get_info();

        assert!(info.capabilities.tools.is_some());
    }

    #[test]
    fn test_server_instructions() {
        let server = create_test_server();
        let info = server.get_info();

        assert!(info.instructions.is_some());
        assert!(info.instructions.unwrap().contains("Keycloak"));
    }

    #[test]
    fn test_protocol_version() {
        let server = create_test_server();
        let info = server.get_info();

        assert_eq!(info.protocol_version, ProtocolVersion::V_2024_11_05);
    }

    #[tokio::test]
    async fn test_server_token_update() {
        let server = create_test_server();
        server.set_token("new-token".to_string()).await;
    }
}
