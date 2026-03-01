//! Authentication error types
//!
//! This module defines error types specific to OAuth 2.1 resource server authentication,
//! including JWT validation errors and JWKS fetching errors.

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use thiserror::Error;

/// Authentication and authorization error types
#[derive(Debug, Error)]
pub enum AuthError {
    /// Missing authorization header in request
    #[error("Missing authorization header")]
    MissingToken,

    /// Invalid token format (not Bearer, malformed, etc.)
    #[error("Invalid token format")]
    InvalidTokenFormat,

    /// Token has expired (exp claim in the past)
    #[error("Token expired")]
    TokenExpired,

    /// Token signature verification failed
    #[error("Invalid signature")]
    InvalidSignature,

    /// Token issuer (iss claim) doesn't match expected value
    #[error("Invalid issuer")]
    InvalidIssuer,

    /// Token audience (aud claim) doesn't match expected value
    #[error("Invalid audience")]
    InvalidAudience,

    /// Required scope is missing from the token
    #[error("Missing required scope: {0}")]
    MissingScope(String),

    /// Error fetching JWKS from authorization server
    #[error("JWKS fetch error: {0}")]
    JwksFetchError(String),

    /// Key ID (kid) not found in JWKS
    #[error("Key not found: {0}")]
    KeyNotFound(String),

    /// Internal error during validation
    #[error("Internal error: {0}")]
    Internal(String),
}

impl AuthError {
    /// Get the WWW-Authenticate header value for this error
    ///
    /// As per RFC 6750, the WWW-Authenticate header should indicate
    /// the type of authentication required and the error that occurred.
    pub fn www_authenticate_header(&self) -> String {
        let error_code = match self {
            AuthError::MissingToken => return "Bearer".to_string(),
            AuthError::InvalidTokenFormat => "invalid_token",
            AuthError::TokenExpired => "invalid_token",
            AuthError::InvalidSignature => "invalid_token",
            AuthError::InvalidIssuer => "invalid_token",
            AuthError::InvalidAudience => "invalid_token",
            AuthError::MissingScope(scope) => {
                return format!(
                    "Bearer error=\"insufficient_scope\", error_description=\"Missing required scope: {}\"",
                    scope
                );
            }
            AuthError::JwksFetchError(_) => "invalid_token",
            AuthError::KeyNotFound(_) => "invalid_token",
            AuthError::Internal(_) => "invalid_token",
        };

        format!(
            "Bearer error=\"{}\", error_description=\"{}\"",
            error_code,
            self.to_string().replace('"', "'")
        )
    }

