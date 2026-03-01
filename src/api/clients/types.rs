//! Type definitions for Keycloak Client resources.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Keycloak Client representation.
///
/// Represents an OAuth 2.0 client in Keycloak. Clients are entities that can request
/// authentication of a user. Clients come in two forms - public and confidential.
#[derive(Debug, Clone, Serialize, Deserialize, Default, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClientRepresentation {
    /// Unique identifier for the client (UUID)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Client ID used in authentication flows (e.g., "my-app")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,

    /// Display name of the client
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Description of the client
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the client is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// Protocol used (openid-connect, saml)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,

    /// If true, client is public (no client secret required)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_client: Option<bool>,

    /// If true, client will only use bearer tokens (no auth redirects)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bearer_only: Option<bool>,

    /// If true, users must consent to client access
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent_required: Option<bool>,

    /// Enable standard OpenID Connect redirect-based authentication
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard_flow_enabled: Option<bool>,

    /// Enable implicit flow (deprecated in OAuth 2.1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implicit_flow_enabled: Option<bool>,

    /// Enable direct access grants (Resource Owner Password Credentials)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_access_grants_enabled: Option<bool>,

    /// Enable service accounts for this client
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_accounts_enabled: Option<bool>,

    /// Enable authorization services (fine-grained permissions)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_services_enabled: Option<bool>,

    /// Root URL prepended to relative URLs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_url: Option<String>,

    /// Valid redirect URIs for this client
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_uris: Option<Vec<String>>,

    /// Allowed CORS origins
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_origins: Option<Vec<String>>,

    /// Admin URL for Keycloak to call back to the client
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_url: Option<String>,

    /// Default URL used when linking to this client
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,

    /// Client secret (confidential clients only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,

    /// Default client scopes added to all tokens
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_client_scopes: Option<Vec<String>>,

    /// Optional client scopes that can be requested
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional_client_scopes: Option<Vec<String>>,

    /// Additional client attributes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<HashMap<String, String>>,

    /// Whether full scope is allowed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_scope_allowed: Option<bool>,

    /// Whether surrogate auth is required
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surrogate_auth_required: Option<bool>,

    /// Client authenticator type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_authenticator_type: Option<String>,

    /// Front channel logout enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frontchannel_logout: Option<bool>,

    /// Node re-registration timeout
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_re_registration_timeout: Option<i32>,

    /// Not before time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_before: Option<i32>,
}

/// Parameters for listing clients.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClientListParams {
    /// The realm name
    pub realm: String,

    /// Filter by client ID (exact match unless search=true)
    #[serde(default)]
    pub client_id: Option<String>,

    /// If true, perform substring search on client ID
    #[serde(default)]
    pub search: Option<bool>,

    /// Enable viewing of clients (viewable_only)
    #[serde(default)]
    pub viewable_only: Option<bool>,

    /// Pagination: first result index
    #[serde(default)]
    pub first: Option<i32>,

    /// Pagination: max results to return
    #[serde(default)]
    pub max: Option<i32>,
}

/// Parameters for getting a single client.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClientGetParams {
    /// The realm name
    pub realm: String,

    /// The client UUID (not client_id)
    pub id: String,
}

/// Parameters for creating a client.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClientCreateParams {
    /// The realm name
    pub realm: String,

    /// The client representation to create
    pub client: ClientRepresentation,
}

/// Parameters for updating a client.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClientUpdateParams {
    /// The realm name
    pub realm: String,

    /// The client UUID (not client_id)
    pub id: String,

    /// The updated client representation
    pub client: ClientRepresentation,
}

