//! Role composite and membership API functions.
//!
//! This module provides functionality for managing composite roles (roles that contain other roles)
//! and querying role membership (users and groups assigned to a role).

use crate::api::{ApiError, KeycloakClient};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::RoleRepresentation;

/// Represents a user in Keycloak.
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

/// Represents a group in Keycloak.
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

/// Parameters for listing composite roles of a role.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RoleCompositesListParams {
    /// The realm name (not id!)
    pub realm: String,

    /// The role name (not id!)
    pub role_name: String,
}

/// Parameters for adding composite roles to a role.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RoleCompositesAddParams {
    /// The realm name (not id!)
    pub realm: String,

    /// The parent role name (not id!)
    pub role_name: String,

    /// The roles to add as composites (array of role representations)
    pub roles: Vec<RoleRepresentation>,
}

/// Parameters for removing composite roles from a role.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RoleCompositesRemoveParams {
    /// The realm name (not id!)
    pub realm: String,

    /// The parent role name (not id!)
    pub role_name: String,

    /// The roles to remove from composites (array of role representations)
    pub roles: Vec<RoleRepresentation>,
}

/// Parameters for listing users with a specific role.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RoleUsersListParams {
    /// The realm name (not id!)
    pub realm: String,

    /// The role name (not id!)
    pub role_name: String,

    /// Pagination offset (0-based)
    #[serde(default)]
    pub first: Option<i32>,

    /// Maximum number of results to return
    #[serde(default)]
    pub max: Option<i32>,
}

/// Parameters for listing groups with a specific role.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RoleGroupsListParams {
    /// The realm name (not id!)
    pub realm: String,

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

/// List the composite roles of a specific role.
///
/// Returns all roles that are included in the specified role as composites.
pub async fn realm_role_composites_list(
    client: &KeycloakClient,
    token: &str,
    params: &RoleCompositesListParams,
) -> Result<Vec<RoleRepresentation>, ApiError> {
    let path = format!(
        "/admin/realms/{}/roles/{}/composites",
        params.realm,
        urlencoding::encode(&params.role_name)
    );
    client.get(&path, token).await
}

/// Add composite roles to a role.
///
/// Makes the specified role a composite role by adding other roles as its components.
/// The parent role will inherit all permissions from the composite roles.
pub async fn realm_role_composites_add(
    client: &KeycloakClient,
    token: &str,
    params: &RoleCompositesAddParams,
) -> Result<(), ApiError> {
    let path = format!(
        "/admin/realms/{}/roles/{}/composites",
        params.realm,
        urlencoding::encode(&params.role_name)
    );
    client.post_no_response(&path, token, &params.roles).await
}

/// Remove composite roles from a role.
///
/// Removes the specified roles from the parent role's composites.
pub async fn realm_role_composites_remove(
    client: &KeycloakClient,
    token: &str,
    params: &RoleCompositesRemoveParams,
) -> Result<(), ApiError> {
    let path = format!(
        "/admin/realms/{}/roles/{}/composites",
        params.realm,
        urlencoding::encode(&params.role_name)
    );

    client.delete_with_body(&path, token, &params.roles).await
}

