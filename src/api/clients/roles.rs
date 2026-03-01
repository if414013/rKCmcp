//! Client role management API functions.
//!
//! This module provides functionality for managing client-specific roles in Keycloak,
//! including CRUD operations and querying users/groups assigned to a client role.

use crate::api::roles::RoleRepresentation;
use crate::api::{ApiError, KeycloakClient};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a user in Keycloak (simplified for role membership queries).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserRepresentation {
    /// The unique identifier for the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The username
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    /// The user's email address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// The user's first name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,

    /// The user's last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    /// Whether the user is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// Custom attributes associated with the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<HashMap<String, Vec<String>>>,
}

/// Represents a group in Keycloak (simplified for role membership queries).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GroupRepresentation {
    /// The unique identifier for the group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The name of the group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The path of the group in the group hierarchy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    /// Sub-groups of this group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_groups: Option<Vec<GroupRepresentation>>,

    /// Custom attributes associated with the group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<HashMap<String, Vec<String>>>,
}

/// Parameters for listing client roles.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClientRoleListParams {
    /// The realm name (not id!)
    pub realm: String,

    /// The client ID (UUID, not clientId!)
    pub client_id: String,

    /// Search string to filter roles by name
    #[serde(default)]
    pub search: Option<String>,

    /// Pagination offset (0-based)
    #[serde(default)]
    pub first: Option<i32>,

    /// Maximum number of results to return
    #[serde(default)]
    pub max: Option<i32>,

    /// If true, returns only basic role info (id, name, description)
    #[serde(default)]
    pub brief_representation: Option<bool>,
}

/// Parameters for getting a single client role by name.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClientRoleGetParams {
    /// The realm name (not id!)
    pub realm: String,

    /// The client ID (UUID, not clientId!)
    pub client_id: String,

    /// The role name (not id!)
    pub role_name: String,
}

/// Parameters for creating a new client role.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClientRoleCreateParams {
    /// The realm name (not id!)
    pub realm: String,

    /// The client ID (UUID, not clientId!)
    pub client_id: String,

    /// The name of the new role (required)
    pub name: String,

    /// A description of the role
    #[serde(default)]
    pub description: Option<String>,

    /// Custom attributes for the role
    #[serde(default)]
    pub attributes: Option<HashMap<String, Vec<String>>>,
}

/// Parameters for updating an existing client role.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClientRoleUpdateParams {
    /// The realm name (not id!)
    pub realm: String,

    /// The client ID (UUID, not clientId!)
    pub client_id: String,

    /// The current role name (not id!)
    pub role_name: String,

    /// The new name for the role (if renaming)
    #[serde(default)]
    pub new_name: Option<String>,

    /// The new description for the role
    #[serde(default)]
    pub description: Option<String>,

    /// Custom attributes for the role
    #[serde(default)]
    pub attributes: Option<HashMap<String, Vec<String>>>,
}

/// Parameters for deleting a client role.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClientRoleDeleteParams {
    /// The realm name (not id!)
    pub realm: String,

    /// The client ID (UUID, not clientId!)
    pub client_id: String,

    /// The role name (not id!)
    pub role_name: String,
}

/// Parameters for listing users with a specific client role.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClientRoleUsersListParams {
    /// The realm name (not id!)
    pub realm: String,

    /// The client ID (UUID, not clientId!)
    pub client_id: String,

    /// The role name (not id!)
    pub role_name: String,

    /// Pagination offset (0-based)
    #[serde(default)]
    pub first: Option<i32>,

    /// Maximum number of results to return
    #[serde(default)]
    pub max: Option<i32>,
}

/// Parameters for listing groups with a specific client role.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClientRoleGroupsListParams {
    /// The realm name (not id!)
    pub realm: String,

    /// The client ID (UUID, not clientId!)
    pub client_id: String,

    /// The role name (not id!)
    pub role_name: String,

    /// Pagination offset (0-based)
    #[serde(default)]
    pub first: Option<i32>,

    /// Maximum number of results to return
    #[serde(default)]
    pub max: Option<i32>,

    /// If true, returns only basic group info
    #[serde(default)]
    pub brief_representation: Option<bool>,
}

