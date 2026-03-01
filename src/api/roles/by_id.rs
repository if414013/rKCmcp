//! Roles-by-ID API functions for accessing roles by UUID.
//!
//! This module provides functionality for managing roles using their unique ID (UUID)
//! instead of their name. This is useful when role names may change or contain special characters.

use crate::api::{ApiError, KeycloakClient};
use serde::Deserialize;

use super::RoleRepresentation;

/// Parameters for getting a role by ID.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RoleByIdGetParams {
    /// The realm name (not id!)
    pub realm: String,

    /// The role ID (UUID)
    pub role_id: String,
}

/// Parameters for updating a role by ID.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RoleByIdUpdateParams {
    /// The realm name (not id!)
    pub realm: String,

    /// The role ID (UUID)
    pub role_id: String,

    /// The role representation with updated fields
    pub role: RoleRepresentation,
}

/// Parameters for deleting a role by ID.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RoleByIdDeleteParams {
    /// The realm name (not id!)
    pub realm: String,

    /// The role ID (UUID)
    pub role_id: String,
}

/// Parameters for listing composite roles of a role by ID.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RoleByIdCompositesListParams {
    /// The realm name (not id!)
    pub realm: String,

    /// The role ID (UUID)
    pub role_id: String,
}

/// Parameters for adding composite roles to a role by ID.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RoleByIdCompositesAddParams {
    /// The realm name (not id!)
    pub realm: String,

    /// The parent role ID (UUID)
    pub role_id: String,

    /// The roles to add as composites (array of role representations)
    pub roles: Vec<RoleRepresentation>,
}

/// Parameters for removing composite roles from a role by ID.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RoleByIdCompositesRemoveParams {
    /// The realm name (not id!)
    pub realm: String,

    /// The parent role ID (UUID)
    pub role_id: String,

    /// The roles to remove from composites (array of role representations)
    pub roles: Vec<RoleRepresentation>,
}

/// Get a role by its ID (UUID).
///
/// Returns the role representation for the specified role ID.
pub async fn role_by_id_get(
    client: &KeycloakClient,
    token: &str,
    params: &RoleByIdGetParams,
) -> Result<RoleRepresentation, ApiError> {
    let path = format!(
        "/admin/realms/{}/roles-by-id/{}",
        params.realm, params.role_id
    );
    client.get(&path, token).await
}

/// Update a role by its ID (UUID).
///
/// Updates the role with the provided representation.
pub async fn role_by_id_update(
    client: &KeycloakClient,
    token: &str,
    params: &RoleByIdUpdateParams,
) -> Result<(), ApiError> {
    let path = format!(
        "/admin/realms/{}/roles-by-id/{}",
        params.realm, params.role_id
    );
    client.put(&path, token, &params.role).await
}

/// Delete a role by its ID (UUID).
///
/// Permanently removes the role from the realm.
pub async fn role_by_id_delete(
    client: &KeycloakClient,
    token: &str,
    params: &RoleByIdDeleteParams,
) -> Result<(), ApiError> {
    let path = format!(
        "/admin/realms/{}/roles-by-id/{}",
        params.realm, params.role_id
    );
    client.delete(&path, token).await
}

/// List the composite roles of a role by ID.
///
/// Returns all roles that are included in the specified role as composites.
pub async fn role_by_id_composites_list(
    client: &KeycloakClient,
    token: &str,
    params: &RoleByIdCompositesListParams,
) -> Result<Vec<RoleRepresentation>, ApiError> {
    let path = format!(
        "/admin/realms/{}/roles-by-id/{}/composites",
        params.realm, params.role_id
    );
    client.get(&path, token).await
}

/// Add composite roles to a role by ID.
///
/// Makes the specified role a composite role by adding other roles as its components.
pub async fn role_by_id_composites_add(
    client: &KeycloakClient,
    token: &str,
    params: &RoleByIdCompositesAddParams,
) -> Result<(), ApiError> {
    let path = format!(
        "/admin/realms/{}/roles-by-id/{}/composites",
        params.realm, params.role_id
    );
    client.post_no_response(&path, token, &params.roles).await
}

