//! Keycloak Realms Defaults Admin API module.
//!
//! Provides operations for managing default groups and client scopes.

use crate::api::groups::GroupRepresentation;
use crate::api::{ApiError, KeycloakClient};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Client scope representation.
///
/// Represents an OpenID Connect or SAML scope that can be attached to clients.
#[derive(Debug, Clone, Serialize, Deserialize, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClientScopeRepresentation {
    /// Unique identifier for the client scope (UUID)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Name of the client scope
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Description of the client scope
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Protocol (openid-connect, saml)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,

    /// Custom attributes for the scope
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<HashMap<String, String>>,

    /// Protocol mappers for this scope
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_mappers: Option<Vec<ProtocolMapperRepresentation>>,
}

/// Protocol mapper representation.
///
/// Configures how user attributes are mapped to token claims.
#[derive(Debug, Clone, Serialize, Deserialize, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ProtocolMapperRepresentation {
    /// Unique identifier for the mapper (UUID)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Name of the mapper
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Protocol (openid-connect, saml)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,

    /// Mapper type identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_mapper: Option<String>,

    /// Whether the mapper is enabled for consent screen
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent_required: Option<bool>,

    /// Configuration key-value pairs for the mapper
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<HashMap<String, String>>,
}

/// Parameters for listing default groups.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RealmDefaultGroupsListParams {
    /// The realm name
    pub realm: String,
}

/// Parameters for adding a default group.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RealmDefaultGroupAddParams {
    /// The realm name
    pub realm: String,

    /// The group ID to add as default
    pub group_id: String,
}

/// Parameters for removing a default group.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RealmDefaultGroupRemoveParams {
    /// The realm name
    pub realm: String,

    /// The group ID to remove from defaults
    pub group_id: String,
}

/// Parameters for listing default client scopes.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RealmDefaultClientScopesListParams {
    /// The realm name
    pub realm: String,
}

/// Parameters for listing default optional client scopes.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RealmDefaultOptionalScopesListParams {
    /// The realm name
    pub realm: String,
}

/// Get default groups for a realm.
///
/// GET /admin/realms/{realm}/default-groups
pub async fn realm_default_groups_list(
    client: &KeycloakClient,
    token: &str,
    params: &RealmDefaultGroupsListParams,
) -> Result<Vec<GroupRepresentation>, ApiError> {
    let path = format!(
        "/admin/realms/{}/default-groups",
        urlencoding::encode(&params.realm)
    );

    client.get(&path, token).await
}

/// Add a group to the realm's default groups.
///
/// PUT /admin/realms/{realm}/default-groups/{groupId}
pub async fn realm_default_group_add(
    client: &KeycloakClient,
    token: &str,
    params: &RealmDefaultGroupAddParams,
) -> Result<(), ApiError> {
    let path = format!(
        "/admin/realms/{}/default-groups/{}",
        urlencoding::encode(&params.realm),
        urlencoding::encode(&params.group_id)
    );

    client.put(&path, token, &serde_json::Value::Null).await
}

/// Remove a group from the realm's default groups.
///
/// DELETE /admin/realms/{realm}/default-groups/{groupId}
pub async fn realm_default_group_remove(
    client: &KeycloakClient,
    token: &str,
    params: &RealmDefaultGroupRemoveParams,
) -> Result<(), ApiError> {
    let path = format!(
        "/admin/realms/{}/default-groups/{}",
        urlencoding::encode(&params.realm),
        urlencoding::encode(&params.group_id)
    );

    client.delete(&path, token).await
}

/// Get default client scopes for a realm.
///
/// GET /admin/realms/{realm}/default-default-client-scopes
pub async fn realm_default_client_scopes_list(
    client: &KeycloakClient,
    token: &str,
    params: &RealmDefaultClientScopesListParams,
) -> Result<Vec<ClientScopeRepresentation>, ApiError> {
    let path = format!(
        "/admin/realms/{}/default-default-client-scopes",
        urlencoding::encode(&params.realm)
    );

    client.get(&path, token).await
}

