//! Error types and handling
//!
//! This module defines unified error types for the MCP server, mapping Keycloak API errors
//! and internal errors to MCP JSON-RPC error codes.

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Unified error type for the MCP server
#[derive(Debug, Error)]
pub enum McpServerError {
    /// Keycloak API error
    #[error("API error: {0}")]
    Api(#[from] crate::api::error::ApiError),

    /// Authentication error
    #[error("Authentication error: {0}")]
    Auth(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),

    /// Internal server error
    #[error("Internal error: {0}")]
    Internal(String),

    /// Invalid request or parameters
    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    /// Resource not found
    #[error("Resource not found: {0}")]
    NotFound(String),

    /// Permission denied
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
}

impl McpServerError {
    /// Convert error to MCP JSON-RPC error code
    ///
    /// Maps server errors to JSON-RPC 2.0 error codes:
    /// - InvalidRequest → -32600 (JSON-RPC standard)
    /// - NotFound → -32001 (custom)
    /// - PermissionDenied → -32002 (custom)
    /// - Auth → -32003 (custom)
    /// - Internal → -32603 (JSON-RPC standard)
    /// - Api errors are mapped based on their type
    pub fn to_mcp_error_code(&self) -> i32 {
        match self {
            McpServerError::InvalidRequest(_) => -32600,
            McpServerError::NotFound(_) => -32001,
            McpServerError::PermissionDenied(_) => -32002,
            McpServerError::Auth(_) => -32003,
            McpServerError::Internal(_) => -32603,
            McpServerError::Config(_) => -32603,
            McpServerError::Api(api_err) => api_err_to_mcp_code(api_err),
        }
    }

    /// Get sanitized error message for client response
    ///
    /// Ensures internal implementation details are not leaked to clients
    pub fn get_client_message(&self) -> String {
        match self {
            McpServerError::InvalidRequest(msg) => format!("Invalid request: {}", msg),
            McpServerError::NotFound(msg) => format!("Not found: {}", msg),
            McpServerError::PermissionDenied(_) => "Permission denied".to_string(),
            McpServerError::Auth(_) => "Authentication failed".to_string(),
            McpServerError::Internal(_) => "Internal server error".to_string(),
            McpServerError::Config(_) => "Server configuration error".to_string(),
            McpServerError::Api(api_err) => api_err_to_client_message(api_err),
        }
    }
}

/// Maps API errors to MCP JSON-RPC error codes
fn api_err_to_mcp_code(err: &crate::api::error::ApiError) -> i32 {
    use crate::api::error::ApiError;
    match err {
        ApiError::NotFound => -32001,
        ApiError::Unauthorized => -32003,
        ApiError::Forbidden => -32002,
        ApiError::BadRequest(_) => -32602, // Invalid params
        ApiError::Conflict(_) => -32603,   // Internal error (conflict state)
        ApiError::ServerError(_) => -32603,
        ApiError::HttpError(_) => -32603,
        ApiError::JsonError(_) => -32600,
    }
}

/// Converts API errors to sanitized client messages
fn api_err_to_client_message(err: &crate::api::error::ApiError) -> String {
    use crate::api::error::ApiError;
    match err {
        ApiError::NotFound => "Resource not found".to_string(),
        ApiError::Unauthorized => "Invalid or missing credentials".to_string(),
        ApiError::Forbidden => "Insufficient permissions".to_string(),
        ApiError::BadRequest(msg) => format!("Invalid request: {}", msg),
        ApiError::Conflict(msg) => format!("Resource conflict: {}", msg),
        ApiError::ServerError(_) => "Keycloak server error".to_string(),
        ApiError::HttpError(_) => "Network error".to_string(),
        ApiError::JsonError(_) => "Data format error".to_string(),
    }
}

/// MCP JSON-RPC error response format
///
/// Represents a JSON-RPC 2.0 error response as per the MCP specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpErrorResponse {
    /// JSON-RPC version
    pub jsonrpc: String,
    /// Error object
    pub error: ErrorObject,
    /// Request ID (if available)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<serde_json::Value>,
}

