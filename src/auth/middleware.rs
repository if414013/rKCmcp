//! Axum middleware for OAuth 2.1 token extraction and validation

use crate::auth::error::AuthError;
use crate::auth::jwt::{extract_bearer_token, Claims, JwtValidator};
use axum::{
    body::Body,
    extract::{Request, State},
    http::{header::AUTHORIZATION, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use std::sync::Arc;

#[derive(Clone)]
pub struct AuthState {
    pub validator: Arc<JwtValidator>,
}

impl AuthState {
    pub fn new(validator: Arc<JwtValidator>) -> Self {
        Self { validator }
    }
}

pub async fn auth_middleware(
    State(state): State<AuthState>,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, AuthError> {
    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or(AuthError::MissingToken)?;

    let token = extract_bearer_token(auth_header)?;

    let claims = state.validator.validate(token).await?;

    request.extensions_mut().insert(claims);

    Ok(next.run(request).await)
}

pub async fn optional_auth_middleware(
    State(state): State<AuthState>,
    mut request: Request<Body>,
    next: Next,
) -> Response {
    if let Some(auth_header) = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
    {
        if let Ok(token) = extract_bearer_token(auth_header) {
            if let Ok(claims) = state.validator.validate(token).await {
                request.extensions_mut().insert(claims);
            }
        }
    }

    next.run(request).await
}

pub fn require_scope(claims: &Claims, scope: &str) -> Result<(), AuthError> {
    if claims.has_scope(scope) {
        Ok(())
    } else {
        Err(AuthError::MissingScope(scope.to_string()))
    }
}

pub fn require_role(claims: &Claims, role: &str) -> Result<(), AuthError> {
    if claims.has_role(role) {
        Ok(())
    } else {
        Err(AuthError::MissingScope(format!("role:{}", role)))
    }
}

pub fn www_authenticate_response(error: &AuthError) -> Response {
    let status = error.status_code();
    let www_auth = error.www_authenticate_header();

    Response::builder()
        .status(status)
        .header("WWW-Authenticate", www_auth)
        .header("Content-Type", "application/json")
        .body(Body::from(
            serde_json::json!({
                "error": "unauthorized",
                "error_description": error.to_string()
            })
            .to_string(),
        ))
        .unwrap_or_else(|_| {
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_claims() -> Claims {
        Claims {
            sub: "user-123".to_string(),
            iss: "http://localhost".to_string(),
            aud: None,
            exp: 9999999999,
            iat: 0,
            scope: Some("openid profile admin".to_string()),
            realm_access: Some(crate::auth::jwt::RealmAccess {
                roles: vec!["user".to_string(), "admin".to_string()],
            }),
            resource_access: None,
            azp: None,
            preferred_username: None,
            email: None,
            name: None,
        }
    }

    #[test]
    fn test_require_scope_success() {
        let claims = make_test_claims();
        assert!(require_scope(&claims, "openid").is_ok());
        assert!(require_scope(&claims, "profile").is_ok());
        assert!(require_scope(&claims, "admin").is_ok());
    }

    #[test]
    fn test_require_scope_failure() {
        let claims = make_test_claims();
        let result = require_scope(&claims, "superadmin");
        assert!(result.is_err());
        match result.unwrap_err() {
            AuthError::MissingScope(scope) => assert_eq!(scope, "superadmin"),
            _ => panic!("Expected MissingScope error"),
        }
    }

    #[test]
    fn test_require_role_success() {
        let claims = make_test_claims();
        assert!(require_role(&claims, "user").is_ok());
        assert!(require_role(&claims, "admin").is_ok());
    }

    #[test]
    fn test_require_role_failure() {
        let claims = make_test_claims();
        let result = require_role(&claims, "superadmin");
        assert!(result.is_err());
    }

    #[test]
    fn test_www_authenticate_response_missing_token() {
        let error = AuthError::MissingToken;
        let response = www_authenticate_response(&error);
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[test]
    fn test_www_authenticate_response_missing_scope() {
        let error = AuthError::MissingScope("admin".to_string());
        let response = www_authenticate_response(&error);
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }
}