/// Remove composite roles from a role by ID.
///
/// Removes the specified roles from the parent role's composites.
pub async fn role_by_id_composites_remove(
    client: &KeycloakClient,
    token: &str,
    params: &RoleByIdCompositesRemoveParams,
) -> Result<(), ApiError> {
    let path = format!(
        "/admin/realms/{}/roles-by-id/{}/composites",
        params.realm, params.role_id
    );
    client.delete_with_body(&path, token, &params.roles).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    const TEST_TOKEN: &str = "test-bearer-token";
    const TEST_ROLE_ID: &str = "550e8400-e29b-41d4-a716-446655440000";

    #[tokio::test]
    async fn test_role_by_id_get_success() {
        let mock_server = MockServer::start().await;

        let role = RoleRepresentation {
            id: Some(TEST_ROLE_ID.to_string()),
            name: Some("admin".to_string()),
            description: Some("Administrator role".to_string()),
            composite: Some(false),
            client_role: Some(false),
            container_id: Some("master".to_string()),
            attributes: None,
        };

        Mock::given(method("GET"))
            .and(path(format!(
                "/admin/realms/master/roles-by-id/{}",
                TEST_ROLE_ID
            )))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&role))
            .mount(&mock_server)
            .await;

        let client =
            KeycloakClient::new(mock_server.uri()).expect("client creation should succeed");
        let params = RoleByIdGetParams {
            realm: "master".to_string(),
            role_id: TEST_ROLE_ID.to_string(),
        };

        let result = role_by_id_get(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
        let returned_role = result.expect("should return role");
        assert_eq!(returned_role.name, Some("admin".to_string()));
        assert_eq!(returned_role.id, Some(TEST_ROLE_ID.to_string()));
    }

    #[tokio::test]
    async fn test_role_by_id_get_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(format!(
                "/admin/realms/master/roles-by-id/{}",
                TEST_ROLE_ID
            )))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client =
            KeycloakClient::new(mock_server.uri()).expect("client creation should succeed");
        let params = RoleByIdGetParams {
            realm: "master".to_string(),
            role_id: TEST_ROLE_ID.to_string(),
        };

        let result = role_by_id_get(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_role_by_id_update_success() {
        let mock_server = MockServer::start().await;

        let updated_role = RoleRepresentation {
            id: Some(TEST_ROLE_ID.to_string()),
            name: Some("updated-admin".to_string()),
            description: Some("Updated admin role".to_string()),
            ..Default::default()
        };

        Mock::given(method("PUT"))
            .and(path(format!(
                "/admin/realms/master/roles-by-id/{}",
                TEST_ROLE_ID
            )))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .and(header("content-type", "application/json"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let client =
            KeycloakClient::new(mock_server.uri()).expect("client creation should succeed");
        let params = RoleByIdUpdateParams {
            realm: "master".to_string(),
            role_id: TEST_ROLE_ID.to_string(),
            role: updated_role,
        };

        let result = role_by_id_update(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_role_by_id_update_not_found() {
        let mock_server = MockServer::start().await;

        let updated_role = RoleRepresentation {
            name: Some("nonexistent".to_string()),
            ..Default::default()
        };

        Mock::given(method("PUT"))
            .and(path(format!(
                "/admin/realms/master/roles-by-id/{}",
                TEST_ROLE_ID
            )))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client =
            KeycloakClient::new(mock_server.uri()).expect("client creation should succeed");
        let params = RoleByIdUpdateParams {
            realm: "master".to_string(),
            role_id: TEST_ROLE_ID.to_string(),
            role: updated_role,
        };

        let result = role_by_id_update(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_role_by_id_delete_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path(format!(
                "/admin/realms/master/roles-by-id/{}",
                TEST_ROLE_ID
            )))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let client =
            KeycloakClient::new(mock_server.uri()).expect("client creation should succeed");
        let params = RoleByIdDeleteParams {
            realm: "master".to_string(),
            role_id: TEST_ROLE_ID.to_string(),
        };

        let result = role_by_id_delete(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_role_by_id_delete_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path(format!(
                "/admin/realms/master/roles-by-id/{}",
                TEST_ROLE_ID
            )))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client =
            KeycloakClient::new(mock_server.uri()).expect("client creation should succeed");
        let params = RoleByIdDeleteParams {
            realm: "master".to_string(),
            role_id: TEST_ROLE_ID.to_string(),
        };

        let result = role_by_id_delete(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_role_by_id_composites_list_success() {
        let mock_server = MockServer::start().await;

        let composite_roles = vec![
            RoleRepresentation {
                id: Some("child-id-1".to_string()),
                name: Some("child-role-1".to_string()),
                description: Some("First child role".to_string()),
                ..Default::default()
            },
            RoleRepresentation {
                id: Some("child-id-2".to_string()),
                name: Some("child-role-2".to_string()),
                description: Some("Second child role".to_string()),
                ..Default::default()
            },
        ];

        Mock::given(method("GET"))
            .and(path(format!(
                "/admin/realms/master/roles-by-id/{}/composites",
                TEST_ROLE_ID
            )))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&composite_roles))
            .mount(&mock_server)
            .await;

        let client =
            KeycloakClient::new(mock_server.uri()).expect("client creation should succeed");
        let params = RoleByIdCompositesListParams {
            realm: "master".to_string(),
            role_id: TEST_ROLE_ID.to_string(),
        };

        let result = role_by_id_composites_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
        let roles = result.expect("should return roles");
        assert_eq!(roles.len(), 2);
        assert_eq!(roles[0].name, Some("child-role-1".to_string()));
    }

    #[tokio::test]
    async fn test_role_by_id_composites_list_empty() {
        let mock_server = MockServer::start().await;

        let composite_roles: Vec<RoleRepresentation> = vec![];

        Mock::given(method("GET"))
            .and(path(format!(
                "/admin/realms/master/roles-by-id/{}/composites",
                TEST_ROLE_ID
            )))
            .respond_with(ResponseTemplate::new(200).set_body_json(&composite_roles))
            .mount(&mock_server)
            .await;

        let client =
            KeycloakClient::new(mock_server.uri()).expect("client creation should succeed");
        let params = RoleByIdCompositesListParams {
            realm: "master".to_string(),
            role_id: TEST_ROLE_ID.to_string(),
        };

        let result = role_by_id_composites_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
        let roles = result.expect("should return empty list");
        assert!(roles.is_empty());
    }

    #[tokio::test]
    async fn test_role_by_id_composites_add_success() {
        let mock_server = MockServer::start().await;

        let roles_to_add = vec![RoleRepresentation {
            id: Some("child-id".to_string()),
            name: Some("child-role".to_string()),
            ..Default::default()
        }];

        Mock::given(method("POST"))
            .and(path(format!(
                "/admin/realms/master/roles-by-id/{}/composites",
                TEST_ROLE_ID
            )))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .and(header("content-type", "application/json"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let client =
            KeycloakClient::new(mock_server.uri()).expect("client creation should succeed");
        let params = RoleByIdCompositesAddParams {
            realm: "master".to_string(),
            role_id: TEST_ROLE_ID.to_string(),
            roles: roles_to_add,
        };

        let result = role_by_id_composites_add(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_role_by_id_composites_add_not_found() {
        let mock_server = MockServer::start().await;

        let roles_to_add = vec![RoleRepresentation {
            id: Some("nonexistent-id".to_string()),
            name: Some("nonexistent".to_string()),
            ..Default::default()
        }];

        Mock::given(method("POST"))
            .and(path(format!(
                "/admin/realms/master/roles-by-id/{}/composites",
                TEST_ROLE_ID
            )))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client =
            KeycloakClient::new(mock_server.uri()).expect("client creation should succeed");
        let params = RoleByIdCompositesAddParams {
            realm: "master".to_string(),
            role_id: TEST_ROLE_ID.to_string(),
            roles: roles_to_add,
        };

        let result = role_by_id_composites_add(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_role_by_id_composites_remove_success() {
        let mock_server = MockServer::start().await;

        let roles_to_remove = vec![RoleRepresentation {
            id: Some("child-id".to_string()),
            name: Some("child-role".to_string()),
            ..Default::default()
        }];

        Mock::given(method("DELETE"))
            .and(path(format!(
                "/admin/realms/master/roles-by-id/{}/composites",
                TEST_ROLE_ID
            )))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .and(header("content-type", "application/json"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let client =
            KeycloakClient::new(mock_server.uri()).expect("client creation should succeed");
        let params = RoleByIdCompositesRemoveParams {
            realm: "master".to_string(),
            role_id: TEST_ROLE_ID.to_string(),
            roles: roles_to_remove,
        };

        let result = role_by_id_composites_remove(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_role_by_id_composites_remove_not_found() {
        let mock_server = MockServer::start().await;

        let roles_to_remove = vec![RoleRepresentation {
            id: Some("nonexistent-id".to_string()),
            name: Some("nonexistent".to_string()),
            ..Default::default()
        }];

        Mock::given(method("DELETE"))
            .and(path(format!(
                "/admin/realms/master/roles-by-id/{}/composites",
                TEST_ROLE_ID
            )))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client =
            KeycloakClient::new(mock_server.uri()).expect("client creation should succeed");
        let params = RoleByIdCompositesRemoveParams {
            realm: "master".to_string(),
            role_id: TEST_ROLE_ID.to_string(),
            roles: roles_to_remove,
        };

        let result = role_by_id_composites_remove(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_role_by_id_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(format!(
                "/admin/realms/master/roles-by-id/{}",
                TEST_ROLE_ID
            )))
            .respond_with(ResponseTemplate::new(401))
            .mount(&mock_server)
            .await;

        let client =
            KeycloakClient::new(mock_server.uri()).expect("client creation should succeed");
        let params = RoleByIdGetParams {
            realm: "master".to_string(),
            role_id: TEST_ROLE_ID.to_string(),
        };

        let result = role_by_id_get(&client, "invalid-token", &params).await;
        assert!(matches!(result, Err(ApiError::Unauthorized)));
    }

    #[tokio::test]
    async fn test_role_by_id_forbidden() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(format!(
                "/admin/realms/master/roles-by-id/{}",
                TEST_ROLE_ID
            )))
            .respond_with(ResponseTemplate::new(403))
            .mount(&mock_server)
            .await;

        let client =
            KeycloakClient::new(mock_server.uri()).expect("client creation should succeed");
        let params = RoleByIdGetParams {
            realm: "master".to_string(),
            role_id: TEST_ROLE_ID.to_string(),
        };

        let result = role_by_id_get(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::Forbidden)));
    }
}