/// JSON-RPC error object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorObject {
    /// Error code
    pub code: i32,
    /// Error message
    pub message: String,
    /// Optional error data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl McpErrorResponse {
    /// Create a new error response from a McpServerError
    pub fn from_error(error: &McpServerError, id: Option<serde_json::Value>) -> Self {
        McpErrorResponse {
            jsonrpc: "2.0".to_string(),
            error: ErrorObject {
                code: error.to_mcp_error_code(),
                message: error.get_client_message(),
                data: None,
            },
            id,
        }
    }

    /// Create a parse error response
    pub fn parse_error(id: Option<serde_json::Value>) -> Self {
        McpErrorResponse {
            jsonrpc: "2.0".to_string(),
            error: ErrorObject {
                code: -32700,
                message: "Parse error".to_string(),
                data: None,
            },
            id,
        }
    }

    /// Create a method not found response
    pub fn method_not_found(id: Option<serde_json::Value>) -> Self {
        McpErrorResponse {
            jsonrpc: "2.0".to_string(),
            error: ErrorObject {
                code: -32601,
                message: "Method not found".to_string(),
                data: None,
            },
            id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::error::ApiError;

    #[test]
    fn test_invalid_request_error_code() {
        let err = McpServerError::InvalidRequest("test param".to_string());
        assert_eq!(err.to_mcp_error_code(), -32600);
    }

    #[test]
    fn test_not_found_error_code() {
        let err = McpServerError::NotFound("user".to_string());
        assert_eq!(err.to_mcp_error_code(), -32001);
    }

    #[test]
    fn test_permission_denied_error_code() {
        let err = McpServerError::PermissionDenied("access denied".to_string());
        assert_eq!(err.to_mcp_error_code(), -32002);
    }

    #[test]
    fn test_auth_error_code() {
        let err = McpServerError::Auth("invalid token".to_string());
        assert_eq!(err.to_mcp_error_code(), -32003);
    }

    #[test]
    fn test_internal_error_code() {
        let err = McpServerError::Internal("unexpected state".to_string());
        assert_eq!(err.to_mcp_error_code(), -32603);
    }

    #[test]
    fn test_config_error_code() {
        let err = McpServerError::Config("missing env var".to_string());
        assert_eq!(err.to_mcp_error_code(), -32603);
    }

    #[test]
    fn test_api_not_found_error_code() {
        let api_err = ApiError::NotFound;
        let err = McpServerError::Api(api_err);
        assert_eq!(err.to_mcp_error_code(), -32001);
    }

    #[test]
    fn test_api_unauthorized_error_code() {
        let api_err = ApiError::Unauthorized;
        let err = McpServerError::Api(api_err);
        assert_eq!(err.to_mcp_error_code(), -32003);
    }

    #[test]
    fn test_api_forbidden_error_code() {
        let api_err = ApiError::Forbidden;
        let err = McpServerError::Api(api_err);
        assert_eq!(err.to_mcp_error_code(), -32002);
    }

    #[test]
    fn test_api_bad_request_error_code() {
        let api_err = ApiError::BadRequest("invalid data".to_string());
        let err = McpServerError::Api(api_err);
        assert_eq!(err.to_mcp_error_code(), -32602);
    }

    #[test]
    fn test_api_conflict_error_code() {
        let api_err = ApiError::Conflict("already exists".to_string());
        let err = McpServerError::Api(api_err);
        assert_eq!(err.to_mcp_error_code(), -32603);
    }

    #[test]
    fn test_api_server_error_code() {
        let api_err = ApiError::ServerError("500 error".to_string());
        let err = McpServerError::Api(api_err);
        assert_eq!(err.to_mcp_error_code(), -32603);
    }

    #[test]
    fn test_invalid_request_client_message() {
        let err = McpServerError::InvalidRequest("bad param".to_string());
        let msg = err.get_client_message();
        assert!(msg.contains("Invalid request"));
        assert!(msg.contains("bad param"));
    }

    #[test]
    fn test_not_found_client_message() {
        let err = McpServerError::NotFound("user 123".to_string());
        let msg = err.get_client_message();
        assert!(msg.contains("Not found"));
        assert!(msg.contains("user 123"));
    }

    #[test]
    fn test_permission_denied_sanitizes_message() {
        let err = McpServerError::PermissionDenied("internal reason".to_string());
        let msg = err.get_client_message();
        assert_eq!(msg, "Permission denied");
        // Verify internal reason is not exposed
        assert!(!msg.contains("internal reason"));
    }

    #[test]
    fn test_auth_sanitizes_message() {
        let err = McpServerError::Auth("specific token error".to_string());
        let msg = err.get_client_message();
        assert_eq!(msg, "Authentication failed");
        // Verify internal details are not exposed
        assert!(!msg.contains("token error"));
    }

    #[test]
    fn test_internal_sanitizes_message() {
        let err = McpServerError::Internal("database connection failed".to_string());
        let msg = err.get_client_message();
        assert_eq!(msg, "Internal server error");
        // Verify internal details are not exposed
        assert!(!msg.contains("database"));
    }

    #[test]
    fn test_api_not_found_client_message() {
        let api_err = ApiError::NotFound;
        let err = McpServerError::Api(api_err);
        let msg = err.get_client_message();
        assert_eq!(msg, "Resource not found");
    }

    #[test]
    fn test_api_unauthorized_client_message() {
        let api_err = ApiError::Unauthorized;
        let err = McpServerError::Api(api_err);
        let msg = err.get_client_message();
        assert!(msg.contains("credentials"));
    }

    #[test]
    fn test_api_forbidden_client_message() {
        let api_err = ApiError::Forbidden;
        let err = McpServerError::Api(api_err);
        let msg = err.get_client_message();
        assert!(msg.contains("permissions"));
    }

    #[test]
    fn test_api_bad_request_client_message() {
        let api_err = ApiError::BadRequest("invalid field".to_string());
        let err = McpServerError::Api(api_err);
        let msg = err.get_client_message();
        assert!(msg.contains("Invalid request"));
    }

    #[test]
    fn test_from_api_error_conversion() {
        let api_err = ApiError::NotFound;
        let err: McpServerError = api_err.into();
        assert_eq!(err.to_mcp_error_code(), -32001);
    }

    #[test]
    fn test_mcp_error_response_creation() {
        let err = McpServerError::NotFound("test".to_string());
        let resp = McpErrorResponse::from_error(&err, Some(serde_json::json!(1)));
        assert_eq!(resp.jsonrpc, "2.0");
        assert_eq!(resp.error.code, -32001);
        assert_eq!(resp.id, Some(serde_json::json!(1)));
    }

    #[test]
    fn test_mcp_error_response_serialization() {
        let err = McpServerError::InvalidRequest("test".to_string());
        let resp = McpErrorResponse::from_error(&err, Some(serde_json::json!(1)));
        let json = serde_json::to_string(&resp).unwrap();
        assert!(json.contains("\"code\":-32600"));
        assert!(json.contains("\"jsonrpc\":\"2.0\""));
    }

    #[test]
    fn test_parse_error_response() {
        let resp = McpErrorResponse::parse_error(Some(serde_json::json!(1)));
        assert_eq!(resp.error.code, -32700);
        assert!(resp.error.message.contains("Parse error"));
    }

    #[test]
    fn test_method_not_found_response() {
        let resp = McpErrorResponse::method_not_found(Some(serde_json::json!(1)));
        assert_eq!(resp.error.code, -32601);
        assert!(resp.error.message.contains("Method not found"));
    }

    #[test]
    fn test_error_response_without_id() {
        let err = McpServerError::Auth("test".to_string());
        let resp = McpErrorResponse::from_error(&err, None);
        assert_eq!(resp.id, None);
    }

    #[test]
    fn test_display_implementation() {
        let err = McpServerError::NotFound("test user".to_string());
        let display = format!("{}", err);
        assert!(display.contains("Resource not found"));
        assert!(display.contains("test user"));
    }

    #[test]
    fn test_error_chain_from_api() {
        let api_err = ApiError::BadRequest("invalid name".to_string());
        let err = McpServerError::from(api_err);
        match err {
            McpServerError::Api(api_err) => {
                assert!(format!("{}", api_err).contains("invalid name"));
            }
            _ => panic!("Expected Api variant"),
        }
    }

    #[test]
    fn test_multiple_error_types_different_codes() {
        let errors = vec![
            (McpServerError::InvalidRequest("x".to_string()), -32600),
            (McpServerError::NotFound("x".to_string()), -32001),
            (McpServerError::PermissionDenied("x".to_string()), -32002),
            (McpServerError::Auth("x".to_string()), -32003),
            (McpServerError::Internal("x".to_string()), -32603),
            (McpServerError::Config("x".to_string()), -32603),
        ];
        for (err, expected_code) in errors {
            assert_eq!(
                err.to_mcp_error_code(),
                expected_code,
                "Error type did not map to expected code"
            );
        }
    }

    #[test]
    fn test_json_serialization_roundtrip() {
        let err = McpServerError::NotFound("resource".to_string());
        let resp = McpErrorResponse::from_error(&err, Some(serde_json::json!(42)));
        let json = serde_json::to_string(&resp).unwrap();
        let deserialized: McpErrorResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.error.code, -32001);
        assert_eq!(deserialized.id, Some(serde_json::json!(42)));
    }
}