    /// Get the appropriate HTTP status code for this error
    pub fn status_code(&self) -> StatusCode {
        match self {
            AuthError::MissingToken => StatusCode::UNAUTHORIZED,
            AuthError::InvalidTokenFormat => StatusCode::UNAUTHORIZED,
            AuthError::TokenExpired => StatusCode::UNAUTHORIZED,
            AuthError::InvalidSignature => StatusCode::UNAUTHORIZED,
            AuthError::InvalidIssuer => StatusCode::UNAUTHORIZED,
            AuthError::InvalidAudience => StatusCode::UNAUTHORIZED,
            AuthError::MissingScope(_) => StatusCode::FORBIDDEN,
            AuthError::JwksFetchError(_) => StatusCode::SERVICE_UNAVAILABLE,
            AuthError::KeyNotFound(_) => StatusCode::UNAUTHORIZED,
            AuthError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

/// Error response body for authentication failures
#[derive(Debug, Serialize)]
pub struct AuthErrorResponse {
    pub error: String,
    pub error_description: String,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let www_authenticate = self.www_authenticate_header();

        let body = AuthErrorResponse {
            error: match &self {
                AuthError::MissingScope(_) => "insufficient_scope".to_string(),
                _ => "invalid_token".to_string(),
            },
            error_description: self.to_string(),
        };

        let body_json = serde_json::to_string(&body).unwrap_or_else(|_| {
            r#"{"error":"internal_error","error_description":"Failed to serialize error"}"#
                .to_string()
        });

        Response::builder()
            .status(status)
            .header("WWW-Authenticate", www_authenticate)
            .header("Content-Type", "application/json")
            .body(axum::body::Body::from(body_json))
            .unwrap_or_else(|_| {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_missing_token_error() {
        let err = AuthError::MissingToken;
        assert_eq!(err.status_code(), StatusCode::UNAUTHORIZED);
        assert_eq!(err.www_authenticate_header(), "Bearer");
        assert!(err.to_string().contains("Missing authorization header"));
    }

    #[test]
    fn test_invalid_token_format_error() {
        let err = AuthError::InvalidTokenFormat;
        assert_eq!(err.status_code(), StatusCode::UNAUTHORIZED);
        assert!(err.www_authenticate_header().contains("invalid_token"));
    }

    #[test]
    fn test_token_expired_error() {
        let err = AuthError::TokenExpired;
        assert_eq!(err.status_code(), StatusCode::UNAUTHORIZED);
        assert!(err.www_authenticate_header().contains("invalid_token"));
        assert!(err.to_string().contains("expired"));
    }

    #[test]
    fn test_invalid_signature_error() {
        let err = AuthError::InvalidSignature;
        assert_eq!(err.status_code(), StatusCode::UNAUTHORIZED);
        assert!(err.www_authenticate_header().contains("invalid_token"));
    }

    #[test]
    fn test_invalid_issuer_error() {
        let err = AuthError::InvalidIssuer;
        assert_eq!(err.status_code(), StatusCode::UNAUTHORIZED);
        assert!(err.www_authenticate_header().contains("invalid_token"));
    }

    #[test]
    fn test_invalid_audience_error() {
        let err = AuthError::InvalidAudience;
        assert_eq!(err.status_code(), StatusCode::UNAUTHORIZED);
        assert!(err.www_authenticate_header().contains("invalid_token"));
    }

    #[test]
    fn test_missing_scope_error() {
        let err = AuthError::MissingScope("admin".to_string());
        assert_eq!(err.status_code(), StatusCode::FORBIDDEN);
        let header = err.www_authenticate_header();
        assert!(header.contains("insufficient_scope"));
        assert!(header.contains("admin"));
    }

    #[test]
    fn test_jwks_fetch_error() {
        let err = AuthError::JwksFetchError("connection timeout".to_string());
        assert_eq!(err.status_code(), StatusCode::SERVICE_UNAVAILABLE);
        assert!(err.to_string().contains("connection timeout"));
    }

    #[test]
    fn test_key_not_found_error() {
        let err = AuthError::KeyNotFound("key123".to_string());
        assert_eq!(err.status_code(), StatusCode::UNAUTHORIZED);
        assert!(err.to_string().contains("key123"));
    }

    #[test]
    fn test_internal_error() {
        let err = AuthError::Internal("unexpected state".to_string());
        assert_eq!(err.status_code(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_error_display_implementation() {
        let errors = vec![
            (AuthError::MissingToken, "Missing authorization header"),
            (AuthError::InvalidTokenFormat, "Invalid token format"),
            (AuthError::TokenExpired, "Token expired"),
            (AuthError::InvalidSignature, "Invalid signature"),
            (AuthError::InvalidIssuer, "Invalid issuer"),
            (AuthError::InvalidAudience, "Invalid audience"),
            (
                AuthError::MissingScope("read".to_string()),
                "Missing required scope: read",
            ),
            (
                AuthError::JwksFetchError("network".to_string()),
                "JWKS fetch error: network",
            ),
            (
                AuthError::KeyNotFound("abc".to_string()),
                "Key not found: abc",
            ),
            (
                AuthError::Internal("error".to_string()),
                "Internal error: error",
            ),
        ];

        for (err, expected) in errors {
            assert_eq!(err.to_string(), expected);
        }
    }
}
