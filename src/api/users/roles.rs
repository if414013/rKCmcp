//! User role mapping tools for Keycloak Admin REST API.
//!
//! This module provides tools for managing user role mappings including:
//! - Listing realm roles assigned to a user
//! - Adding realm roles to a user
//! - Removing realm roles from a user
//! - Listing available realm roles for a user
//! - Listing client roles assigned to a user
//! - Adding client roles to a user
//! - Removing client roles from a user

use serde::Deserialize;

use crate::api::roles::RoleRepresentation;
use crate::api::{ApiError, KeycloakClient};

/// Parameters for listing realm roles assigned to a user.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserRealmRolesListParams {
    /// The realm name
    pub realm: String,
    /// The user ID
    pub user_id: String,
}

impl UserRealmRolesListParams {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.realm.trim().is_empty() {
            return Err(ApiError::BadRequest("realm is required".to_string()));
        }
        if self.user_id.trim().is_empty() {
            return Err(ApiError::BadRequest("user_id is required".to_string()));
        }
        Ok(())
    }
}

/// Parameters for adding realm roles to a user.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserRealmRolesAddParams {
    /// The realm name
    pub realm: String,
    /// The user ID
    pub user_id: String,
    /// The roles to add (array of role representations with at least id or name)
    pub roles: Vec<RoleRepresentation>,
}

impl UserRealmRolesAddParams {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.realm.trim().is_empty() {
            return Err(ApiError::BadRequest("realm is required".to_string()));
        }
        if self.user_id.trim().is_empty() {
            return Err(ApiError::BadRequest("user_id is required".to_string()));
        }
        if self.roles.is_empty() {
            return Err(ApiError::BadRequest(
                "at least one role is required".to_string(),
            ));
        }
        Ok(())
    }
}

/// Parameters for removing realm roles from a user.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserRealmRolesRemoveParams {
    /// The realm name
    pub realm: String,
    /// The user ID
    pub user_id: String,
    /// The roles to remove (array of role representations with at least id or name)
    pub roles: Vec<RoleRepresentation>,
}

impl UserRealmRolesRemoveParams {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.realm.trim().is_empty() {
            return Err(ApiError::BadRequest("realm is required".to_string()));
        }
        if self.user_id.trim().is_empty() {
            return Err(ApiError::BadRequest("user_id is required".to_string()));
        }
        if self.roles.is_empty() {
            return Err(ApiError::BadRequest(
                "at least one role is required".to_string(),
            ));
        }
        Ok(())
    }
}

/// Parameters for listing available realm roles for a user.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserRealmRolesAvailableParams {
    /// The realm name
    pub realm: String,
    /// The user ID
    pub user_id: String,
}

impl UserRealmRolesAvailableParams {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.realm.trim().is_empty() {
            return Err(ApiError::BadRequest("realm is required".to_string()));
        }
        if self.user_id.trim().is_empty() {
            return Err(ApiError::BadRequest("user_id is required".to_string()));
        }
        Ok(())
    }
}

/// Parameters for listing client roles assigned to a user.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserClientRolesListParams {
    /// The realm name
    pub realm: String,
    /// The user ID
    pub user_id: String,
    /// The client ID (the unique identifier, not client_id)
    pub client: String,
}

impl UserClientRolesListParams {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.realm.trim().is_empty() {
            return Err(ApiError::BadRequest("realm is required".to_string()));
        }
        if self.user_id.trim().is_empty() {
            return Err(ApiError::BadRequest("user_id is required".to_string()));
        }
        if self.client.trim().is_empty() {
            return Err(ApiError::BadRequest("client is required".to_string()));
        }
        Ok(())
    }
}

/// Parameters for adding client roles to a user.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserClientRolesAddParams {
    /// The realm name
    pub realm: String,
    /// The user ID
    pub user_id: String,
    /// The client ID (the unique identifier, not client_id)
    pub client: String,
    /// The roles to add (array of role representations with at least id or name)
    pub roles: Vec<RoleRepresentation>,
}

impl UserClientRolesAddParams {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.realm.trim().is_empty() {
            return Err(ApiError::BadRequest("realm is required".to_string()));
        }
        if self.user_id.trim().is_empty() {
            return Err(ApiError::BadRequest("user_id is required".to_string()));
        }
        if self.client.trim().is_empty() {
            return Err(ApiError::BadRequest("client is required".to_string()));
        }
        if self.roles.is_empty() {
            return Err(ApiError::BadRequest(
                "at least one role is required".to_string(),
            ));
        }
        Ok(())
    }
}

