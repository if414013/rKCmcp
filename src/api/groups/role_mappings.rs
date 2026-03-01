//! Group role mapping tools for Keycloak Admin REST API.
//!
//! This module provides tools for managing group role mappings including:
//! - Listing realm roles assigned to a group
//! - Adding realm roles to a group
//! - Removing realm roles from a group
//! - Listing available realm roles for a group
//! - Listing client roles assigned to a group
//! - Adding client roles to a group
//! - Removing client roles from a group

use serde::Deserialize;

use crate::api::roles::RoleRepresentation;
use crate::api::{ApiError, KeycloakClient};

/// Parameters for listing realm roles assigned to a group.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GroupRealmRolesListParams {
    /// The realm name
    pub realm: String,
    /// The group ID
    pub group_id: String,
}

impl GroupRealmRolesListParams {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.realm.trim().is_empty() {
            return Err(ApiError::BadRequest("realm is required".to_string()));
        }
        if self.group_id.trim().is_empty() {
            return Err(ApiError::BadRequest("group_id is required".to_string()));
        }
        Ok(())
    }
}

/// Parameters for adding realm roles to a group.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GroupRealmRolesAddParams {
    /// The realm name
    pub realm: String,
    /// The group ID
    pub group_id: String,
    /// The roles to add (array of role representations with at least id or name)
    pub roles: Vec<RoleRepresentation>,
}

impl GroupRealmRolesAddParams {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.realm.trim().is_empty() {
            return Err(ApiError::BadRequest("realm is required".to_string()));
        }
        if self.group_id.trim().is_empty() {
            return Err(ApiError::BadRequest("group_id is required".to_string()));
        }
        if self.roles.is_empty() {
            return Err(ApiError::BadRequest(
                "at least one role is required".to_string(),
            ));
        }
        Ok(())
    }
}

/// Parameters for removing realm roles from a group.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GroupRealmRolesRemoveParams {
    /// The realm name
    pub realm: String,
    /// The group ID
    pub group_id: String,
    /// The roles to remove (array of role representations with at least id or name)
    pub roles: Vec<RoleRepresentation>,
}

impl GroupRealmRolesRemoveParams {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.realm.trim().is_empty() {
            return Err(ApiError::BadRequest("realm is required".to_string()));
        }
        if self.group_id.trim().is_empty() {
            return Err(ApiError::BadRequest("group_id is required".to_string()));
        }
        if self.roles.is_empty() {
            return Err(ApiError::BadRequest(
                "at least one role is required".to_string(),
            ));
        }
        Ok(())
    }
}

/// Parameters for listing available realm roles for a group.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GroupRealmRolesAvailableParams {
    /// The realm name
    pub realm: String,
    /// The group ID
    pub group_id: String,
}

impl GroupRealmRolesAvailableParams {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.realm.trim().is_empty() {
            return Err(ApiError::BadRequest("realm is required".to_string()));
        }
        if self.group_id.trim().is_empty() {
            return Err(ApiError::BadRequest("group_id is required".to_string()));
        }
        Ok(())
    }
}

/// Parameters for listing client roles assigned to a group.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GroupClientRolesListParams {
    /// The realm name
    pub realm: String,
    /// The group ID
    pub group_id: String,
    /// The client ID (the unique identifier, not client_id)
    pub client: String,
}

impl GroupClientRolesListParams {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.realm.trim().is_empty() {
            return Err(ApiError::BadRequest("realm is required".to_string()));
        }
        if self.group_id.trim().is_empty() {
            return Err(ApiError::BadRequest("group_id is required".to_string()));
        }
        if self.client.trim().is_empty() {
            return Err(ApiError::BadRequest("client is required".to_string()));
        }
        Ok(())
    }
}

/// Parameters for adding client roles to a group.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GroupClientRolesAddParams {
    /// The realm name
    pub realm: String,
    /// The group ID
    pub group_id: String,
    /// The client ID (the unique identifier, not client_id)
    pub client: String,
    /// The roles to add (array of role representations with at least id or name)
    pub roles: Vec<RoleRepresentation>,
}