/// List roles for a specific client.
///
/// GET /admin/realms/{realm}/clients/{id}/roles
pub async fn client_roles_list(
    client: &KeycloakClient,
    token: &str,
    params: &ClientRoleListParams,
) -> Result<Vec<RoleRepresentation>, ApiError> {
    let mut path = format!(
        "/admin/realms/{}/clients/{}/roles",
        params.realm,
        urlencoding::encode(&params.client_id)
    );

    let mut query_parts = Vec::new();

    if let Some(ref search) = params.search {
        query_parts.push(format!("search={}", urlencoding::encode(search)));
    }
    if let Some(first) = params.first {
        query_parts.push(format!("first={}", first));
    }
    if let Some(max) = params.max {
        query_parts.push(format!("max={}", max));
    }
    if let Some(brief) = params.brief_representation {
        query_parts.push(format!("briefRepresentation={}", brief));
    }

    if !query_parts.is_empty() {
        path = format!("{}?{}", path, query_parts.join("&"));
    }

    client.get(&path, token).await
}

/// Get a single client role by name.
///
/// GET /admin/realms/{realm}/clients/{id}/roles/{role-name}
pub async fn client_role_get(
    client: &KeycloakClient,
    token: &str,
    params: &ClientRoleGetParams,
) -> Result<RoleRepresentation, ApiError> {
    let path = format!(
        "/admin/realms/{}/clients/{}/roles/{}",
        params.realm,
        urlencoding::encode(&params.client_id),
        urlencoding::encode(&params.role_name)
    );

    client.get(&path, token).await
}

/// Create a new role for a client.
///
/// POST /admin/realms/{realm}/clients/{id}/roles
pub async fn client_role_create(
    client: &KeycloakClient,
    token: &str,
    params: &ClientRoleCreateParams,
) -> Result<(), ApiError> {
    let path = format!(
        "/admin/realms/{}/clients/{}/roles",
        params.realm,
        urlencoding::encode(&params.client_id)
    );

    let role = RoleRepresentation {
        name: Some(params.name.clone()),
        description: params.description.clone(),
        attributes: params.attributes.clone(),
        client_role: Some(true),
        ..Default::default()
    };

    client.post_no_response(&path, token, &role).await
}

/// Update an existing client role.
///
/// PUT /admin/realms/{realm}/clients/{id}/roles/{role-name}
pub async fn client_role_update(
    client: &KeycloakClient,
    token: &str,
    params: &ClientRoleUpdateParams,
) -> Result<(), ApiError> {
    let path = format!(
        "/admin/realms/{}/clients/{}/roles/{}",
        params.realm,
        urlencoding::encode(&params.client_id),
        urlencoding::encode(&params.role_name)
    );

    let role = RoleRepresentation {
        name: params
            .new_name
            .clone()
            .or_else(|| Some(params.role_name.clone())),
        description: params.description.clone(),
        attributes: params.attributes.clone(),
        client_role: Some(true),
        ..Default::default()
    };

    client.put(&path, token, &role).await
}

/// Delete a client role.
///
/// DELETE /admin/realms/{realm}/clients/{id}/roles/{role-name}
pub async fn client_role_delete(
    client: &KeycloakClient,
    token: &str,
    params: &ClientRoleDeleteParams,
) -> Result<(), ApiError> {
    let path = format!(
        "/admin/realms/{}/clients/{}/roles/{}",
        params.realm,
        urlencoding::encode(&params.client_id),
        urlencoding::encode(&params.role_name)
    );

    client.delete(&path, token).await
}

/// List all users that have a specific client role assigned.
///
/// GET /admin/realms/{realm}/clients/{id}/roles/{role-name}/users
pub async fn client_role_users_list(
    client: &KeycloakClient,
    token: &str,
    params: &ClientRoleUsersListParams,
) -> Result<Vec<UserRepresentation>, ApiError> {
    let mut path = format!(
        "/admin/realms/{}/clients/{}/roles/{}/users",
        params.realm,
        urlencoding::encode(&params.client_id),
        urlencoding::encode(&params.role_name)
    );

    let mut query_parts = Vec::new();
    if let Some(first) = params.first {
        query_parts.push(format!("first={}", first));
    }
    if let Some(max) = params.max {
        query_parts.push(format!("max={}", max));
    }

    if !query_parts.is_empty() {
        path = format!("{}?{}", path, query_parts.join("&"));
    }

    client.get(&path, token).await
}