/// Parameters for deleting a client.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClientDeleteParams {
    /// The realm name
    pub realm: String,

    /// The client UUID (not client_id)
    pub id: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_representation_default() {
        let client = ClientRepresentation::default();
        assert!(client.id.is_none());
        assert!(client.client_id.is_none());
        assert!(client.enabled.is_none());
    }

    #[test]
    fn test_client_representation_serialization() {
        let client = ClientRepresentation {
            client_id: Some("my-app".to_string()),
            enabled: Some(true),
            public_client: Some(false),
            ..Default::default()
        };

        let json = serde_json::to_string(&client).expect("Failed to serialize");
        assert!(json.contains("\"clientId\":\"my-app\""));
        assert!(json.contains("\"enabled\":true"));
        assert!(json.contains("\"publicClient\":false"));
        assert!(!json.contains("\"id\""));
    }

    #[test]
    fn test_client_representation_deserialization() {
        let json = r#"{
            "id": "abc-123",
            "clientId": "my-client",
            "name": "My Client",
            "enabled": true,
            "protocol": "openid-connect",
            "publicClient": true,
            "standardFlowEnabled": true,
            "directAccessGrantsEnabled": false,
            "serviceAccountsEnabled": false,
            "redirectUris": ["http://localhost:8080/*"],
            "webOrigins": ["+"]
        }"#;

        let client: ClientRepresentation =
            serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(client.id, Some("abc-123".to_string()));
        assert_eq!(client.client_id, Some("my-client".to_string()));
        assert_eq!(client.name, Some("My Client".to_string()));
        assert_eq!(client.enabled, Some(true));
        assert_eq!(client.protocol, Some("openid-connect".to_string()));
        assert_eq!(client.public_client, Some(true));
        assert_eq!(client.standard_flow_enabled, Some(true));
        assert_eq!(client.direct_access_grants_enabled, Some(false));
        assert_eq!(client.service_accounts_enabled, Some(false));
        assert_eq!(
            client.redirect_uris,
            Some(vec!["http://localhost:8080/*".to_string()])
        );
        assert_eq!(client.web_origins, Some(vec!["+".to_string()]));
    }

    #[test]
    fn test_client_representation_with_attributes() {
        let mut attrs = HashMap::new();
        attrs.insert("custom.key".to_string(), "custom.value".to_string());

        let client = ClientRepresentation {
            client_id: Some("app-with-attrs".to_string()),
            attributes: Some(attrs),
            ..Default::default()
        };

        let json = serde_json::to_string(&client).expect("Failed to serialize");
        assert!(json.contains("\"attributes\""));
        assert!(json.contains("\"custom.key\":\"custom.value\""));
    }

    #[test]
    fn test_client_list_params_deserialization() {
        let json = r#"{
            "realm": "master",
            "clientId": "my-app",
            "search": true,
            "first": 0,
            "max": 100
        }"#;

        let params: ClientListParams = serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(params.realm, "master");
        assert_eq!(params.client_id, Some("my-app".to_string()));
        assert_eq!(params.search, Some(true));
        assert_eq!(params.first, Some(0));
        assert_eq!(params.max, Some(100));
    }

    #[test]
    fn test_client_list_params_minimal() {
        let json = r#"{"realm": "test-realm"}"#;

        let params: ClientListParams = serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(params.realm, "test-realm");
        assert!(params.client_id.is_none());
        assert!(params.search.is_none());
        assert!(params.first.is_none());
        assert!(params.max.is_none());
    }

    #[test]
    fn test_client_get_params_deserialization() {
        let json = r#"{
            "realm": "master",
            "id": "abc-123-def"
        }"#;

        let params: ClientGetParams = serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(params.realm, "master");
        assert_eq!(params.id, "abc-123-def");
    }

    #[test]
    fn test_client_create_params_deserialization() {
        let json = r#"{
            "realm": "master",
            "client": {
                "clientId": "new-app",
                "enabled": true
            }
        }"#;

        let params: ClientCreateParams = serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(params.realm, "master");
        assert_eq!(params.client.client_id, Some("new-app".to_string()));
        assert_eq!(params.client.enabled, Some(true));
    }

    #[test]
    fn test_client_update_params_deserialization() {
        let json = r#"{
            "realm": "master",
            "id": "abc-123",
            "client": {
                "clientId": "updated-app",
                "name": "Updated App Name"
            }
        }"#;

        let params: ClientUpdateParams = serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(params.realm, "master");
        assert_eq!(params.id, "abc-123");
        assert_eq!(params.client.client_id, Some("updated-app".to_string()));
        assert_eq!(params.client.name, Some("Updated App Name".to_string()));
    }

    #[test]
    fn test_client_delete_params_deserialization() {
        let json = r#"{
            "realm": "master",
            "id": "abc-123-to-delete"
        }"#;

        let params: ClientDeleteParams = serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(params.realm, "master");
        assert_eq!(params.id, "abc-123-to-delete");
    }
}
