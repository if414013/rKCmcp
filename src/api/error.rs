//! API error types for Keycloak HTTP client operations.
//!
//! This module defines error types for handling Keycloak Admin API responses,
//! including HTTP errors, authentication failures, and Keycloak-specific error messages.

use serde::Deserialize;
use thiserror::Error;

/// Keycloak error response structure.
///
/// Keycloak returns errors in JSON format with `error` and `error_description` fields.
#[derive(Debug, Deserialize)]
pub struct KeycloakErrorResponse {
    /// Error code or type
    #[serde(default)]
    pub error: Option<String>,
    /// Human-readable error description
    #[serde(default)]
    pub error_description: Option<String>,
    /// Alternative error message field used by some endpoints
    #[serde(default, rename = "errorMessage")]
    pub error_message: Option<String>,
}

impl KeycloakErrorResponse {
    /// Get the error message, preferring error_description over error_message over error
    pub fn message(&self) -> String {
        self.error_description
            .clone()
            .or_else(|| self.error_message.clone())
            .or_else(|| self.error.clone())
            .unwrap_or_else(|| "Unknown error".to_string())
    }
}

/// API errors that can occur when communicating with Keycloak.
#[derive(Debug, Error)]
pub enum ApiError {
    /// HTTP transport error from reqwest
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),

    /// Resource not found (404)
    #[error("Resource not found")]
    NotFound,

    /// Unauthorized - invalid or missing token (401)
    #[error("Unauthorized: authentication required")]
    Unauthorized,

    /// Forbidden - insufficient permissions (403)
    #[error("Forbidden: insufficient permissions")]
    Forbidden,

    /// Conflict - resource already exists or state conflict (409)
    #[error("Conflict: {0}")]
    Conflict(String),

    /// Bad request - validation error or malformed request (400)
    #[error("Bad request: {0}")]
    BadRequest(String),

    /// Server error - Keycloak internal error (5xx)
    #[error("Server error: {0}")]
    ServerError(String),

    /// JSON serialization/deserialization error
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}

impl ApiError {
    /// Create an ApiError from an HTTP status code and optional Keycloak error response.
    ///
    /// This method maps HTTP status codes to appropriate ApiError variants,
    /// extracting error details from Keycloak's error response format.
    pub fn from_response(
        status: reqwest::StatusCode,
        error_response: Option<KeycloakErrorResponse>,
    ) -> Self {
        let message = error_response.map(|e| e.message()).unwrap_or_else(|| {
            status
                .canonical_reason()
                .unwrap_or("Unknown error")
                .to_string()
        });

        match status.as_u16() {
            400 => ApiError::BadRequest(message),
            401 => ApiError::Unauthorized,
            403 => ApiError::Forbidden,
            404 => ApiError::NotFound,
            409 => ApiError::Conflict(message),
            500..=599 => ApiError::ServerError(message),
            _ => ApiError::ServerError(format!(
                "Unexpected status {}: {}",
                status.as_u16(),
                message
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::StatusCode;

    #[test]
    fn test_keycloak_error_response_message_prefers_description() {
        let response = KeycloakErrorResponse {
            error: Some("error_code".to_string()),
            error_description: Some("Detailed description".to_string()),
            error_message: Some("Error message".to_string()),
        };
        assert_eq!(response.message(), "Detailed description");
    }

    #[test]
    fn test_keycloak_error_response_message_falls_back_to_error_message() {
        let response = KeycloakErrorResponse {
            error: Some("error_code".to_string()),
            error_description: None,
            error_message: Some("Error message".to_string()),
        };
        assert_eq!(response.message(), "Error message");
    }

    #[test]
    fn test_keycloak_error_response_message_falls_back_to_error() {
        let response = KeycloakErrorResponse {
            error: Some("error_code".to_string()),
            error_description: None,
            error_message: None,
        };
        assert_eq!(response.message(), "error_code");
    }

    #[test]
    fn test_keycloak_error_response_message_default() {
        let response = KeycloakErrorResponse {
            error: None,
            error_description: None,
            error_message: None,
        };
        assert_eq!(response.message(), "Unknown error");
    }

    #[test]
    fn test_api_error_from_response_bad_request() {
        let error = ApiError::from_response(
            StatusCode::BAD_REQUEST,
            Some(KeycloakErrorResponse {
                error: None,
                error_description: Some("Invalid user data".to_string()),
                error_message: None,
            }),
        );
        match error {
            ApiError::BadRequest(msg) => assert_eq!(msg, "Invalid user data"),
            _ => panic!("Expected BadRequest"),
        }
    }

    #[test]
    fn test_api_error_from_response_unauthorized() {
        let error = ApiError::from_response(StatusCode::UNAUTHORIZED, None);
        assert!(matches!(error, ApiError::Unauthorized));
    }

    #[test]
    fn test_api_error_from_response_forbidden() {
        let error = ApiError::from_response(StatusCode::FORBIDDEN, None);
        assert!(matches!(error, ApiError::Forbidden));
    }

    #[test]
    fn test_api_error_from_response_not_found() {
        let error = ApiError::from_response(StatusCode::NOT_FOUND, None);
        assert!(matches!(error, ApiError::NotFound));
    }

    #[test]
    fn test_api_error_from_response_conflict() {
        let error = ApiError::from_response(
            StatusCode::CONFLICT,
            Some(KeycloakErrorResponse {
                error: None,
                error_description: Some("User already exists".to_string()),
                error_message: None,
            }),
        );
        match error {
            ApiError::Conflict(msg) => assert_eq!(msg, "User already exists"),
            _ => panic!("Expected Conflict"),
        }
    }

    #[test]
    fn test_api_error_from_response_server_error() {
        let error = ApiError::from_response(StatusCode::INTERNAL_SERVER_ERROR, None);
        assert!(matches!(error, ApiError::ServerError(_)));
    }

    #[test]
    fn test_api_error_display() {
        let error = ApiError::NotFound;
        assert_eq!(format!("{}", error), "Resource not found");

        let error = ApiError::BadRequest("Invalid input".to_string());
        assert_eq!(format!("{}", error), "Bad request: Invalid input");
    }
}
