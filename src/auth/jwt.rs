//! JWT validation for OAuth 2.1 resource server

use crate::auth::error::AuthError;
use crate::auth::jwks::JwksCache;
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Deserializer, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StringOrVec {
    Single(String),
    Multiple(Vec<String>),
}

impl StringOrVec {
    pub fn contains(&self, value: &str) -> bool {
        match self {
            StringOrVec::Single(s) => s == value,
            StringOrVec::Multiple(v) => v.iter().any(|s| s == value),
        }
    }

    pub fn as_vec(&self) -> Vec<String> {
        match self {
            StringOrVec::Single(s) => vec![s.clone()],
            StringOrVec::Multiple(v) => v.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealmAccess {
    pub roles: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceAccess {
    #[serde(flatten)]
    pub clients: std::collections::HashMap<String, RealmAccess>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub iss: String,
    #[serde(default, deserialize_with = "deserialize_aud")]
    pub aud: Option<StringOrVec>,
    pub exp: i64,
    pub iat: i64,
    #[serde(default)]
    pub scope: Option<String>,
    #[serde(default)]
    pub realm_access: Option<RealmAccess>,
    #[serde(default)]
    pub resource_access: Option<ResourceAccess>,
    #[serde(default)]
    pub azp: Option<String>,
    #[serde(default)]
    pub preferred_username: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
}

fn deserialize_aud<'de, D>(deserializer: D) -> Result<Option<StringOrVec>, D::Error>
where
    D: Deserializer<'de>,
{
    Option::<StringOrVec>::deserialize(deserializer)
}

impl Claims {
    pub fn has_scope(&self, required_scope: &str) -> bool {
        self.scope
            .as_ref()
            .map(|s| s.split_whitespace().any(|scope| scope == required_scope))
            .unwrap_or(false)
    }

    pub fn has_role(&self, role: &str) -> bool {
        self.realm_access
            .as_ref()
            .map(|ra| ra.roles.iter().any(|r| r == role))
            .unwrap_or(false)
    }

    pub fn has_client_role(&self, client: &str, role: &str) -> bool {
        self.resource_access
            .as_ref()
            .and_then(|ra| ra.clients.get(client))
            .map(|client_access| client_access.roles.iter().any(|r| r == role))
            .unwrap_or(false)
    }
}

#[derive(Debug, Clone)]
pub struct JwtValidatorConfig {
    pub issuer: String,
    pub audience: Option<String>,
    pub leeway_seconds: u64,
    pub required_scopes: Vec<String>,
}

impl JwtValidatorConfig {
    pub fn new(issuer: String) -> Self {
        Self {
            issuer,
            audience: None,
            leeway_seconds: 60,
            required_scopes: Vec::new(),
        }
    }

    pub fn with_audience(mut self, audience: String) -> Self {
        self.audience = Some(audience);
        self
    }

    pub fn with_leeway(mut self, seconds: u64) -> Self {
        self.leeway_seconds = seconds;
        self
    }

    pub fn with_required_scopes(mut self, scopes: Vec<String>) -> Self {
        self.required_scopes = scopes;
        self
    }
}

pub struct JwtValidator {
    config: JwtValidatorConfig,
    jwks_cache: Arc<JwksCache>,
}

impl JwtValidator {
    pub fn new(config: JwtValidatorConfig, jwks_cache: Arc<JwksCache>) -> Self {
        Self { config, jwks_cache }
    }

    pub async fn validate(&self, token: &str) -> Result<Claims, AuthError> {
        let header = decode_header(token).map_err(|_| AuthError::InvalidTokenFormat)?;

        let kid = header.kid.ok_or(AuthError::InvalidTokenFormat)?;

        let jwk = self.jwks_cache.get_key(&kid).await?;

        let decoding_key = DecodingKey::from_jwk(&jwk)
            .map_err(|e| AuthError::Internal(format!("Failed to create decoding key: {}", e)))?;

        let algorithm = match header.alg {
            jsonwebtoken::Algorithm::RS256 => Algorithm::RS256,
            jsonwebtoken::Algorithm::RS384 => Algorithm::RS384,
            jsonwebtoken::Algorithm::RS512 => Algorithm::RS512,
            jsonwebtoken::Algorithm::ES256 => Algorithm::ES256,
            jsonwebtoken::Algorithm::ES384 => Algorithm::ES384,
            _ => return Err(AuthError::InvalidTokenFormat),
        };

        let mut validation = Validation::new(algorithm);
        validation.set_issuer(&[&self.config.issuer]);
        validation.leeway = self.config.leeway_seconds;

        if let Some(ref aud) = self.config.audience {
            validation.set_audience(&[aud]);
        } else {
            validation.validate_aud = false;
        }

        let token_data =
            decode::<Claims>(token, &decoding_key, &validation).map_err(|e| match e.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthError::TokenExpired,
                jsonwebtoken::errors::ErrorKind::InvalidSignature => AuthError::InvalidSignature,
                jsonwebtoken::errors::ErrorKind::InvalidIssuer => AuthError::InvalidIssuer,
                jsonwebtoken::errors::ErrorKind::InvalidAudience => AuthError::InvalidAudience,
                _ => AuthError::InvalidTokenFormat,
            })?;

        for required_scope in &self.config.required_scopes {
            if !token_data.claims.has_scope(required_scope) {
                return Err(AuthError::MissingScope(required_scope.clone()));
            }
        }

        Ok(token_data.claims)
    }
}

pub fn extract_bearer_token(auth_header: &str) -> Result<&str, AuthError> {
    let parts: Vec<&str> = auth_header.splitn(2, ' ').collect();
    if parts.len() != 2 {
        return Err(AuthError::InvalidTokenFormat);
    }
    if !parts[0].eq_ignore_ascii_case("bearer") {
        return Err(AuthError::InvalidTokenFormat);
    }
    Ok(parts[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_or_vec_single() {
        let sov = StringOrVec::Single("test".to_string());
        assert!(sov.contains("test"));
        assert!(!sov.contains("other"));
        assert_eq!(sov.as_vec(), vec!["test".to_string()]);
    }

    #[test]
    fn test_string_or_vec_multiple() {
        let sov = StringOrVec::Multiple(vec!["a".to_string(), "b".to_string()]);
        assert!(sov.contains("a"));
        assert!(sov.contains("b"));
        assert!(!sov.contains("c"));
        assert_eq!(sov.as_vec(), vec!["a".to_string(), "b".to_string()]);
    }

    #[test]
    fn test_claims_has_scope() {
        let claims = Claims {
            sub: "user1".to_string(),
            iss: "http://localhost".to_string(),
            aud: None,
            exp: 9999999999,
            iat: 0,
            scope: Some("openid profile email".to_string()),
            realm_access: None,
            resource_access: None,
            azp: None,
            preferred_username: None,
            email: None,
            name: None,
        };

        assert!(claims.has_scope("openid"));
        assert!(claims.has_scope("profile"));
        assert!(claims.has_scope("email"));
        assert!(!claims.has_scope("admin"));
    }

    #[test]
    fn test_claims_has_role() {
        let claims = Claims {
            sub: "user1".to_string(),
            iss: "http://localhost".to_string(),
            aud: None,
            exp: 9999999999,
            iat: 0,
            scope: None,
            realm_access: Some(RealmAccess {
                roles: vec!["admin".to_string(), "user".to_string()],
            }),
            resource_access: None,
            azp: None,
            preferred_username: None,
            email: None,
            name: None,
        };

        assert!(claims.has_role("admin"));
        assert!(claims.has_role("user"));
        assert!(!claims.has_role("superadmin"));
    }

    #[test]
    fn test_claims_has_client_role() {
        let mut clients = std::collections::HashMap::new();
        clients.insert(
            "my-client".to_string(),
            RealmAccess {
                roles: vec!["client-admin".to_string()],
            },
        );

        let claims = Claims {
            sub: "user1".to_string(),
            iss: "http://localhost".to_string(),
            aud: None,
            exp: 9999999999,
            iat: 0,
            scope: None,
            realm_access: None,
            resource_access: Some(ResourceAccess { clients }),
            azp: None,
            preferred_username: None,
            email: None,
            name: None,
        };

        assert!(claims.has_client_role("my-client", "client-admin"));
        assert!(!claims.has_client_role("my-client", "other-role"));
        assert!(!claims.has_client_role("other-client", "client-admin"));
    }

    #[test]
    fn test_extract_bearer_token() {
        assert_eq!(extract_bearer_token("Bearer abc123").unwrap(), "abc123");
        assert_eq!(
            extract_bearer_token("bearer token.with.dots").unwrap(),
            "token.with.dots"
        );
        assert!(extract_bearer_token("Basic abc123").is_err());
        assert!(extract_bearer_token("Bearertoken").is_err());
        assert!(extract_bearer_token("").is_err());
    }

    #[test]
    fn test_jwt_validator_config_builder() {
        let config = JwtValidatorConfig::new("http://localhost/realms/test".to_string())
            .with_audience("my-app".to_string())
            .with_leeway(30)
            .with_required_scopes(vec!["openid".to_string()]);

        assert_eq!(config.issuer, "http://localhost/realms/test");
        assert_eq!(config.audience, Some("my-app".to_string()));
        assert_eq!(config.leeway_seconds, 30);
        assert_eq!(config.required_scopes, vec!["openid".to_string()]);
    }

    #[test]
    fn test_claims_deserialization() {
        let json = r#"{
            "sub": "user-123",
            "iss": "http://keycloak/realms/test",
            "aud": "my-client",
            "exp": 9999999999,
            "iat": 1000000000,
            "scope": "openid profile",
            "preferred_username": "testuser"
        }"#;

        let claims: Claims = serde_json::from_str(json).unwrap();
        assert_eq!(claims.sub, "user-123");
        assert_eq!(claims.iss, "http://keycloak/realms/test");
        assert!(claims.has_scope("openid"));
        assert_eq!(claims.preferred_username, Some("testuser".to_string()));
    }

    #[test]
    fn test_claims_deserialization_array_audience() {
        let json = r#"{
            "sub": "user-123",
            "iss": "http://keycloak/realms/test",
            "aud": ["client-a", "client-b"],
            "exp": 9999999999,
            "iat": 1000000000
        }"#;

        let claims: Claims = serde_json::from_str(json).unwrap();
        assert!(claims.aud.as_ref().unwrap().contains("client-a"));
        assert!(claims.aud.as_ref().unwrap().contains("client-b"));
    }
}