impl GroupClientRolesAddParams {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.realm.trim().is_empty() {
            return Err(ApiError::BadRequest("realm is required".to_string()));
        }
        if self.group_id.trim().is_empty() {
            return Err(ApiError::BadRequest("group_id is required".to_string()));
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

/// Parameters for removing client roles from a group.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GroupClientRolesRemoveParams {
    /// The realm name
    pub realm: String,
    /// The group ID
    pub group_id: String,
    /// The client ID (the unique identifier, not client_id)
    pub client: String,
    /// The roles to remove (array of role representations with at least id or name)
    pub roles: Vec<RoleRepresentation>,
}

impl GroupClientRolesRemoveParams {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.realm.trim().is_empty() {
            return Err(ApiError::BadRequest("realm is required".to_string()));
        }
        if self.group_id.trim().is_empty() {
            return Err(ApiError::BadRequest("group_id is required".to_string()));
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

/// List realm roles assigned to a group.
///
/// Returns all realm-level roles that are directly assigned to the group.
///
/// # Endpoint
/// `GET /admin/realms/{realm}/groups/{id}/role-mappings/realm`
pub async fn group_realm_roles_list(
    client: &KeycloakClient,
    token: &str,
    params: &GroupRealmRolesListParams,
) -> Result<Vec<RoleRepresentation>, ApiError> {
    params.validate()?;

    let path = format!(
        "/admin/realms/{}/groups/{}/role-mappings/realm",
        params.realm,
        urlencoding::encode(&params.group_id)
    );
    client.get(&path, token).await
}

/// Add realm roles to a group.
///
/// Assigns the specified realm-level roles to the group.
///
/// # Endpoint
/// `POST /admin/realms/{realm}/groups/{id}/role-mappings/realm`
pub async fn group_realm_roles_add(
    client: &KeycloakClient,
    token: &str,
    params: &GroupRealmRolesAddParams,
) -> Result<(), ApiError> {
    params.validate()?;

    let path = format!(
        "/admin/realms/{}/groups/{}/role-mappings/realm",
        params.realm,
        urlencoding::encode(&params.group_id)
    );
    client.post_no_response(&path, token, &params.roles).await
}

/// Remove realm roles from a group.
///
/// Removes the specified realm-level roles from the group.
///
/// # Endpoint
/// `DELETE /admin/realms/{realm}/groups/{id}/role-mappings/realm`
pub async fn group_realm_roles_remove(
    client: &KeycloakClient,
    token: &str,
    params: &GroupRealmRolesRemoveParams,
) -> Result<(), ApiError> {
    params.validate()?;

    let path = format!(
        "/admin/realms/{}/groups/{}/role-mappings/realm",
        params.realm,
        urlencoding::encode(&params.group_id)
    );
    client.delete_with_body(&path, token, &params.roles).await
}

/// List available realm roles for a group.
///
/// Returns realm-level roles that are available to be assigned to the group
/// (i.e., roles that the group doesn't already have).
///
/// # Endpoint
/// `GET /admin/realms/{realm}/groups/{id}/role-mappings/realm/available`
pub async fn group_realm_roles_available(
    client: &KeycloakClient,
    token: &str,
    params: &GroupRealmRolesAvailableParams,
) -> Result<Vec<RoleRepresentation>, ApiError> {
    params.validate()?;

    let path = format!(
        "/admin/realms/{}/groups/{}/role-mappings/realm/available",
        params.realm,
        urlencoding::encode(&params.group_id)
    );
    client.get(&path, token).await
}

/// List client roles assigned to a group.
///
/// Returns all client-level roles for the specified client that are
/// directly assigned to the group.
///
/// # Endpoint
/// `GET /admin/realms/{realm}/groups/{id}/role-mappings/clients/{client}`
pub async fn group_client_roles_list(
    client: &KeycloakClient,
    token: &str,
    params: &GroupClientRolesListParams,
) -> Result<Vec<RoleRepresentation>, ApiError> {
    params.validate()?;

    let path = format!(
        "/admin/realms/{}/groups/{}/role-mappings/clients/{}",
        params.realm,
        urlencoding::encode(&params.group_id),
        urlencoding::encode(&params.client)
    );
    client.get(&path, token).await
}

/// Add client roles to a group.
///
/// Assigns the specified client-level roles to the group.
///
/// # Endpoint
/// `POST /admin/realms/{realm}/groups/{id}/role-mappings/clients/{client}`
pub async fn group_client_roles_add(
    client: &KeycloakClient,
    token: &str,
    params: &GroupClientRolesAddParams,
) -> Result<(), ApiError> {
    params.validate()?;

    let path = format!(
        "/admin/realms/{}/groups/{}/role-mappings/clients/{}",
        params.realm,
        urlencoding::encode(&params.group_id),
        urlencoding::encode(&params.client)
    );
    client.post_no_response(&path, token, &params.roles).await
}

/// Remove client roles from a group.
///
/// Removes the specified client-level roles from the group.
///
/// # Endpoint
/// `DELETE /admin/realms/{realm}/groups/{id}/role-mappings/clients/{client}`
pub async fn group_client_roles_remove(
    client: &KeycloakClient,
    token: &str,
    params: &GroupClientRolesRemoveParams,
) -> Result<(), ApiError> {
    params.validate()?;

    let path = format!(
        "/admin/realms/{}/groups/{}/role-mappings/clients/{}",
        params.realm,
        urlencoding::encode(&params.group_id),
        urlencoding::encode(&params.client)
    );
    client.delete_with_body(&path, token, &params.roles).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    const TEST_TOKEN: &str = "test-bearer-token";

    mod group_realm_roles_list_params_tests {
        use super::*;

        #[test]
        fn test_validate_success() {
            let params = GroupRealmRolesListParams {
                realm: "master".to_string(),
                group_id: "123".to_string(),
            };
            assert!(params.validate().is_ok());
        }

        #[test]
        fn test_validate_empty_realm() {
            let params = GroupRealmRolesListParams {
                realm: "".to_string(),
                group_id: "123".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_whitespace_realm() {
            let params = GroupRealmRolesListParams {
                realm: "   ".to_string(),
                group_id: "123".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_group_id() {
            let params = GroupRealmRolesListParams {
                realm: "master".to_string(),
                group_id: "".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }
    }

    mod group_realm_roles_add_params_tests {
        use super::*;

        #[test]
        fn test_validate_success() {
            let params = GroupRealmRolesAddParams {
                realm: "master".to_string(),
                group_id: "123".to_string(),
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
            let params = GroupRealmRolesAddParams {
                realm: "".to_string(),
                group_id: "123".to_string(),
                roles: vec![RoleRepresentation::default()],
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_group_id() {
            let params = GroupRealmRolesAddParams {
                realm: "master".to_string(),
                group_id: "".to_string(),
                roles: vec![RoleRepresentation::default()],
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_roles() {
            let params = GroupRealmRolesAddParams {
                realm: "master".to_string(),
                group_id: "123".to_string(),
                roles: vec![],
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }
    }

    mod group_realm_roles_remove_params_tests {
        use super::*;

        #[test]
        fn test_validate_success() {
            let params = GroupRealmRolesRemoveParams {
                realm: "master".to_string(),
                group_id: "123".to_string(),
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
            let params = GroupRealmRolesRemoveParams {
                realm: "".to_string(),
                group_id: "123".to_string(),
                roles: vec![RoleRepresentation::default()],
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_group_id() {
            let params = GroupRealmRolesRemoveParams {
                realm: "master".to_string(),
                group_id: "".to_string(),
                roles: vec![RoleRepresentation::default()],
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_roles() {
            let params = GroupRealmRolesRemoveParams {
                realm: "master".to_string(),
                group_id: "123".to_string(),
                roles: vec![],
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }
    }

    mod group_realm_roles_available_params_tests {
        use super::*;

        #[test]
        fn test_validate_success() {
            let params = GroupRealmRolesAvailableParams {
                realm: "master".to_string(),
                group_id: "123".to_string(),
            };
            assert!(params.validate().is_ok());
        }

        #[test]
        fn test_validate_empty_realm() {
            let params = GroupRealmRolesAvailableParams {
                realm: "".to_string(),
                group_id: "123".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_group_id() {
            let params = GroupRealmRolesAvailableParams {
                realm: "master".to_string(),
                group_id: "".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }
    }

    mod group_client_roles_list_params_tests {
        use super::*;

        #[test]
        fn test_validate_success() {
            let params = GroupClientRolesListParams {
                realm: "master".to_string(),
                group_id: "123".to_string(),
                client: "my-client-id".to_string(),
            };
            assert!(params.validate().is_ok());
        }

        #[test]
        fn test_validate_empty_realm() {
            let params = GroupClientRolesListParams {
                realm: "".to_string(),
                group_id: "123".to_string(),
                client: "my-client-id".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_group_id() {
            let params = GroupClientRolesListParams {
                realm: "master".to_string(),
                group_id: "".to_string(),
                client: "my-client-id".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_client() {
            let params = GroupClientRolesListParams {
                realm: "master".to_string(),
                group_id: "123".to_string(),
                client: "".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }
    }

    mod group_client_roles_add_params_tests {
        use super::*;

        #[test]
        fn test_validate_success() {
            let params = GroupClientRolesAddParams {
                realm: "master".to_string(),
                group_id: "123".to_string(),
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
            let params = GroupClientRolesAddParams {
                realm: "".to_string(),
                group_id: "123".to_string(),
                client: "my-client-id".to_string(),
                roles: vec![RoleRepresentation::default()],
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_group_id() {
            let params = GroupClientRolesAddParams {
                realm: "master".to_string(),
                group_id: "".to_string(),
                client: "my-client-id".to_string(),
                roles: vec![RoleRepresentation::default()],
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_client() {
            let params = GroupClientRolesAddParams {
                realm: "master".to_string(),
                group_id: "123".to_string(),
                client: "".to_string(),
                roles: vec![RoleRepresentation::default()],
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_roles() {
            let params = GroupClientRolesAddParams {
                realm: "master".to_string(),
                group_id: "123".to_string(),
                client: "my-client-id".to_string(),
                roles: vec![],
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }
    }

    mod group_client_roles_remove_params_tests {
        use super::*;

        #[test]
        fn test_validate_success() {
            let params = GroupClientRolesRemoveParams {
                realm: "master".to_string(),
                group_id: "123".to_string(),
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
            let params = GroupClientRolesRemoveParams {
                realm: "".to_string(),
                group_id: "123".to_string(),
                client: "my-client-id".to_string(),
                roles: vec![RoleRepresentation::default()],
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_group_id() {
            let params = GroupClientRolesRemoveParams {
                realm: "master".to_string(),
                group_id: "".to_string(),
                client: "my-client-id".to_string(),
                roles: vec![RoleRepresentation::default()],
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_client() {
            let params = GroupClientRolesRemoveParams {
                realm: "master".to_string(),
                group_id: "123".to_string(),
                client: "".to_string(),
                roles: vec![RoleRepresentation::default()],
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_roles() {
            let params = GroupClientRolesRemoveParams {
                realm: "master".to_string(),
                group_id: "123".to_string(),
                client: "my-client-id".to_string(),
                roles: vec![],
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }
    }

    mod group_realm_roles_list_tests {
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
                .and(path("/admin/realms/master/groups/123/role-mappings/realm"))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .respond_with(ResponseTemplate::new(200).set_body_json(&expected_roles))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupRealmRolesListParams {
                realm: "master".to_string(),
                group_id: "123".to_string(),
            };

            let roles = group_realm_roles_list(&client, TEST_TOKEN, &params)
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
                .and(path("/admin/realms/master/groups/123/role-mappings/realm"))
                .respond_with(
                    ResponseTemplate::new(200).set_body_json::<Vec<RoleRepresentation>>(vec![]),
                )
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupRealmRolesListParams {
                realm: "master".to_string(),
                group_id: "123".to_string(),
            };

            let roles = group_realm_roles_list(&client, TEST_TOKEN, &params)
                .await
                .expect("Failed to list realm roles");
            assert!(roles.is_empty());
        }

        #[tokio::test]
        async fn test_list_group_not_found() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path("/admin/realms/master/groups/999/role-mappings/realm"))
                .respond_with(ResponseTemplate::new(404))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupRealmRolesListParams {
                realm: "master".to_string(),
                group_id: "999".to_string(),
            };

            let result = group_realm_roles_list(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::NotFound)));
        }

        #[tokio::test]
        async fn test_list_validation_fails() {
            let mock_server = MockServer::start().await;
            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupRealmRolesListParams {
                realm: "".to_string(),
                group_id: "123".to_string(),
            };

            let result = group_realm_roles_list(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::BadRequest(_))));
        }
    }

    mod group_realm_roles_add_tests {
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
                .and(path("/admin/realms/master/groups/123/role-mappings/realm"))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .and(header("content-type", "application/json"))
                .respond_with(ResponseTemplate::new(204))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupRealmRolesAddParams {
                realm: "master".to_string(),
                group_id: "123".to_string(),
                roles: roles_to_add,
            };

            let result = group_realm_roles_add(&client, TEST_TOKEN, &params).await;
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
                .and(path("/admin/realms/master/groups/123/role-mappings/realm"))
                .respond_with(ResponseTemplate::new(204))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupRealmRolesAddParams {
                realm: "master".to_string(),
                group_id: "123".to_string(),
                roles: roles_to_add,
            };

            let result = group_realm_roles_add(&client, TEST_TOKEN, &params).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_add_group_not_found() {
            let mock_server = MockServer::start().await;

            Mock::given(method("POST"))
                .and(path("/admin/realms/master/groups/999/role-mappings/realm"))
                .respond_with(ResponseTemplate::new(404))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupRealmRolesAddParams {
                realm: "master".to_string(),
                group_id: "999".to_string(),
                roles: vec![RoleRepresentation {
                    id: Some("role-1".to_string()),
                    ..Default::default()
                }],
            };

            let result = group_realm_roles_add(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::NotFound)));
        }

        #[tokio::test]
        async fn test_add_validation_fails() {
            let mock_server = MockServer::start().await;
            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupRealmRolesAddParams {
                realm: "master".to_string(),
                group_id: "123".to_string(),
                roles: vec![],
            };

            let result = group_realm_roles_add(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::BadRequest(_))));
        }
    }

    mod group_realm_roles_remove_tests {
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
                .and(path("/admin/realms/master/groups/123/role-mappings/realm"))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .and(header("content-type", "application/json"))
                .respond_with(ResponseTemplate::new(204))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupRealmRolesRemoveParams {
                realm: "master".to_string(),
                group_id: "123".to_string(),
                roles: roles_to_remove,
            };

            let result = group_realm_roles_remove(&client, TEST_TOKEN, &params).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_remove_group_not_found() {
            let mock_server = MockServer::start().await;

            Mock::given(method("DELETE"))
                .and(path("/admin/realms/master/groups/999/role-mappings/realm"))
                .respond_with(ResponseTemplate::new(404))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupRealmRolesRemoveParams {
                realm: "master".to_string(),
                group_id: "999".to_string(),
                roles: vec![RoleRepresentation {
                    id: Some("role-1".to_string()),
                    ..Default::default()
                }],
            };

            let result = group_realm_roles_remove(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::NotFound)));
        }

        #[tokio::test]
        async fn test_remove_validation_fails() {
            let mock_server = MockServer::start().await;
            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupRealmRolesRemoveParams {
                realm: "master".to_string(),
                group_id: "123".to_string(),
                roles: vec![],
            };

            let result = group_realm_roles_remove(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::BadRequest(_))));
        }
    }

    mod group_realm_roles_available_tests {
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
                    "/admin/realms/master/groups/123/role-mappings/realm/available",
                ))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .respond_with(ResponseTemplate::new(200).set_body_json(&available_roles))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupRealmRolesAvailableParams {
                realm: "master".to_string(),
                group_id: "123".to_string(),
            };

            let roles = group_realm_roles_available(&client, TEST_TOKEN, &params)
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
                    "/admin/realms/master/groups/123/role-mappings/realm/available",
                ))
                .respond_with(
                    ResponseTemplate::new(200).set_body_json::<Vec<RoleRepresentation>>(vec![]),
                )
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupRealmRolesAvailableParams {
                realm: "master".to_string(),
                group_id: "123".to_string(),
            };

            let roles = group_realm_roles_available(&client, TEST_TOKEN, &params)
                .await
                .expect("Failed to list available roles");
            assert!(roles.is_empty());
        }

        #[tokio::test]
        async fn test_available_group_not_found() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path(
                    "/admin/realms/master/groups/999/role-mappings/realm/available",
                ))
                .respond_with(ResponseTemplate::new(404))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupRealmRolesAvailableParams {
                realm: "master".to_string(),
                group_id: "999".to_string(),
            };

            let result = group_realm_roles_available(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::NotFound)));
        }
    }

    mod group_client_roles_list_tests {
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
                    "/admin/realms/master/groups/123/role-mappings/clients/my-client-id",
                ))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .respond_with(ResponseTemplate::new(200).set_body_json(&expected_roles))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupClientRolesListParams {
                realm: "master".to_string(),
                group_id: "123".to_string(),
                client: "my-client-id".to_string(),
            };

            let roles = group_client_roles_list(&client, TEST_TOKEN, &params)
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
                    "/admin/realms/master/groups/123/role-mappings/clients/my-client-id",
                ))
                .respond_with(
                    ResponseTemplate::new(200).set_body_json::<Vec<RoleRepresentation>>(vec![]),
                )
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupClientRolesListParams {
                realm: "master".to_string(),
                group_id: "123".to_string(),
                client: "my-client-id".to_string(),
            };

            let roles = group_client_roles_list(&client, TEST_TOKEN, &params)
                .await
                .expect("Failed to list client roles");
            assert!(roles.is_empty());
        }

        #[tokio::test]
        async fn test_list_group_not_found() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path(
                    "/admin/realms/master/groups/999/role-mappings/clients/my-client-id",
                ))
                .respond_with(ResponseTemplate::new(404))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupClientRolesListParams {
                realm: "master".to_string(),
                group_id: "999".to_string(),
                client: "my-client-id".to_string(),
            };

            let result = group_client_roles_list(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::NotFound)));
        }

        #[tokio::test]
        async fn test_list_client_not_found() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path(
                    "/admin/realms/master/groups/123/role-mappings/clients/nonexistent",
                ))
                .respond_with(ResponseTemplate::new(404))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupClientRolesListParams {
                realm: "master".to_string(),
                group_id: "123".to_string(),
                client: "nonexistent".to_string(),
            };

            let result = group_client_roles_list(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::NotFound)));
        }
    }

    mod group_client_roles_add_tests {
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
                    "/admin/realms/master/groups/123/role-mappings/clients/my-client-id",
                ))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .and(header("content-type", "application/json"))
                .respond_with(ResponseTemplate::new(204))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupClientRolesAddParams {
                realm: "master".to_string(),
                group_id: "123".to_string(),
                client: "my-client-id".to_string(),
                roles: roles_to_add,
            };

            let result = group_client_roles_add(&client, TEST_TOKEN, &params).await;
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
                    "/admin/realms/master/groups/123/role-mappings/clients/my-client-id",
                ))
                .respond_with(ResponseTemplate::new(204))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupClientRolesAddParams {
                realm: "master".to_string(),
                group_id: "123".to_string(),
                client: "my-client-id".to_string(),
                roles: roles_to_add,
            };

            let result = group_client_roles_add(&client, TEST_TOKEN, &params).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_add_group_not_found() {
            let mock_server = MockServer::start().await;

            Mock::given(method("POST"))
                .and(path(
                    "/admin/realms/master/groups/999/role-mappings/clients/my-client-id",
                ))
                .respond_with(ResponseTemplate::new(404))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupClientRolesAddParams {
                realm: "master".to_string(),
                group_id: "999".to_string(),
                client: "my-client-id".to_string(),
                roles: vec![RoleRepresentation {
                    id: Some("client-role-1".to_string()),
                    ..Default::default()
                }],
            };

            let result = group_client_roles_add(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::NotFound)));
        }

        #[tokio::test]
        async fn test_add_validation_fails() {
            let mock_server = MockServer::start().await;
            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupClientRolesAddParams {
                realm: "master".to_string(),
                group_id: "123".to_string(),
                client: "my-client-id".to_string(),
                roles: vec![],
            };

            let result = group_client_roles_add(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::BadRequest(_))));
        }
    }

    mod group_client_roles_remove_tests {
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
                    "/admin/realms/master/groups/123/role-mappings/clients/my-client-id",
                ))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .and(header("content-type", "application/json"))
                .respond_with(ResponseTemplate::new(204))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupClientRolesRemoveParams {
                realm: "master".to_string(),
                group_id: "123".to_string(),
                client: "my-client-id".to_string(),
                roles: roles_to_remove,
            };

            let result = group_client_roles_remove(&client, TEST_TOKEN, &params).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_remove_group_not_found() {
            let mock_server = MockServer::start().await;

            Mock::given(method("DELETE"))
                .and(path(
                    "/admin/realms/master/groups/999/role-mappings/clients/my-client-id",
                ))
                .respond_with(ResponseTemplate::new(404))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupClientRolesRemoveParams {
                realm: "master".to_string(),
                group_id: "999".to_string(),
                client: "my-client-id".to_string(),
                roles: vec![RoleRepresentation {
                    id: Some("client-role-1".to_string()),
                    ..Default::default()
                }],
            };

            let result = group_client_roles_remove(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::NotFound)));
        }

        #[tokio::test]
        async fn test_remove_validation_fails() {
            let mock_server = MockServer::start().await;
            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupClientRolesRemoveParams {
                realm: "master".to_string(),
                group_id: "123".to_string(),
                client: "my-client-id".to_string(),
                roles: vec![],
            };

            let result = group_client_roles_remove(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::BadRequest(_))));
        }
    }
}
