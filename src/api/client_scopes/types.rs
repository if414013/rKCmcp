//! Type definitions for Keycloak Client Scopes API.

use crate::api::realms::ClientScopeRepresentation;
use schemars::JsonSchema;
use serde::Deserialize;

/// Parameters for listing client scopes.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClientScopeListParams {
    /// The realm name
    pub realm: String,
}

/// Parameters for getting a single client scope.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClientScopeGetParams {
    /// The realm name
    pub realm: String,

    /// The client scope UUID
    pub id: String,
}

/// Parameters for creating a client scope.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClientScopeCreateParams {
    /// The realm name
    pub realm: String,

    /// The client scope representation to create
    pub client_scope: ClientScopeRepresentation,
}

/// Parameters for updating a client scope.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClientScopeUpdateParams {
    /// The realm name
    pub realm: String,

    /// The client scope UUID
    pub id: String,

    /// The updated client scope representation
    pub client_scope: ClientScopeRepresentation,
}

/// Parameters for deleting a client scope.
///
/// **Warning**: Built-in scopes (like `email`, `profile`, `openid`, `address`, `phone`, `offline_access`)
/// are protected by Keycloak and attempting to delete them will fail with a 403 or 409 error.
/// Only custom client scopes can be deleted.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClientScopeDeleteParams {
    /// The realm name
    pub realm: String,

    /// The client scope UUID
    pub id: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_scope_list_params_deserialization() {
        let json = r#"{"realm": "master"}"#;

        let params: ClientScopeListParams =
            serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(params.realm, "master");
    }

    #[test]
    fn test_client_scope_get_params_deserialization() {
        let json = r#"{
            "realm": "master",
            "id": "abc-123-def"
        }"#;

        let params: ClientScopeGetParams =
            serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(params.realm, "master");
        assert_eq!(params.id, "abc-123-def");
    }

    #[test]
    fn test_client_scope_create_params_deserialization() {
        let json = r#"{
            "realm": "master",
            "clientScope": {
                "name": "custom-scope",
                "protocol": "openid-connect"
            }
        }"#;

        let params: ClientScopeCreateParams =
            serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(params.realm, "master");
        assert_eq!(params.client_scope.name, Some("custom-scope".to_string()));
        assert_eq!(
            params.client_scope.protocol,
            Some("openid-connect".to_string())
        );
    }

    #[test]
    fn test_client_scope_update_params_deserialization() {
        let json = r#"{
            "realm": "master",
            "id": "abc-123",
            "clientScope": {
                "name": "updated-scope",
                "description": "Updated description"
            }
        }"#;

        let params: ClientScopeUpdateParams =
            serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(params.realm, "master");
        assert_eq!(params.id, "abc-123");
        assert_eq!(params.client_scope.name, Some("updated-scope".to_string()));
        assert_eq!(
            params.client_scope.description,
            Some("Updated description".to_string())
        );
    }

    #[test]
    fn test_client_scope_delete_params_deserialization() {
        let json = r#"{
            "realm": "master",
            "id": "abc-123-to-delete"
        }"#;

        let params: ClientScopeDeleteParams =
            serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(params.realm, "master");
        assert_eq!(params.id, "abc-123-to-delete");
    }
}