/// Parameters for removing client roles from a user.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserClientRolesRemoveParams {
    /// The realm name
    pub realm: String,
    /// The user ID
    pub user_id: String,
    /// The client ID (the unique identifier, not client_id)
    pub client: String,
    /// The roles to remove (array of role representations with at least id or name)
    pub roles: Vec<RoleRepresentation>,
}

impl UserClientRolesRemoveParams {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.realm.trim().is_empty() {
            return Err(ApiError::BadRequest("realm is required".to_string()));
        }
        if self.user_id.trim().is_empty() {
            return Err(ApiError::BadRequest("user_id is required".to_string()));
        }
        if self.client.trim().is_empty() {
            return Err(ApiError::BadRequest("client is required".to_string()));
        }
        if self.roles.is_empty() {
            return Err(ApiError::BadRequest(
                "at least one role is required".to_string(),
            ));
        }
        Ok(())
    }
}

/// List realm roles assigned to a user.
///
/// Returns all realm-level roles that are directly assigned to the user.
///
/// # Endpoint
/// `GET /admin/realms/{realm}/users/{id}/role-mappings/realm`
pub async fn user_realm_roles_list(
    client: &KeycloakClient,
    token: &str,
    params: &UserRealmRolesListParams,
) -> Result<Vec<RoleRepresentation>, ApiError> {
    params.validate()?;

    let path = format!(
        "/admin/realms/{}/users/{}/role-mappings/realm",
        params.realm, params.user_id
    );
    client.get(&path, token).await
}

/// Add realm roles to a user.
///
/// Assigns the specified realm-level roles to the user.
///
/// # Endpoint
/// `POST /admin/realms/{realm}/users/{id}/role-mappings/realm`
pub async fn user_realm_roles_add(
    client: &KeycloakClient,
    token: &str,
    params: &UserRealmRolesAddParams,
) -> Result<(), ApiError> {
    params.validate()?;

    let path = format!(
        "/admin/realms/{}/users/{}/role-mappings/realm",
        params.realm, params.user_id
    );
    client.post_no_response(&path, token, &params.roles).await
}

/// Remove realm roles from a user.
///
/// Removes the specified realm-level roles from the user.
///
/// # Endpoint
/// `DELETE /admin/realms/{realm}/users/{id}/role-mappings/realm`
pub async fn user_realm_roles_remove(
    client: &KeycloakClient,
    token: &str,
    params: &UserRealmRolesRemoveParams,
) -> Result<(), ApiError> {
    params.validate()?;

    let path = format!(
        "/admin/realms/{}/users/{}/role-mappings/realm",
        params.realm, params.user_id
    );
    client.delete_with_body(&path, token, &params.roles).await
}

/// List available realm roles for a user.
///
/// Returns realm-level roles that are available to be assigned to the user
/// (i.e., roles that the user doesn't already have).
///
/// # Endpoint
/// `GET /admin/realms/{realm}/users/{id}/role-mappings/realm/available`
pub async fn user_realm_roles_available(
    client: &KeycloakClient,
    token: &str,
    params: &UserRealmRolesAvailableParams,
) -> Result<Vec<RoleRepresentation>, ApiError> {
    params.validate()?;

    let path = format!(
        "/admin/realms/{}/users/{}/role-mappings/realm/available",
        params.realm, params.user_id
    );
    client.get(&path, token).await
}

/// List client roles assigned to a user.
///
/// Returns all client-level roles for the specified client that are
/// directly assigned to the user.
///
/// # Endpoint
/// `GET /admin/realms/{realm}/users/{id}/role-mappings/clients/{client}`
pub async fn user_client_roles_list(
    client: &KeycloakClient,
    token: &str,
    params: &UserClientRolesListParams,
) -> Result<Vec<RoleRepresentation>, ApiError> {
    params.validate()?;

    let path = format!(
        "/admin/realms/{}/users/{}/role-mappings/clients/{}",
        params.realm, params.user_id, params.client
    );
    client.get(&path, token).await
}