/// List all groups that have a specific client role assigned.
///
/// GET /admin/realms/{realm}/clients/{id}/roles/{role-name}/groups
pub async fn client_role_groups_list(
    client: &KeycloakClient,
    token: &str,
    params: &ClientRoleGroupsListParams,
) -> Result<Vec<GroupRepresentation>, ApiError> {
    let mut path = format!(
        "/admin/realms/{}/clients/{}/roles/{}/groups",
        params.realm,
        urlencoding::encode(&params.client_id),
        urlencoding::encode(&params.role_name)
    );

    let mut query_parts = Vec::new();
    if let Some(first) = params.first {
        query_parts.push(format!("first={}", first));
    }
    if let Some(max) = params.max {
        query_parts.push(format!("max={}", max));
    }
    if let Some(brief) = params.brief_representation {
        query_parts.push(format!("briefRepresentation={}", brief));
    }

    if !query_parts.is_empty() {
        path = format!("{}?{}", path, query_parts.join("&"));
    }

    client.get(&path, token).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{header, method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    const TEST_TOKEN: &str = "test-bearer-token";
    const TEST_CLIENT_ID: &str = "abc-123-client-uuid";

    fn sample_role() -> RoleRepresentation {
        RoleRepresentation {
            id: Some("role-uuid-123".to_string()),
            name: Some("test-role".to_string()),
            description: Some("A test client role".to_string()),
            composite: Some(false),
            client_role: Some(true),
            container_id: Some(TEST_CLIENT_ID.to_string()),
            attributes: None,
        }
    }

    #[tokio::test]
    async fn test_client_roles_list_success() {
        let mock_server = MockServer::start().await;

        let expected_roles = vec![sample_role()];

        Mock::given(method("GET"))
            .and(path(format!(
                "/admin/realms/master/clients/{}/roles",
                TEST_CLIENT_ID
            )))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_roles))
            .mount(&mock_server)
            .await;

        let client =
            KeycloakClient::new(mock_server.uri()).expect("client creation should succeed");
        let params = ClientRoleListParams {
            realm: "master".to_string(),
            client_id: TEST_CLIENT_ID.to_string(),
            search: None,
            first: None,
            max: None,
            brief_representation: None,
        };

        let result = client_roles_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());

        let roles = result.expect("should return roles");
        assert_eq!(roles.len(), 1);
        assert_eq!(roles[0].name, Some("test-role".to_string()));
        assert_eq!(roles[0].client_role, Some(true));
    }

    #[tokio::test]
    async fn test_client_roles_list_with_filters() {
        let mock_server = MockServer::start().await;

        let expected_roles = vec![sample_role()];

        Mock::given(method("GET"))
            .and(path(format!(
                "/admin/realms/test-realm/clients/{}/roles",
                TEST_CLIENT_ID
            )))
            .and(query_param("search", "admin"))
            .and(query_param("first", "0"))
            .and(query_param("max", "10"))
            .and(query_param("briefRepresentation", "true"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_roles))
            .mount(&mock_server)
            .await;

        let client =
            KeycloakClient::new(mock_server.uri()).expect("client creation should succeed");
        let params = ClientRoleListParams {
            realm: "test-realm".to_string(),
            client_id: TEST_CLIENT_ID.to_string(),
            search: Some("admin".to_string()),
            first: Some(0),
            max: Some(10),
            brief_representation: Some(true),
        };

        let result = client_roles_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_client_roles_list_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(format!(
                "/admin/realms/master/clients/{}/roles",
                TEST_CLIENT_ID
            )))
            .respond_with(ResponseTemplate::new(401))
            .mount(&mock_server)
            .await;

        let client =
            KeycloakClient::new(mock_server.uri()).expect("client creation should succeed");
        let params = ClientRoleListParams {
            realm: "master".to_string(),
            client_id: TEST_CLIENT_ID.to_string(),
            search: None,
            first: None,
            max: None,
            brief_representation: None,
        };

        let result = client_roles_list(&client, "invalid-token", &params).await;
        assert!(matches!(result, Err(ApiError::Unauthorized)));
    }

    #[tokio::test]
    async fn test_client_role_get_success() {
        let mock_server = MockServer::start().await;

        let expected_role = sample_role();

        Mock::given(method("GET"))
            .and(path(format!(
                "/admin/realms/master/clients/{}/roles/test-role",
                TEST_CLIENT_ID
            )))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_role))
            .mount(&mock_server)
            .await;

        let client =
            KeycloakClient::new(mock_server.uri()).expect("client creation should succeed");
        let params = ClientRoleGetParams {
            realm: "master".to_string(),
            client_id: TEST_CLIENT_ID.to_string(),
            role_name: "test-role".to_string(),
        };

        let result = client_role_get(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());

        let role = result.expect("should return role");
        assert_eq!(role.name, Some("test-role".to_string()));
        assert_eq!(role.id, Some("role-uuid-123".to_string()));
    }

    #[tokio::test]
    async fn test_client_role_get_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(format!(
                "/admin/realms/master/clients/{}/roles/nonexistent",
                TEST_CLIENT_ID
            )))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client =
            KeycloakClient::new(mock_server.uri()).expect("client creation should succeed");
        let params = ClientRoleGetParams {
            realm: "master".to_string(),
            client_id: TEST_CLIENT_ID.to_string(),
            role_name: "nonexistent".to_string(),
        };

        let result = client_role_get(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_client_role_get_with_special_characters() {
        let mock_server = MockServer::start().await;

        let role = RoleRepresentation {
            id: Some("456".to_string()),
            name: Some("my role".to_string()),
            client_role: Some(true),
            ..Default::default()
        };

        Mock::given(method("GET"))
            .and(path(format!(
                "/admin/realms/master/clients/{}/roles/my%20role",
                TEST_CLIENT_ID
            )))
            .respond_with(ResponseTemplate::new(200).set_body_json(&role))
            .mount(&mock_server)
            .await;

        let client =
            KeycloakClient::new(mock_server.uri()).expect("client creation should succeed");
        let params = ClientRoleGetParams {
            realm: "master".to_string(),
            client_id: TEST_CLIENT_ID.to_string(),
            role_name: "my role".to_string(),
        };

        let result = client_role_get(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_client_role_create_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path(format!(
                "/admin/realms/master/clients/{}/roles",
                TEST_CLIENT_ID
            )))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .and(header("content-type", "application/json"))
            .respond_with(ResponseTemplate::new(201))
            .mount(&mock_server)
            .await;

        let client =
            KeycloakClient::new(mock_server.uri()).expect("client creation should succeed");
        let params = ClientRoleCreateParams {
            realm: "master".to_string(),
            client_id: TEST_CLIENT_ID.to_string(),
            name: "new-role".to_string(),
            description: Some("A new client role".to_string()),
            attributes: None,
        };

        let result = client_role_create(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_client_role_create_conflict() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path(format!(
                "/admin/realms/master/clients/{}/roles",
                TEST_CLIENT_ID
            )))
            .respond_with(ResponseTemplate::new(409).set_body_json(serde_json::json!({
                "errorMessage": "Role with name admin already exists"
            })))
            .mount(&mock_server)
            .await;

        let client =
            KeycloakClient::new(mock_server.uri()).expect("client creation should succeed");
        let params = ClientRoleCreateParams {
            realm: "master".to_string(),
            client_id: TEST_CLIENT_ID.to_string(),
            name: "admin".to_string(),
            description: None,
            attributes: None,
        };

        let result = client_role_create(&client, TEST_TOKEN, &params).await;
        match result {
            Err(ApiError::Conflict(msg)) => {
                assert!(msg.contains("already exists"));
            }
            _ => panic!("Expected Conflict error"),
        }
    }

    #[tokio::test]
    async fn test_client_role_update_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path(format!(
                "/admin/realms/master/clients/{}/roles/old-name",
                TEST_CLIENT_ID
            )))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let client =
            KeycloakClient::new(mock_server.uri()).expect("client creation should succeed");
        let params = ClientRoleUpdateParams {
            realm: "master".to_string(),
            client_id: TEST_CLIENT_ID.to_string(),
            role_name: "old-name".to_string(),
            new_name: Some("new-name".to_string()),
            description: Some("Updated description".to_string()),
            attributes: None,
        };

        let result = client_role_update(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_client_role_update_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path(format!(
                "/admin/realms/master/clients/{}/roles/nonexistent",
                TEST_CLIENT_ID
            )))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client =
            KeycloakClient::new(mock_server.uri()).expect("client creation should succeed");
        let params = ClientRoleUpdateParams {
            realm: "master".to_string(),
            client_id: TEST_CLIENT_ID.to_string(),
            role_name: "nonexistent".to_string(),
            new_name: None,
            description: Some("New description".to_string()),
            attributes: None,
        };

        let result = client_role_update(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_client_role_delete_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path(format!(
                "/admin/realms/master/clients/{}/roles/test-role",
                TEST_CLIENT_ID
            )))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let client =
            KeycloakClient::new(mock_server.uri()).expect("client creation should succeed");
        let params = ClientRoleDeleteParams {
            realm: "master".to_string(),
            client_id: TEST_CLIENT_ID.to_string(),
            role_name: "test-role".to_string(),
        };

        let result = client_role_delete(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_client_role_delete_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path(format!(
                "/admin/realms/master/clients/{}/roles/nonexistent",
                TEST_CLIENT_ID
            )))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client =
            KeycloakClient::new(mock_server.uri()).expect("client creation should succeed");
        let params = ClientRoleDeleteParams {
            realm: "master".to_string(),
            client_id: TEST_CLIENT_ID.to_string(),
            role_name: "nonexistent".to_string(),
        };

        let result = client_role_delete(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_client_role_users_list_success() {
        let mock_server = MockServer::start().await;

        let users = vec![
            UserRepresentation {
                id: Some("user1".to_string()),
                username: Some("john.doe".to_string()),
                email: Some("john@example.com".to_string()),
                first_name: Some("John".to_string()),
                last_name: Some("Doe".to_string()),
                enabled: Some(true),
                ..Default::default()
            },
            UserRepresentation {
                id: Some("user2".to_string()),
                username: Some("jane.doe".to_string()),
                email: Some("jane@example.com".to_string()),
                first_name: Some("Jane".to_string()),
                last_name: Some("Doe".to_string()),
                enabled: Some(true),
                ..Default::default()
            },
        ];

        Mock::given(method("GET"))
            .and(path(format!(
                "/admin/realms/master/clients/{}/roles/admin/users",
                TEST_CLIENT_ID
            )))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&users))
            .mount(&mock_server)
            .await;

        let client =
            KeycloakClient::new(mock_server.uri()).expect("client creation should succeed");
        let params = ClientRoleUsersListParams {
            realm: "master".to_string(),
            client_id: TEST_CLIENT_ID.to_string(),
            role_name: "admin".to_string(),
            first: None,
            max: None,
        };

        let result = client_role_users_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
        let returned_users = result.expect("should return users");
        assert_eq!(returned_users.len(), 2);
        assert_eq!(returned_users[0].username, Some("john.doe".to_string()));
    }

    #[tokio::test]
    async fn test_client_role_users_list_with_pagination() {
        let mock_server = MockServer::start().await;

        let users: Vec<UserRepresentation> = vec![];

        Mock::given(method("GET"))
            .and(path(format!(
                "/admin/realms/master/clients/{}/roles/admin/users",
                TEST_CLIENT_ID
            )))
            .and(query_param("first", "10"))
            .and(query_param("max", "20"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&users))
            .mount(&mock_server)
            .await;

        let client =
            KeycloakClient::new(mock_server.uri()).expect("client creation should succeed");
        let params = ClientRoleUsersListParams {
            realm: "master".to_string(),
            client_id: TEST_CLIENT_ID.to_string(),
            role_name: "admin".to_string(),
            first: Some(10),
            max: Some(20),
        };

        let result = client_role_users_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_client_role_users_list_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(format!(
                "/admin/realms/master/clients/{}/roles/nonexistent/users",
                TEST_CLIENT_ID
            )))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client =
            KeycloakClient::new(mock_server.uri()).expect("client creation should succeed");
        let params = ClientRoleUsersListParams {
            realm: "master".to_string(),
            client_id: TEST_CLIENT_ID.to_string(),
            role_name: "nonexistent".to_string(),
            first: None,
            max: None,
        };

        let result = client_role_users_list(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_client_role_groups_list_success() {
        let mock_server = MockServer::start().await;

        let groups = vec![
            GroupRepresentation {
                id: Some("group1".to_string()),
                name: Some("Engineering".to_string()),
                path: Some("/Engineering".to_string()),
                sub_groups: None,
                attributes: None,
            },
            GroupRepresentation {
                id: Some("group2".to_string()),
                name: Some("Marketing".to_string()),
                path: Some("/Marketing".to_string()),
                sub_groups: None,
                attributes: None,
            },
        ];

        Mock::given(method("GET"))
            .and(path(format!(
                "/admin/realms/master/clients/{}/roles/admin/groups",
                TEST_CLIENT_ID
            )))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&groups))
            .mount(&mock_server)
            .await;

        let client =
            KeycloakClient::new(mock_server.uri()).expect("client creation should succeed");
        let params = ClientRoleGroupsListParams {
            realm: "master".to_string(),
            client_id: TEST_CLIENT_ID.to_string(),
            role_name: "admin".to_string(),
            first: None,
            max: None,
            brief_representation: None,
        };

        let result = client_role_groups_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
        let returned_groups = result.expect("should return groups");
        assert_eq!(returned_groups.len(), 2);
        assert_eq!(returned_groups[0].name, Some("Engineering".to_string()));
    }

    #[tokio::test]
    async fn test_client_role_groups_list_with_pagination_and_brief() {
        let mock_server = MockServer::start().await;

        let groups: Vec<GroupRepresentation> = vec![];

        Mock::given(method("GET"))
            .and(path(format!(
                "/admin/realms/master/clients/{}/roles/admin/groups",
                TEST_CLIENT_ID
            )))
            .and(query_param("first", "5"))
            .and(query_param("max", "10"))
            .and(query_param("briefRepresentation", "true"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&groups))
            .mount(&mock_server)
            .await;

        let client =
            KeycloakClient::new(mock_server.uri()).expect("client creation should succeed");
        let params = ClientRoleGroupsListParams {
            realm: "master".to_string(),
            client_id: TEST_CLIENT_ID.to_string(),
            role_name: "admin".to_string(),
            first: Some(5),
            max: Some(10),
            brief_representation: Some(true),
        };

        let result = client_role_groups_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_client_role_groups_list_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(format!(
                "/admin/realms/master/clients/{}/roles/nonexistent/groups",
                TEST_CLIENT_ID
            )))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client =
            KeycloakClient::new(mock_server.uri()).expect("client creation should succeed");
        let params = ClientRoleGroupsListParams {
            realm: "master".to_string(),
            client_id: TEST_CLIENT_ID.to_string(),
            role_name: "nonexistent".to_string(),
            first: None,
            max: None,
            brief_representation: None,
        };

        let result = client_role_groups_list(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_client_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/master/clients/nonexistent-client/roles",
            ))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client =
            KeycloakClient::new(mock_server.uri()).expect("client creation should succeed");
        let params = ClientRoleListParams {
            realm: "master".to_string(),
            client_id: "nonexistent-client".to_string(),
            search: None,
            first: None,
            max: None,
            brief_representation: None,
        };

        let result = client_roles_list(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_user_representation_serialization() {
        let user = UserRepresentation {
            id: Some("123".to_string()),
            username: Some("testuser".to_string()),
            email: Some("test@example.com".to_string()),
            first_name: Some("Test".to_string()),
            last_name: Some("User".to_string()),
            enabled: Some(true),
            attributes: None,
        };

        let json = serde_json::to_string(&user).expect("serialization should succeed");
        assert!(json.contains("\"id\":\"123\""));
        assert!(json.contains("\"username\":\"testuser\""));
        assert!(json.contains("\"email\":\"test@example.com\""));
    }

    #[tokio::test]
    async fn test_group_representation_serialization() {
        let group = GroupRepresentation {
            id: Some("456".to_string()),
            name: Some("TestGroup".to_string()),
            path: Some("/TestGroup".to_string()),
            sub_groups: None,
            attributes: None,
        };

        let json = serde_json::to_string(&group).expect("serialization should succeed");
        assert!(json.contains("\"id\":\"456\""));
        assert!(json.contains("\"name\":\"TestGroup\""));
        assert!(json.contains("\"path\":\"/TestGroup\""));
    }

    #[tokio::test]
    async fn test_forbidden_error() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(format!(
                "/admin/realms/master/clients/{}/roles",
                TEST_CLIENT_ID
            )))
            .respond_with(ResponseTemplate::new(403))
            .mount(&mock_server)
            .await;

        let client =
            KeycloakClient::new(mock_server.uri()).expect("client creation should succeed");
        let params = ClientRoleListParams {
            realm: "master".to_string(),
            client_id: TEST_CLIENT_ID.to_string(),
            search: None,
            first: None,
            max: None,
            brief_representation: None,
        };

        let result = client_roles_list(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::Forbidden)));
    }
}