/// Get default optional client scopes for a realm.
///
/// GET /admin/realms/{realm}/default-optional-client-scopes
pub async fn realm_default_optional_scopes_list(
    client: &KeycloakClient,
    token: &str,
    params: &RealmDefaultOptionalScopesListParams,
) -> Result<Vec<ClientScopeRepresentation>, ApiError> {
    let path = format!(
        "/admin/realms/{}/default-optional-client-scopes",
        urlencoding::encode(&params.realm)
    );

    client.get(&path, token).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    const TEST_TOKEN: &str = "test-bearer-token";

    fn sample_group() -> GroupRepresentation {
        GroupRepresentation {
            id: Some("group-123".to_string()),
            name: Some("default-users".to_string()),
            path: Some("/default-users".to_string()),
            ..Default::default()
        }
    }

    fn sample_client_scope() -> ClientScopeRepresentation {
        ClientScopeRepresentation {
            id: Some("scope-123".to_string()),
            name: Some("email".to_string()),
            description: Some("OpenID Connect email scope".to_string()),
            protocol: Some("openid-connect".to_string()),
            ..Default::default()
        }
    }

    // ==================== realm_default_groups_list tests ====================

    #[tokio::test]
    async fn test_realm_default_groups_list_success() {
        let mock_server = MockServer::start().await;

        let expected_groups = vec![sample_group()];

        Mock::given(method("GET"))
            .and(path("/admin/realms/test-realm/default-groups"))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_groups))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmDefaultGroupsListParams {
            realm: "test-realm".to_string(),
        };

        let result = realm_default_groups_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());

        let groups = result.expect("Failed to get default groups");
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].name, Some("default-users".to_string()));
    }

    #[tokio::test]
    async fn test_realm_default_groups_list_empty() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/admin/realms/test-realm/default-groups"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(Vec::<GroupRepresentation>::new()),
            )
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmDefaultGroupsListParams {
            realm: "test-realm".to_string(),
        };

        let result = realm_default_groups_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_realm_default_groups_list_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/admin/realms/nonexistent/default-groups"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmDefaultGroupsListParams {
            realm: "nonexistent".to_string(),
        };

        let result = realm_default_groups_list(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_realm_default_groups_list_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/admin/realms/test-realm/default-groups"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmDefaultGroupsListParams {
            realm: "test-realm".to_string(),
        };

        let result = realm_default_groups_list(&client, "invalid-token", &params).await;
        assert!(matches!(result, Err(ApiError::Unauthorized)));
    }

    // ==================== realm_default_group_add tests ====================

    #[tokio::test]
    async fn test_realm_default_group_add_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path("/admin/realms/test-realm/default-groups/group-123"))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmDefaultGroupAddParams {
            realm: "test-realm".to_string(),
            group_id: "group-123".to_string(),
        };

        let result = realm_default_group_add(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_realm_default_group_add_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path("/admin/realms/test-realm/default-groups/nonexistent"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmDefaultGroupAddParams {
            realm: "test-realm".to_string(),
            group_id: "nonexistent".to_string(),
        };

        let result = realm_default_group_add(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_realm_default_group_add_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path("/admin/realms/test-realm/default-groups/group-123"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmDefaultGroupAddParams {
            realm: "test-realm".to_string(),
            group_id: "group-123".to_string(),
        };

        let result = realm_default_group_add(&client, "invalid-token", &params).await;
        assert!(matches!(result, Err(ApiError::Unauthorized)));
    }

    #[tokio::test]
    async fn test_realm_default_group_add_with_special_characters() {
        let mock_server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path("/admin/realms/my%20realm/default-groups/group%2F123"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmDefaultGroupAddParams {
            realm: "my realm".to_string(),
            group_id: "group/123".to_string(),
        };

        let result = realm_default_group_add(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    // ==================== realm_default_group_remove tests ====================

    #[tokio::test]
    async fn test_realm_default_group_remove_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/admin/realms/test-realm/default-groups/group-123"))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmDefaultGroupRemoveParams {
            realm: "test-realm".to_string(),
            group_id: "group-123".to_string(),
        };

        let result = realm_default_group_remove(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_realm_default_group_remove_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/admin/realms/test-realm/default-groups/nonexistent"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmDefaultGroupRemoveParams {
            realm: "test-realm".to_string(),
            group_id: "nonexistent".to_string(),
        };

        let result = realm_default_group_remove(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_realm_default_group_remove_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/admin/realms/test-realm/default-groups/group-123"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmDefaultGroupRemoveParams {
            realm: "test-realm".to_string(),
            group_id: "group-123".to_string(),
        };

        let result = realm_default_group_remove(&client, "invalid-token", &params).await;
        assert!(matches!(result, Err(ApiError::Unauthorized)));
    }

    // ==================== realm_default_client_scopes_list tests ====================

    #[tokio::test]
    async fn test_realm_default_client_scopes_list_success() {
        let mock_server = MockServer::start().await;

        let expected_scopes = vec![sample_client_scope()];

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/test-realm/default-default-client-scopes",
            ))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_scopes))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmDefaultClientScopesListParams {
            realm: "test-realm".to_string(),
        };

        let result = realm_default_client_scopes_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());

        let scopes = result.expect("Failed to get default client scopes");
        assert_eq!(scopes.len(), 1);
        assert_eq!(scopes[0].name, Some("email".to_string()));
    }

    #[tokio::test]
    async fn test_realm_default_client_scopes_list_empty() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/test-realm/default-default-client-scopes",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(Vec::<ClientScopeRepresentation>::new()),
            )
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmDefaultClientScopesListParams {
            realm: "test-realm".to_string(),
        };

        let result = realm_default_client_scopes_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_realm_default_client_scopes_list_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/nonexistent/default-default-client-scopes",
            ))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmDefaultClientScopesListParams {
            realm: "nonexistent".to_string(),
        };

        let result = realm_default_client_scopes_list(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_realm_default_client_scopes_list_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/test-realm/default-default-client-scopes",
            ))
            .respond_with(ResponseTemplate::new(401))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmDefaultClientScopesListParams {
            realm: "test-realm".to_string(),
        };

        let result = realm_default_client_scopes_list(&client, "invalid-token", &params).await;
        assert!(matches!(result, Err(ApiError::Unauthorized)));
    }

    // ==================== realm_default_optional_scopes_list tests ====================

    #[tokio::test]
    async fn test_realm_default_optional_scopes_list_success() {
        let mock_server = MockServer::start().await;

        let expected_scopes = vec![sample_client_scope()];

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/test-realm/default-optional-client-scopes",
            ))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_scopes))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmDefaultOptionalScopesListParams {
            realm: "test-realm".to_string(),
        };

        let result = realm_default_optional_scopes_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());

        let scopes = result.expect("Failed to get default optional scopes");
        assert_eq!(scopes.len(), 1);
        assert_eq!(scopes[0].name, Some("email".to_string()));
    }

    #[tokio::test]
    async fn test_realm_default_optional_scopes_list_empty() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/test-realm/default-optional-client-scopes",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(Vec::<ClientScopeRepresentation>::new()),
            )
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmDefaultOptionalScopesListParams {
            realm: "test-realm".to_string(),
        };

        let result = realm_default_optional_scopes_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_realm_default_optional_scopes_list_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/nonexistent/default-optional-client-scopes",
            ))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmDefaultOptionalScopesListParams {
            realm: "nonexistent".to_string(),
        };

        let result = realm_default_optional_scopes_list(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_realm_default_optional_scopes_list_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/test-realm/default-optional-client-scopes",
            ))
            .respond_with(ResponseTemplate::new(401))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmDefaultOptionalScopesListParams {
            realm: "test-realm".to_string(),
        };

        let result = realm_default_optional_scopes_list(&client, "invalid-token", &params).await;
        assert!(matches!(result, Err(ApiError::Unauthorized)));
    }
}