/// Add client roles to a user.
///
/// Assigns the specified client-level roles to the user.
///
/// # Endpoint
/// `POST /admin/realms/{realm}/users/{id}/role-mappings/clients/{client}`
pub async fn user_client_roles_add(
    client: &KeycloakClient,
    token: &str,
    params: &UserClientRolesAddParams,
) -> Result<(), ApiError> {
    params.validate()?;

    let path = format!(
        "/admin/realms/{}/users/{}/role-mappings/clients/{}",
        params.realm, params.user_id, params.client
    );
    client.post_no_response(&path, token, &params.roles).await
}

/// Remove client roles from a user.
///
/// Removes the specified client-level roles from the user.
///
/// # Endpoint
/// `DELETE /admin/realms/{realm}/users/{id}/role-mappings/clients/{client}`
pub async fn user_client_roles_remove(
    client: &KeycloakClient,
    token: &str,
    params: &UserClientRolesRemoveParams,
) -> Result<(), ApiError> {
    params.validate()?;

    let path = format!(
        "/admin/realms/{}/users/{}/role-mappings/clients/{}",
        params.realm, params.user_id, params.client
    );
    client.delete_with_body(&path, token, &params.roles).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    const TEST_TOKEN: &str = "test-bearer-token";

    mod user_realm_roles_list_params_tests {
        use super::*;

        #[test]
        fn test_validate_success() {
            let params = UserRealmRolesListParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
            };
            assert!(params.validate().is_ok());
        }

        #[test]
        fn test_validate_empty_realm() {
            let params = UserRealmRolesListParams {
                realm: "".to_string(),
                user_id: "123".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_whitespace_realm() {
            let params = UserRealmRolesListParams {
                realm: "   ".to_string(),
                user_id: "123".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_user_id() {
            let params = UserRealmRolesListParams {
                realm: "master".to_string(),
                user_id: "".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }
    }

    mod user_realm_roles_add_params_tests {
        use super::*;

        #[test]
        fn test_validate_success() {
            let params = UserRealmRolesAddParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                roles: vec![RoleRepresentation {
                    id: Some("role-1".to_string()),
                    name: Some("admin".to_string()),
                    ..Default::default()
                }],
            };
            assert!(params.validate().is_ok());
        }

        #[test]
        fn test_validate_empty_realm() {
            let params = UserRealmRolesAddParams {
                realm: "".to_string(),
                user_id: "123".to_string(),
                roles: vec![RoleRepresentation::default()],
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_user_id() {
            let params = UserRealmRolesAddParams {
                realm: "master".to_string(),
                user_id: "".to_string(),
                roles: vec![RoleRepresentation::default()],
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_roles() {
            let params = UserRealmRolesAddParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                roles: vec![],
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }
    }

    mod user_realm_roles_remove_params_tests {
        use super::*;

        #[test]
        fn test_validate_success() {
            let params = UserRealmRolesRemoveParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                roles: vec![RoleRepresentation {
                    id: Some("role-1".to_string()),
                    name: Some("admin".to_string()),
                    ..Default::default()
                }],
            };
            assert!(params.validate().is_ok());
        }

        #[test]
        fn test_validate_empty_realm() {
            let params = UserRealmRolesRemoveParams {
                realm: "".to_string(),
                user_id: "123".to_string(),
                roles: vec![RoleRepresentation::default()],
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_user_id() {
            let params = UserRealmRolesRemoveParams {
                realm: "master".to_string(),
                user_id: "".to_string(),
                roles: vec![RoleRepresentation::default()],
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_roles() {
            let params = UserRealmRolesRemoveParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                roles: vec![],
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }
    }

    mod user_realm_roles_available_params_tests {
        use super::*;

        #[test]
        fn test_validate_success() {
            let params = UserRealmRolesAvailableParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
            };
            assert!(params.validate().is_ok());
        }

        #[test]
        fn test_validate_empty_realm() {
            let params = UserRealmRolesAvailableParams {
                realm: "".to_string(),
                user_id: "123".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_user_id() {
            let params = UserRealmRolesAvailableParams {
                realm: "master".to_string(),
                user_id: "".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }
    }

    mod user_client_roles_list_params_tests {
        use super::*;

        #[test]
        fn test_validate_success() {
            let params = UserClientRolesListParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                client: "my-client-id".to_string(),
            };
            assert!(params.validate().is_ok());
        }

        #[test]
        fn test_validate_empty_realm() {
            let params = UserClientRolesListParams {
                realm: "".to_string(),
                user_id: "123".to_string(),
                client: "my-client-id".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_user_id() {
            let params = UserClientRolesListParams {
                realm: "master".to_string(),
                user_id: "".to_string(),
                client: "my-client-id".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_client() {
            let params = UserClientRolesListParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                client: "".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }
    }

    mod user_client_roles_add_params_tests {
        use super::*;

        #[test]
        fn test_validate_success() {
            let params = UserClientRolesAddParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                client: "my-client-id".to_string(),
                roles: vec![RoleRepresentation {
                    id: Some("role-1".to_string()),
                    name: Some("client-admin".to_string()),
                    ..Default::default()
                }],
            };
            assert!(params.validate().is_ok());
        }

        #[test]
        fn test_validate_empty_realm() {
            let params = UserClientRolesAddParams {
                realm: "".to_string(),
                user_id: "123".to_string(),
                client: "my-client-id".to_string(),
                roles: vec![RoleRepresentation::default()],
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_user_id() {
            let params = UserClientRolesAddParams {
                realm: "master".to_string(),
                user_id: "".to_string(),
                client: "my-client-id".to_string(),
                roles: vec![RoleRepresentation::default()],
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_client() {
            let params = UserClientRolesAddParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                client: "".to_string(),
                roles: vec![RoleRepresentation::default()],
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_roles() {
            let params = UserClientRolesAddParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                client: "my-client-id".to_string(),
                roles: vec![],
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }
    }

    mod user_client_roles_remove_params_tests {
        use super::*;

        #[test]
        fn test_validate_success() {
            let params = UserClientRolesRemoveParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                client: "my-client-id".to_string(),
                roles: vec![RoleRepresentation {
                    id: Some("role-1".to_string()),
                    name: Some("client-admin".to_string()),
                    ..Default::default()
                }],
            };
            assert!(params.validate().is_ok());
        }

        #[test]
        fn test_validate_empty_realm() {
            let params = UserClientRolesRemoveParams {
                realm: "".to_string(),
                user_id: "123".to_string(),
                client: "my-client-id".to_string(),
                roles: vec![RoleRepresentation::default()],
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_user_id() {
            let params = UserClientRolesRemoveParams {
                realm: "master".to_string(),
                user_id: "".to_string(),
                client: "my-client-id".to_string(),
                roles: vec![RoleRepresentation::default()],
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_client() {
            let params = UserClientRolesRemoveParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                client: "".to_string(),
                roles: vec![RoleRepresentation::default()],
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_roles() {
            let params = UserClientRolesRemoveParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                client: "my-client-id".to_string(),
                roles: vec![],
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }
    }

    mod user_realm_roles_list_tests {
        use super::*;

        #[tokio::test]
        async fn test_list_success() {
            let mock_server = MockServer::start().await;

            let expected_roles = vec![
                RoleRepresentation {
                    id: Some("role-1".to_string()),
                    name: Some("admin".to_string()),
                    description: Some("Admin role".to_string()),
                    ..Default::default()
                },
                RoleRepresentation {
                    id: Some("role-2".to_string()),
                    name: Some("user".to_string()),
                    description: Some("User role".to_string()),
                    ..Default::default()
                },
            ];

            Mock::given(method("GET"))
                .and(path("/admin/realms/master/users/123/role-mappings/realm"))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .respond_with(ResponseTemplate::new(200).set_body_json(&expected_roles))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserRealmRolesListParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
            };

            let roles = user_realm_roles_list(&client, TEST_TOKEN, &params)
                .await
                .expect("Failed to list realm roles");
            assert_eq!(roles.len(), 2);
            assert_eq!(roles[0].name.as_deref(), Some("admin"));
            assert_eq!(roles[1].name.as_deref(), Some("user"));
        }

        #[tokio::test]
        async fn test_list_empty() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path("/admin/realms/master/users/123/role-mappings/realm"))
                .respond_with(
                    ResponseTemplate::new(200).set_body_json::<Vec<RoleRepresentation>>(vec![]),
                )
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserRealmRolesListParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
            };

            let roles = user_realm_roles_list(&client, TEST_TOKEN, &params)
                .await
                .expect("Failed to list realm roles");
            assert!(roles.is_empty());
        }

        #[tokio::test]
        async fn test_list_user_not_found() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path("/admin/realms/master/users/999/role-mappings/realm"))
                .respond_with(ResponseTemplate::new(404))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserRealmRolesListParams {
                realm: "master".to_string(),
                user_id: "999".to_string(),
            };

            let result = user_realm_roles_list(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::NotFound)));
        }

        #[tokio::test]
        async fn test_list_validation_fails() {
            let mock_server = MockServer::start().await;
            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserRealmRolesListParams {
                realm: "".to_string(),
                user_id: "123".to_string(),
            };

            let result = user_realm_roles_list(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::BadRequest(_))));
        }
    }

    mod user_realm_roles_add_tests {
        use super::*;

        #[tokio::test]
        async fn test_add_success() {
            let mock_server = MockServer::start().await;

            let roles_to_add = vec![RoleRepresentation {
                id: Some("role-1".to_string()),
                name: Some("admin".to_string()),
                ..Default::default()
            }];

            Mock::given(method("POST"))
                .and(path("/admin/realms/master/users/123/role-mappings/realm"))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .and(header("content-type", "application/json"))
                .respond_with(ResponseTemplate::new(204))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserRealmRolesAddParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                roles: roles_to_add,
            };

            let result = user_realm_roles_add(&client, TEST_TOKEN, &params).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_add_multiple_roles() {
            let mock_server = MockServer::start().await;

            let roles_to_add = vec![
                RoleRepresentation {
                    id: Some("role-1".to_string()),
                    name: Some("admin".to_string()),
                    ..Default::default()
                },
                RoleRepresentation {
                    id: Some("role-2".to_string()),
                    name: Some("moderator".to_string()),
                    ..Default::default()
                },
            ];

            Mock::given(method("POST"))
                .and(path("/admin/realms/master/users/123/role-mappings/realm"))
                .respond_with(ResponseTemplate::new(204))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserRealmRolesAddParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                roles: roles_to_add,
            };

            let result = user_realm_roles_add(&client, TEST_TOKEN, &params).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_add_user_not_found() {
            let mock_server = MockServer::start().await;

            Mock::given(method("POST"))
                .and(path("/admin/realms/master/users/999/role-mappings/realm"))
                .respond_with(ResponseTemplate::new(404))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserRealmRolesAddParams {
                realm: "master".to_string(),
                user_id: "999".to_string(),
                roles: vec![RoleRepresentation {
                    id: Some("role-1".to_string()),
                    ..Default::default()
                }],
            };

            let result = user_realm_roles_add(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::NotFound)));
        }

        #[tokio::test]
        async fn test_add_validation_fails() {
            let mock_server = MockServer::start().await;
            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserRealmRolesAddParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                roles: vec![],
            };

            let result = user_realm_roles_add(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::BadRequest(_))));
        }
    }

    mod user_realm_roles_remove_tests {
        use super::*;

        #[tokio::test]
        async fn test_remove_success() {
            let mock_server = MockServer::start().await;

            let roles_to_remove = vec![RoleRepresentation {
                id: Some("role-1".to_string()),
                name: Some("admin".to_string()),
                ..Default::default()
            }];

            Mock::given(method("DELETE"))
                .and(path("/admin/realms/master/users/123/role-mappings/realm"))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .and(header("content-type", "application/json"))
                .respond_with(ResponseTemplate::new(204))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserRealmRolesRemoveParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                roles: roles_to_remove,
            };

            let result = user_realm_roles_remove(&client, TEST_TOKEN, &params).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_remove_user_not_found() {
            let mock_server = MockServer::start().await;

            Mock::given(method("DELETE"))
                .and(path("/admin/realms/master/users/999/role-mappings/realm"))
                .respond_with(ResponseTemplate::new(404))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserRealmRolesRemoveParams {
                realm: "master".to_string(),
                user_id: "999".to_string(),
                roles: vec![RoleRepresentation {
                    id: Some("role-1".to_string()),
                    ..Default::default()
                }],
            };

            let result = user_realm_roles_remove(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::NotFound)));
        }

        #[tokio::test]
        async fn test_remove_validation_fails() {
            let mock_server = MockServer::start().await;
            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserRealmRolesRemoveParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                roles: vec![],
            };

            let result = user_realm_roles_remove(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::BadRequest(_))));
        }
    }

    mod user_realm_roles_available_tests {
        use super::*;

        #[tokio::test]
        async fn test_available_success() {
            let mock_server = MockServer::start().await;

            let available_roles = vec![
                RoleRepresentation {
                    id: Some("role-3".to_string()),
                    name: Some("moderator".to_string()),
                    ..Default::default()
                },
                RoleRepresentation {
                    id: Some("role-4".to_string()),
                    name: Some("guest".to_string()),
                    ..Default::default()
                },
            ];

            Mock::given(method("GET"))
                .and(path(
                    "/admin/realms/master/users/123/role-mappings/realm/available",
                ))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .respond_with(ResponseTemplate::new(200).set_body_json(&available_roles))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserRealmRolesAvailableParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
            };

            let roles = user_realm_roles_available(&client, TEST_TOKEN, &params)
                .await
                .expect("Failed to list available roles");
            assert_eq!(roles.len(), 2);
            assert_eq!(roles[0].name.as_deref(), Some("moderator"));
        }

        #[tokio::test]
        async fn test_available_empty() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path(
                    "/admin/realms/master/users/123/role-mappings/realm/available",
                ))
                .respond_with(
                    ResponseTemplate::new(200).set_body_json::<Vec<RoleRepresentation>>(vec![]),
                )
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserRealmRolesAvailableParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
            };

            let roles = user_realm_roles_available(&client, TEST_TOKEN, &params)
                .await
                .expect("Failed to list available roles");
            assert!(roles.is_empty());
        }

        #[tokio::test]
        async fn test_available_user_not_found() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path(
                    "/admin/realms/master/users/999/role-mappings/realm/available",
                ))
                .respond_with(ResponseTemplate::new(404))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserRealmRolesAvailableParams {
                realm: "master".to_string(),
                user_id: "999".to_string(),
            };

            let result = user_realm_roles_available(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::NotFound)));
        }
    }

    mod user_client_roles_list_tests {
        use super::*;

        #[tokio::test]
        async fn test_list_success() {
            let mock_server = MockServer::start().await;

            let expected_roles = vec![RoleRepresentation {
                id: Some("client-role-1".to_string()),
                name: Some("client-admin".to_string()),
                description: Some("Client admin role".to_string()),
                client_role: Some(true),
                ..Default::default()
            }];

            Mock::given(method("GET"))
                .and(path(
                    "/admin/realms/master/users/123/role-mappings/clients/my-client-id",
                ))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .respond_with(ResponseTemplate::new(200).set_body_json(&expected_roles))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserClientRolesListParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                client: "my-client-id".to_string(),
            };

            let roles = user_client_roles_list(&client, TEST_TOKEN, &params)
                .await
                .expect("Failed to list client roles");
            assert_eq!(roles.len(), 1);
            assert_eq!(roles[0].name.as_deref(), Some("client-admin"));
            assert_eq!(roles[0].client_role, Some(true));
        }

        #[tokio::test]
        async fn test_list_empty() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path(
                    "/admin/realms/master/users/123/role-mappings/clients/my-client-id",
                ))
                .respond_with(
                    ResponseTemplate::new(200).set_body_json::<Vec<RoleRepresentation>>(vec![]),
                )
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserClientRolesListParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                client: "my-client-id".to_string(),
            };

            let roles = user_client_roles_list(&client, TEST_TOKEN, &params)
                .await
                .expect("Failed to list client roles");
            assert!(roles.is_empty());
        }

        #[tokio::test]
        async fn test_list_user_not_found() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path(
                    "/admin/realms/master/users/999/role-mappings/clients/my-client-id",
                ))
                .respond_with(ResponseTemplate::new(404))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserClientRolesListParams {
                realm: "master".to_string(),
                user_id: "999".to_string(),
                client: "my-client-id".to_string(),
            };

            let result = user_client_roles_list(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::NotFound)));
        }

        #[tokio::test]
        async fn test_list_client_not_found() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path(
                    "/admin/realms/master/users/123/role-mappings/clients/nonexistent",
                ))
                .respond_with(ResponseTemplate::new(404))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserClientRolesListParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                client: "nonexistent".to_string(),
            };

            let result = user_client_roles_list(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::NotFound)));
        }
    }

    mod user_client_roles_add_tests {
        use super::*;

        #[tokio::test]
        async fn test_add_success() {
            let mock_server = MockServer::start().await;

            let roles_to_add = vec![RoleRepresentation {
                id: Some("client-role-1".to_string()),
                name: Some("client-admin".to_string()),
                ..Default::default()
            }];

            Mock::given(method("POST"))
                .and(path(
                    "/admin/realms/master/users/123/role-mappings/clients/my-client-id",
                ))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .and(header("content-type", "application/json"))
                .respond_with(ResponseTemplate::new(204))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserClientRolesAddParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                client: "my-client-id".to_string(),
                roles: roles_to_add,
            };

            let result = user_client_roles_add(&client, TEST_TOKEN, &params).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_add_multiple_roles() {
            let mock_server = MockServer::start().await;

            let roles_to_add = vec![
                RoleRepresentation {
                    id: Some("client-role-1".to_string()),
                    name: Some("client-admin".to_string()),
                    ..Default::default()
                },
                RoleRepresentation {
                    id: Some("client-role-2".to_string()),
                    name: Some("client-user".to_string()),
                    ..Default::default()
                },
            ];

            Mock::given(method("POST"))
                .and(path(
                    "/admin/realms/master/users/123/role-mappings/clients/my-client-id",
                ))
                .respond_with(ResponseTemplate::new(204))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserClientRolesAddParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                client: "my-client-id".to_string(),
                roles: roles_to_add,
            };

            let result = user_client_roles_add(&client, TEST_TOKEN, &params).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_add_user_not_found() {
            let mock_server = MockServer::start().await;

            Mock::given(method("POST"))
                .and(path(
                    "/admin/realms/master/users/999/role-mappings/clients/my-client-id",
                ))
                .respond_with(ResponseTemplate::new(404))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserClientRolesAddParams {
                realm: "master".to_string(),
                user_id: "999".to_string(),
                client: "my-client-id".to_string(),
                roles: vec![RoleRepresentation {
                    id: Some("client-role-1".to_string()),
                    ..Default::default()
                }],
            };

            let result = user_client_roles_add(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::NotFound)));
        }

        #[tokio::test]
        async fn test_add_validation_fails() {
            let mock_server = MockServer::start().await;
            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserClientRolesAddParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                client: "my-client-id".to_string(),
                roles: vec![],
            };

            let result = user_client_roles_add(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::BadRequest(_))));
        }
    }

    mod user_client_roles_remove_tests {
        use super::*;

        #[tokio::test]
        async fn test_remove_success() {
            let mock_server = MockServer::start().await;

            let roles_to_remove = vec![RoleRepresentation {
                id: Some("client-role-1".to_string()),
                name: Some("client-admin".to_string()),
                ..Default::default()
            }];

            Mock::given(method("DELETE"))
                .and(path(
                    "/admin/realms/master/users/123/role-mappings/clients/my-client-id",
                ))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .and(header("content-type", "application/json"))
                .respond_with(ResponseTemplate::new(204))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserClientRolesRemoveParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                client: "my-client-id".to_string(),
                roles: roles_to_remove,
            };

            let result = user_client_roles_remove(&client, TEST_TOKEN, &params).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_remove_user_not_found() {
            let mock_server = MockServer::start().await;

            Mock::given(method("DELETE"))
                .and(path(
                    "/admin/realms/master/users/999/role-mappings/clients/my-client-id",
                ))
                .respond_with(ResponseTemplate::new(404))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserClientRolesRemoveParams {
                realm: "master".to_string(),
                user_id: "999".to_string(),
                client: "my-client-id".to_string(),
                roles: vec![RoleRepresentation {
                    id: Some("client-role-1".to_string()),
                    ..Default::default()
                }],
            };

            let result = user_client_roles_remove(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::NotFound)));
        }

        #[tokio::test]
        async fn test_remove_validation_fails() {
            let mock_server = MockServer::start().await;
            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserClientRolesRemoveParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                client: "my-client-id".to_string(),
                roles: vec![],
            };

            let result = user_client_roles_remove(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::BadRequest(_))));
        }
    }
}
