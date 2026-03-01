//! Configuration management module
//!
//! Provides configuration loading from environment variables for the MCP server.

use std::env;

/// Server configuration loaded from environment variables.
#[derive(Debug, Clone)]
pub struct Config {
    /// Keycloak server base URL
    pub keycloak_url: String,
    /// MCP server port
    pub mcp_port: u16,
    /// Logging level (trace, debug, info, warn, error)
    pub log_level: String,
}

impl Config {
    /// Load configuration from environment variables.
    ///
    /// # Environment Variables
    /// - `KEYCLOAK_URL` - Keycloak server URL (default: "http://localhost:8080")
    /// - `MCP_PORT` - Server port (default: 3000)
    /// - `LOG_LEVEL` - Logging level (default: "info")
    ///
    /// # Panics
    /// Panics if `MCP_PORT` is set but cannot be parsed as a valid port number.
    pub fn from_env() -> Self {
        let keycloak_url =
            env::var("KEYCLOAK_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());

        let mcp_port = env::var("MCP_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(3000);

        let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());

        Self {
            keycloak_url,
            mcp_port,
            log_level,
        }
    }

    /// Get the socket address for the MCP server.
    pub fn bind_address(&self) -> String {
        format!("0.0.0.0:{}", self.mcp_port)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            keycloak_url: "http://localhost:8080".to_string(),
            mcp_port: 3000,
            log_level: "info".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.keycloak_url, "http://localhost:8080");
        assert_eq!(config.mcp_port, 3000);
        assert_eq!(config.log_level, "info");
    }

    #[test]
    fn test_bind_address() {
        let config = Config {
            keycloak_url: "http://localhost:8080".to_string(),
            mcp_port: 4000,
            log_level: "debug".to_string(),
        };
        assert_eq!(config.bind_address(), "0.0.0.0:4000");
    }
}
