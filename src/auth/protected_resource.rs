//! RFC 9728 OAuth 2.0 Protected Resource Metadata endpoint

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtectedResourceMetadata {
    pub resource: String,
    pub authorization_servers: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub scopes_supported: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub bearer_methods_supported: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_documentation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_signing_alg_values_supported: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_policy_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_tos_uri: Option<String>,
}

impl ProtectedResourceMetadata {
    pub fn new(resource: String, authorization_server: String) -> Self {
        Self {
            resource,
            authorization_servers: vec![authorization_server],
            scopes_supported: Vec::new(),
            bearer_methods_supported: vec!["header".to_string()],
            resource_documentation: None,
            resource_signing_alg_values_supported: None,
            resource_policy_uri: None,
            resource_tos_uri: None,
        }
    }

    pub fn with_scopes(mut self, scopes: Vec<String>) -> Self {
        self.scopes_supported = scopes;
        self
    }

    pub fn with_bearer_methods(mut self, methods: Vec<String>) -> Self {
        self.bearer_methods_supported = methods;
        self
    }

    pub fn with_documentation(mut self, url: String) -> Self {
        self.resource_documentation = Some(url);
        self
    }

    pub fn with_signing_algs(mut self, algs: Vec<String>) -> Self {
        self.resource_signing_alg_values_supported = Some(algs);
        self
    }

    pub fn add_authorization_server(mut self, server: String) -> Self {
        self.authorization_servers.push(server);
        self
    }
}

#[derive(Clone)]
pub struct ProtectedResourceState {
    pub metadata: ProtectedResourceMetadata,
}

impl ProtectedResourceState {
    pub fn new(metadata: ProtectedResourceMetadata) -> Self {
        Self { metadata }
    }
}

pub async fn protected_resource_metadata_handler(
    State(state): State<ProtectedResourceState>,
) -> impl IntoResponse {
    (
        StatusCode::OK,
        [("Content-Type", "application/json")],
        Json(state.metadata),
    )
}

pub fn build_protected_resource_metadata(
    resource_url: &str,
    keycloak_url: &str,
    realm: &str,
    scopes: Vec<String>,
) -> ProtectedResourceMetadata {
    let issuer = crate::auth::jwks::build_issuer_url(keycloak_url, realm);

    ProtectedResourceMetadata::new(resource_url.to_string(), issuer)
        .with_scopes(scopes)
        .with_bearer_methods(vec!["header".to_string()])
        .with_signing_algs(vec![
            "RS256".to_string(),
            "RS384".to_string(),
            "RS512".to_string(),
            "ES256".to_string(),
            "ES384".to_string(),
        ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protected_resource_metadata_creation() {
        let metadata = ProtectedResourceMetadata::new(
            "https://api.example.com".to_string(),
            "https://keycloak.example.com/realms/test".to_string(),
        );

        assert_eq!(metadata.resource, "https://api.example.com");
        assert_eq!(
            metadata.authorization_servers,
            vec!["https://keycloak.example.com/realms/test"]
        );
        assert_eq!(metadata.bearer_methods_supported, vec!["header"]);
    }

    #[test]
    fn test_protected_resource_metadata_builder() {
        let metadata = ProtectedResourceMetadata::new(
            "https://api.example.com".to_string(),
            "https://keycloak.example.com/realms/test".to_string(),
        )
        .with_scopes(vec!["openid".to_string(), "profile".to_string()])
        .with_bearer_methods(vec!["header".to_string(), "body".to_string()])
        .with_documentation("https://docs.example.com".to_string())
        .with_signing_algs(vec!["RS256".to_string()]);

        assert_eq!(
            metadata.scopes_supported,
            vec!["openid".to_string(), "profile".to_string()]
        );
        assert_eq!(
            metadata.bearer_methods_supported,
            vec!["header".to_string(), "body".to_string()]
        );
        assert_eq!(
            metadata.resource_documentation,
            Some("https://docs.example.com".to_string())
        );
        assert_eq!(
            metadata.resource_signing_alg_values_supported,
            Some(vec!["RS256".to_string()])
        );
    }

    #[test]
    fn test_add_authorization_server() {
        let metadata = ProtectedResourceMetadata::new(
            "https://api.example.com".to_string(),
            "https://keycloak1.example.com/realms/test".to_string(),
        )
        .add_authorization_server("https://keycloak2.example.com/realms/test".to_string());

        assert_eq!(metadata.authorization_servers.len(), 2);
    }

    #[test]
    fn test_build_protected_resource_metadata() {
        let metadata = build_protected_resource_metadata(
            "https://api.example.com",
            "https://keycloak.example.com",
            "test",
            vec!["openid".to_string(), "profile".to_string()],
        );

        assert_eq!(metadata.resource, "https://api.example.com");
        assert_eq!(
            metadata.authorization_servers,
            vec!["https://keycloak.example.com/realms/test"]
        );
        assert_eq!(
            metadata.scopes_supported,
            vec!["openid".to_string(), "profile".to_string()]
        );
        assert!(metadata
            .resource_signing_alg_values_supported
            .as_ref()
            .unwrap()
            .contains(&"RS256".to_string()));
    }

    #[test]
    fn test_metadata_serialization() {
        let metadata = ProtectedResourceMetadata::new(
            "https://api.example.com".to_string(),
            "https://keycloak.example.com/realms/test".to_string(),
        )
        .with_scopes(vec!["openid".to_string()]);

        let json = serde_json::to_string(&metadata).unwrap();

        assert!(json.contains("\"resource\":\"https://api.example.com\""));
        assert!(json
            .contains("\"authorization_servers\":[\"https://keycloak.example.com/realms/test\"]"));
        assert!(json.contains("\"scopes_supported\":[\"openid\"]"));
    }

    #[test]
    fn test_metadata_serialization_omits_empty_fields() {
        let metadata = ProtectedResourceMetadata::new(
            "https://api.example.com".to_string(),
            "https://keycloak.example.com/realms/test".to_string(),
        );

        let json = serde_json::to_string(&metadata).unwrap();

        assert!(!json.contains("resource_documentation"));
        assert!(!json.contains("resource_policy_uri"));
        assert!(!json.contains("resource_tos_uri"));
    }

    #[test]
    fn test_metadata_deserialization() {
        let json = r#"{
            "resource": "https://api.example.com",
            "authorization_servers": ["https://keycloak.example.com/realms/test"],
            "scopes_supported": ["openid", "profile"],
            "bearer_methods_supported": ["header"]
        }"#;

        let metadata: ProtectedResourceMetadata = serde_json::from_str(json).unwrap();

        assert_eq!(metadata.resource, "https://api.example.com");
        assert_eq!(metadata.scopes_supported.len(), 2);
    }
}