/// List all users that have the specified role assigned.
///
/// Returns users who have this role either directly assigned or through group membership.
pub async fn realm_role_users_list(
    client: &KeycloakClient,
    token: &str,
    params: &RoleUsersListParams,
) -> Result<Vec<UserRepresentation>, ApiError> {
    let mut path = format!(
        "/admin/realms/{}/roles/{}/users",
        params.realm,
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

/// List all groups that have the specified role assigned.
///
/// Returns groups that have this role assigned at the group level.
pub async fn realm_role_groups_list(
    client: &KeycloakClient,
    token: &str,
    params: &RoleGroupsListParams,
) -> Result<Vec<GroupRepresentation>, ApiError> {
    let mut path = format!(
        "/admin/realms/{}/roles/{}/groups",
        params.realm,
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

    #[tokio::test]
    async fn test_realm_role_composites_list_success() {
        let mock_server = MockServer::start().await;

        let composite_roles = vec![
            RoleRepresentation {
                id: Some("1".to_string()),
                name: Some("child-role-1".to_string()),
                description: Some("First child role".to_string()),
                ..Default::default()
            },
            RoleRepresentation {
                id: Some("2".to_string()),
                name: Some("child-role-2".to_string()),
                description: Some("Second child role".to_string()),
                ..Default::default()
            },
        ];

        Mock::given(method("GET"))
            .and(path("/admin/realms/master/roles/parent-role/composites"))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&composite_roles))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("client creation should succeed");
        let params = RoleCompositesListParams {
            realm: "master".to_string(),
            role_name: "parent-role".to_string(),
        };

        let result = realm_role_composites_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
        let roles = result.expect("should return roles");
        assert_eq!(roles.len(), 2);
        assert_eq!(roles[0].name, Some("child-role-1".to_string()));
    }

    #[tokio::test]
    async fn test_realm_role_composites_list_empty() {
        let mock_server = MockServer::start().await;

        let composite_roles: Vec<RoleRepresentation> = vec![];

        Mock::given(method("GET"))
            .and(path("/admin/realms/master/roles/simple-role/composites"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&composite_roles))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("client creation should succeed");
        let params = RoleCompositesListParams {
            realm: "master".to_string(),
            role_name: "simple-role".to_string(),
        };

        let result = realm_role_composites_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
        let roles = result.expect("should return empty list");
        assert!(roles.is_empty());
    }

    #[tokio::test]
    async fn test_realm_role_composites_list_with_special_characters() {
        let mock_server = MockServer::start().await;

        let composite_roles: Vec<RoleRepresentation> = vec![];

        Mock::given(method("GET"))
            .and(path("/admin/realms/master/roles/my%20role/composites"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&composite_roles))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("client creation should succeed");
        let params = RoleCompositesListParams {
            realm: "master".to_string(),
            role_name: "my role".to_string(),
        };

        let result = realm_role_composites_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_realm_role_composites_add_success() {
        let mock_server = MockServer::start().await;

        let roles_to_add = vec![
            RoleRepresentation {
                id: Some("1".to_string()),
                name: Some("child-role".to_string()),
                ..Default::default()
            },
        ];

        Mock::given(method("POST"))
            .and(path("/admin/realms/master/roles/parent-role/composites"))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .and(header("content-type", "application/json"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("client creation should succeed");
        let params = RoleCompositesAddParams {
            realm: "master".to_string(),
            role_name: "parent-role".to_string(),
            roles: roles_to_add,
        };

        let result = realm_role_composites_add(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_realm_role_composites_add_not_found() {
        let mock_server = MockServer::start().await;

        let roles_to_add = vec![
            RoleRepresentation {
                id: Some("999".to_string()),
                name: Some("nonexistent".to_string()),
                ..Default::default()
            },
        ];

        Mock::given(method("POST"))
            .and(path("/admin/realms/master/roles/parent-role/composites"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("client creation should succeed");
        let params = RoleCompositesAddParams {
            realm: "master".to_string(),
            role_name: "parent-role".to_string(),
            roles: roles_to_add,
        };

        let result = realm_role_composites_add(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_realm_role_composites_remove_success() {
        let mock_server = MockServer::start().await;

        let roles_to_remove = vec![
            RoleRepresentation {
                id: Some("1".to_string()),
                name: Some("child-role".to_string()),
                ..Default::default()
            },
        ];

        Mock::given(method("DELETE"))
            .and(path("/admin/realms/master/roles/parent-role/composites"))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .and(header("content-type", "application/json"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("client creation should succeed");
        let params = RoleCompositesRemoveParams {
            realm: "master".to_string(),
            role_name: "parent-role".to_string(),
            roles: roles_to_remove,
        };

        let result = realm_role_composites_remove(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_realm_role_composites_remove_not_found() {
        let mock_server = MockServer::start().await;

        let roles_to_remove = vec![
            RoleRepresentation {
                id: Some("999".to_string()),
                name: Some("nonexistent".to_string()),
                ..Default::default()
            },
        ];

        Mock::given(method("DELETE"))
            .and(path("/admin/realms/master/roles/parent-role/composites"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("client creation should succeed");
        let params = RoleCompositesRemoveParams {
            realm: "master".to_string(),
            role_name: "parent-role".to_string(),
            roles: roles_to_remove,
        };

        let result = realm_role_composites_remove(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_realm_role_users_list_success() {
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
            .and(path("/admin/realms/master/roles/admin/users"))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&users))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("client creation should succeed");
        let params = RoleUsersListParams {
            realm: "master".to_string(),
            role_name: "admin".to_string(),
            first: None,
            max: None,
        };

        let result = realm_role_users_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
        let returned_users = result.expect("should return users");
        assert_eq!(returned_users.len(), 2);
        assert_eq!(returned_users[0].username, Some("john.doe".to_string()));
    }

    #[tokio::test]
    async fn test_realm_role_users_list_with_pagination() {
        let mock_server = MockServer::start().await;

        let users: Vec<UserRepresentation> = vec![];

        Mock::given(method("GET"))
            .and(path("/admin/realms/master/roles/admin/users"))
            .and(query_param("first", "10"))
            .and(query_param("max", "20"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&users))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("client creation should succeed");
        let params = RoleUsersListParams {
            realm: "master".to_string(),
            role_name: "admin".to_string(),
            first: Some(10),
            max: Some(20),
        };

        let result = realm_role_users_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_realm_role_users_list_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/admin/realms/master/roles/nonexistent/users"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("client creation should succeed");
        let params = RoleUsersListParams {
            realm: "master".to_string(),
            role_name: "nonexistent".to_string(),
            first: None,
            max: None,
        };

        let result = realm_role_users_list(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_realm_role_groups_list_success() {
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
            .and(path("/admin/realms/master/roles/admin/groups"))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&groups))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("client creation should succeed");
        let params = RoleGroupsListParams {
            realm: "master".to_string(),
            role_name: "admin".to_string(),
            first: None,
            max: None,
            brief_representation: None,
        };

        let result = realm_role_groups_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
        let returned_groups = result.expect("should return groups");
        assert_eq!(returned_groups.len(), 2);
        assert_eq!(returned_groups[0].name, Some("Engineering".to_string()));
    }

    #[tokio::test]
    async fn test_realm_role_groups_list_with_pagination_and_brief() {
        let mock_server = MockServer::start().await;

        let groups: Vec<GroupRepresentation> = vec![];

        Mock::given(method("GET"))
            .and(path("/admin/realms/master/roles/admin/groups"))
            .and(query_param("first", "5"))
            .and(query_param("max", "10"))
            .and(query_param("briefRepresentation", "true"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&groups))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("client creation should succeed");
        let params = RoleGroupsListParams {
            realm: "master".to_string(),
            role_name: "admin".to_string(),
            first: Some(5),
            max: Some(10),
            brief_representation: Some(true),
        };

        let result = realm_role_groups_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_realm_role_groups_list_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/admin/realms/master/roles/nonexistent/groups"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("client creation should succeed");
        let params = RoleGroupsListParams {
            realm: "master".to_string(),
            role_name: "nonexistent".to_string(),
            first: None,
            max: None,
            brief_representation: None,
        };

        let result = realm_role_groups_list(&client, TEST_TOKEN, &params).await;
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
}
