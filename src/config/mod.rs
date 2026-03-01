//! Configuration management module
//!
//! Provides configuration loading from environment variables for the MCP server.

use std::env;

#[derive(Debug, Clone)]
pub enum ConfigError {
    MissingRequired(String),
    InvalidValue(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::MissingRequired(var) => {
                write!(f, "Missing required environment variable: {}", var)
            }
            ConfigError::InvalidValue(msg) => write!(f, "Invalid configuration value: {}", msg),
        }
    }
}

impl std::error::Error for ConfigError {}

#[derive(Debug, Clone)]
pub struct Config {
    pub keycloak_url: String,
    pub keycloak_realm: String,
    pub mcp_port: u16,
    pub log_level: String,
    pub jwks_cache_ttl: u64,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let keycloak_url = env::var("KEYCLOAK_URL").map_err(|_| {
            ConfigError::MissingRequired(
                "KEYCLOAK_URL is required. Please set it to your Keycloak server URL (e.g., http://localhost:8080)"
                    .to_string(),
            )
        })?;

        let keycloak_realm = env::var("KEYCLOAK_REALM").unwrap_or_else(|_| "master".to_string());

        let mcp_port = match env::var("MCP_PORT") {
            Ok(port_str) => port_str.parse().map_err(|_| {
                ConfigError::InvalidValue(
                    "MCP_PORT must be a valid port number (0-65535)".to_string(),
                )
            })?,
            Err(_) => 3000,
        };

        let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());

        let jwks_cache_ttl = match env::var("JWKS_CACHE_TTL") {
            Ok(ttl_str) => ttl_str.parse().map_err(|_| {
                ConfigError::InvalidValue(
                    "JWKS_CACHE_TTL must be a valid number of seconds".to_string(),
                )
            })?,
            Err(_) => 3600,
        };

        let config = Self {
            keycloak_url,
            keycloak_realm,
            mcp_port,
            log_level,
            jwks_cache_ttl,
        };

        Ok(config)
    }

    pub fn bind_address(&self) -> String {
        format!("0.0.0.0:{}", self.mcp_port)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            keycloak_url: "http://localhost:8080".to_string(),
            keycloak_realm: "master".to_string(),
            mcp_port: 3000,
            log_level: "info".to_string(),
            jwks_cache_ttl: 3600,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.keycloak_url, "http://localhost:8080");
        assert_eq!(config.keycloak_realm, "master");
        assert_eq!(config.mcp_port, 3000);
        assert_eq!(config.log_level, "info");
        assert_eq!(config.jwks_cache_ttl, 3600);
    }

    #[test]
    fn test_bind_address() {
        let config = Config {
            mcp_port: 4000,
            ..Config::default()
        };
        assert_eq!(config.bind_address(), "0.0.0.0:4000");
    }

    #[test]
    #[serial]
    fn test_from_env_with_keycloak_url_required() {
        env::remove_var("KEYCLOAK_URL");

        let result = Config::from_env();
        assert!(result.is_err());

        if let Err(ConfigError::MissingRequired(msg)) = result {
            assert!(msg.contains("KEYCLOAK_URL"));
        } else {
            panic!("Expected MissingRequired error for KEYCLOAK_URL");
        }
    }

    #[test]
    #[serial]
    fn test_from_env_with_invalid_mcp_port() {
        env::set_var("KEYCLOAK_URL", "http://localhost:8080");
        env::set_var("MCP_PORT", "invalid_port_test");

        let result = Config::from_env();

        env::remove_var("MCP_PORT");
        env::remove_var("KEYCLOAK_URL");

        assert!(result.is_err());
        if let Err(ConfigError::InvalidValue(msg)) = result {
            assert!(msg.contains("port number"));
        } else {
            panic!("Expected InvalidValue error for MCP_PORT");
        }
    }

    #[test]
    #[serial]
    fn test_from_env_with_invalid_jwks_cache_ttl() {
        env::set_var("KEYCLOAK_URL", "http://localhost:8080");
        env::set_var("JWKS_CACHE_TTL", "not_a_number_test");

        let result = Config::from_env();

        env::remove_var("JWKS_CACHE_TTL");
        env::remove_var("KEYCLOAK_URL");

        assert!(result.is_err());
        if let Err(ConfigError::InvalidValue(msg)) = result {
            assert!(msg.contains("seconds"));
        } else {
            panic!("Expected InvalidValue error for JWKS_CACHE_TTL");
        }
    }

    #[test]
    #[serial]
    fn test_from_env_with_custom_values() {
        env::set_var("KEYCLOAK_URL", "https://keycloak.example.com");
        env::set_var("KEYCLOAK_REALM", "my-realm");
        env::set_var("MCP_PORT", "8080");
        env::set_var("LOG_LEVEL", "debug");
        env::set_var("JWKS_CACHE_TTL", "7200");

        let result = Config::from_env();

        env::remove_var("KEYCLOAK_URL");
        env::remove_var("KEYCLOAK_REALM");
        env::remove_var("MCP_PORT");
        env::remove_var("LOG_LEVEL");
        env::remove_var("JWKS_CACHE_TTL");

        assert!(result.is_ok());
        let config = result.unwrap();
        assert_eq!(config.keycloak_url, "https://keycloak.example.com");
        assert_eq!(config.keycloak_realm, "my-realm");
        assert_eq!(config.mcp_port, 8080);
        assert_eq!(config.log_level, "debug");
        assert_eq!(config.jwks_cache_ttl, 7200);
    }

    #[test]
    #[serial]
    fn test_from_env_with_defaults() {
        env::set_var("KEYCLOAK_URL", "http://localhost:8080");
        env::remove_var("KEYCLOAK_REALM");
        env::remove_var("MCP_PORT");
        env::remove_var("LOG_LEVEL");
        env::remove_var("JWKS_CACHE_TTL");

        let result = Config::from_env();

        env::remove_var("KEYCLOAK_URL");

        assert!(result.is_ok());
        let config = result.unwrap();
        assert_eq!(config.keycloak_url, "http://localhost:8080");
        assert_eq!(config.keycloak_realm, "master");
        assert_eq!(config.mcp_port, 3000);
        assert_eq!(config.log_level, "info");
        assert_eq!(config.jwks_cache_ttl, 3600);
    }

    #[test]
    fn test_config_error_display() {
        let err = ConfigError::MissingRequired("TEST_VAR".to_string());
        assert!(err.to_string().contains("TEST_VAR"));
        assert!(err.to_string().contains("Missing required"));

        let err = ConfigError::InvalidValue("test value".to_string());
        assert!(err.to_string().contains("test value"));
        assert!(err.to_string().contains("Invalid configuration"));
    }
}
