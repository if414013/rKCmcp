//! Client scope mappings API functions.
//!
//! This module provides functionality for managing scope mappings for clients,
//! allowing clients to have realm roles and client roles mapped to their scope.

use crate::api::roles::RoleRepresentation;
use crate::api::{ApiError, KeycloakClient};
use serde::Deserialize;

/// Parameters for listing realm-level scope mappings for a client.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClientScopeMappingsRealmListParams {
    /// The realm name (not id!)
    pub realm: String,

    /// The client UUID (not clientId!)
    pub id: String,
}

/// Parameters for adding realm-level scope mappings to a client.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClientScopeMappingsRealmAddParams {
    /// The realm name (not id!)
    pub realm: String,

    /// The client UUID (not clientId!)
    pub id: String,

    /// Array of roles to add to the scope
    pub roles: Vec<RoleRepresentation>,
}

/// Parameters for removing realm-level scope mappings from a client.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClientScopeMappingsRealmRemoveParams {
    /// The realm name (not id!)
    pub realm: String,

    /// The client UUID (not clientId!)
    pub id: String,

    /// Array of roles to remove from the scope
    pub roles: Vec<RoleRepresentation>,
}

/// Parameters for listing client-level scope mappings for a client.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClientScopeMappingsClientListParams {
    /// The realm name (not id!)
    pub realm: String,

    /// The client UUID (not clientId!)
    pub id: String,

    /// The UUID of the client whose roles are mapped
    pub client: String,
}

/// Parameters for adding client-level scope mappings to a client.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClientScopeMappingsClientAddParams {
    /// The realm name (not id!)
    pub realm: String,

    /// The client UUID (not clientId!)
    pub id: String,

    /// The UUID of the client whose roles are being mapped
    pub client: String,

    /// Array of roles to add to the scope
    pub roles: Vec<RoleRepresentation>,
}

/// Parameters for removing client-level scope mappings from a client.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClientScopeMappingsClientRemoveParams {
    /// The realm name (not id!)
    pub realm: String,

    /// The client UUID (not clientId!)
    pub id: String,

    /// The UUID of the client whose roles are being unmapped
    pub client: String,

    /// Array of roles to remove from the scope
    pub roles: Vec<RoleRepresentation>,
}

/// List realm-level roles that are mapped to a client's scope.
///
/// GET /admin/realms/{realm}/clients/{id}/scope-mappings/realm
pub async fn client_scope_mappings_realm_list(
    client: &KeycloakClient,
    token: &str,
    params: &ClientScopeMappingsRealmListParams,
) -> Result<Vec<RoleRepresentation>, ApiError> {
    let path = format!(
        "/admin/realms/{}/clients/{}/scope-mappings/realm",
        params.realm,
        urlencoding::encode(&params.id)
    );

    client.get(&path, token).await
}

/// Add realm-level roles to a client's scope.
///
/// POST /admin/realms/{realm}/clients/{id}/scope-mappings/realm
pub async fn client_scope_mappings_realm_add(
    client: &KeycloakClient,
    token: &str,
    params: &ClientScopeMappingsRealmAddParams,
) -> Result<(), ApiError> {
    let path = format!(
        "/admin/realms/{}/clients/{}/scope-mappings/realm",
        params.realm,
        urlencoding::encode(&params.id)
    );

    client.post_no_response(&path, token, &params.roles).await
}

/// Remove realm-level roles from a client's scope.
///
/// DELETE /admin/realms/{realm}/clients/{id}/scope-mappings/realm
pub async fn client_scope_mappings_realm_remove(
    client: &KeycloakClient,
    token: &str,
    params: &ClientScopeMappingsRealmRemoveParams,
) -> Result<(), ApiError> {
    let path = format!(
        "/admin/realms/{}/clients/{}/scope-mappings/realm",
        params.realm,
        urlencoding::encode(&params.id)
    );

    client.delete_with_body(&path, token, &params.roles).await
}

/// List client-level roles that are mapped to a client's scope.
///
/// GET /admin/realms/{realm}/clients/{id}/scope-mappings/clients/{client}
pub async fn client_scope_mappings_client_list(
    client: &KeycloakClient,
    token: &str,
    params: &ClientScopeMappingsClientListParams,
) -> Result<Vec<RoleRepresentation>, ApiError> {
    let path = format!(
        "/admin/realms/{}/clients/{}/scope-mappings/clients/{}",
        params.realm,
        urlencoding::encode(&params.id),
        urlencoding::encode(&params.client)
    );

    client.get(&path, token).await
}

/// Add client-level roles to a client's scope.
///
/// POST /admin/realms/{realm}/clients/{id}/scope-mappings/clients/{client}
pub async fn client_scope_mappings_client_add(
    client: &KeycloakClient,
    token: &str,
    params: &ClientScopeMappingsClientAddParams,
) -> Result<(), ApiError> {
    let path = format!(
        "/admin/realms/{}/clients/{}/scope-mappings/clients/{}",
        params.realm,
        urlencoding::encode(&params.id),
        urlencoding::encode(&params.client)
    );

    client.post_no_response(&path, token, &params.roles).await
}

/// Remove client-level roles from a client's scope.
///
/// DELETE /admin/realms/{realm}/clients/{id}/scope-mappings/clients/{client}
pub async fn client_scope_mappings_client_remove(
    client: &KeycloakClient,
    token: &str,
    params: &ClientScopeMappingsClientRemoveParams,
) -> Result<(), ApiError> {
    let path = format!(
        "/admin/realms/{}/clients/{}/scope-mappings/clients/{}",
        params.realm,
        urlencoding::encode(&params.id),
        urlencoding::encode(&params.client)
    );

    client.delete_with_body(&path, token, &params.roles).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{body_json, header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    const TEST_TOKEN: &str = "test-bearer-token";
    const TEST_CLIENT_ID: &str = "client-uuid-123";
    const TEST_OTHER_CLIENT_ID: &str = "other-client-uuid-456";

    fn sample_role() -> RoleRepresentation {
        RoleRepresentation {
            id: Some("role-id-123".to_string()),
            name: Some("test-role".to_string()),
            description: Some("A test role".to_string()),
            composite: Some(false),
            client_role: Some(false),
            container_id: Some("realm-id".to_string()),
            attributes: None,
        }
    }

    fn sample_client_role() -> RoleRepresentation {
        RoleRepresentation {
            id: Some("client-role-id-456".to_string()),
            name: Some("test-client-role".to_string()),
            description: Some("A test client role".to_string()),
            composite: Some(false),
            client_role: Some(true),
            container_id: Some(TEST_OTHER_CLIENT_ID.to_string()),
            attributes: None,
        }
    }

    #[tokio::test]
    async fn test_client_scope_mappings_realm_list_success() {
        let mock_server = MockServer::start().await;

        let expected_roles = vec![sample_role()];

        Mock::given(method("GET"))
            .and(path(format!(
                "/admin/realms/master/clients/{}/scope-mappings/realm",
                TEST_CLIENT_ID
            )))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_roles))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientScopeMappingsRealmListParams {
            realm: "master".to_string(),
            id: TEST_CLIENT_ID.to_string(),
        };

        let result = client_scope_mappings_realm_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());

        let roles = result.expect("Failed to get realm scope mappings");
        assert_eq!(roles.len(), 1);
        assert_eq!(roles[0].name, Some("test-role".to_string()));
    }

    #[tokio::test]
    async fn test_client_scope_mappings_realm_list_empty() {
        let mock_server = MockServer::start().await;

        let empty_roles: Vec<RoleRepresentation> = vec![];

        Mock::given(method("GET"))
            .and(path(format!(
                "/admin/realms/master/clients/{}/scope-mappings/realm",
                TEST_CLIENT_ID
            )))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&empty_roles))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientScopeMappingsRealmListParams {
            realm: "master".to_string(),
            id: TEST_CLIENT_ID.to_string(),
        };

        let result = client_scope_mappings_realm_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());

        let roles = result.expect("Failed to get realm scope mappings");
        assert_eq!(roles.len(), 0);
    }

    #[tokio::test]
    async fn test_client_scope_mappings_realm_list_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/master/clients/nonexistent/scope-mappings/realm",
            ))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientScopeMappingsRealmListParams {
            realm: "master".to_string(),
            id: "nonexistent".to_string(),
        };

        let result = client_scope_mappings_realm_list(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_client_scope_mappings_realm_add_success() {
        let mock_server = MockServer::start().await;

        let roles = vec![sample_role()];

        Mock::given(method("POST"))
            .and(path(format!(
                "/admin/realms/master/clients/{}/scope-mappings/realm",
                TEST_CLIENT_ID
            )))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .and(body_json(&roles))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientScopeMappingsRealmAddParams {
            realm: "master".to_string(),
            id: TEST_CLIENT_ID.to_string(),
            roles,
        };

        let result = client_scope_mappings_realm_add(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_client_scope_mappings_realm_add_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path(
                "/admin/realms/master/clients/nonexistent/scope-mappings/realm",
            ))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientScopeMappingsRealmAddParams {
            realm: "master".to_string(),
            id: "nonexistent".to_string(),
            roles: vec![sample_role()],
        };

        let result = client_scope_mappings_realm_add(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_client_scope_mappings_realm_remove_success() {
        let mock_server = MockServer::start().await;

        let roles = vec![sample_role()];

        Mock::given(method("DELETE"))
            .and(path(format!(
                "/admin/realms/master/clients/{}/scope-mappings/realm",
                TEST_CLIENT_ID
            )))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .and(body_json(&roles))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientScopeMappingsRealmRemoveParams {
            realm: "master".to_string(),
            id: TEST_CLIENT_ID.to_string(),
            roles,
        };

        let result = client_scope_mappings_realm_remove(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_client_scope_mappings_realm_remove_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path(
                "/admin/realms/master/clients/nonexistent/scope-mappings/realm",
            ))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientScopeMappingsRealmRemoveParams {
            realm: "master".to_string(),
            id: "nonexistent".to_string(),
            roles: vec![sample_role()],
        };

        let result = client_scope_mappings_realm_remove(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_client_scope_mappings_client_list_success() {
        let mock_server = MockServer::start().await;

        let expected_roles = vec![sample_client_role()];

        Mock::given(method("GET"))
            .and(path(format!(
                "/admin/realms/master/clients/{}/scope-mappings/clients/{}",
                TEST_CLIENT_ID, TEST_OTHER_CLIENT_ID
            )))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_roles))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientScopeMappingsClientListParams {
            realm: "master".to_string(),
            id: TEST_CLIENT_ID.to_string(),
            client: TEST_OTHER_CLIENT_ID.to_string(),
        };

        let result = client_scope_mappings_client_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());

        let roles = result.expect("Failed to get client scope mappings");
        assert_eq!(roles.len(), 1);
        assert_eq!(roles[0].name, Some("test-client-role".to_string()));
        assert_eq!(roles[0].client_role, Some(true));
    }

    #[tokio::test]
    async fn test_client_scope_mappings_client_list_empty() {
        let mock_server = MockServer::start().await;

        let empty_roles: Vec<RoleRepresentation> = vec![];

        Mock::given(method("GET"))
            .and(path(format!(
                "/admin/realms/master/clients/{}/scope-mappings/clients/{}",
                TEST_CLIENT_ID, TEST_OTHER_CLIENT_ID
            )))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&empty_roles))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientScopeMappingsClientListParams {
            realm: "master".to_string(),
            id: TEST_CLIENT_ID.to_string(),
            client: TEST_OTHER_CLIENT_ID.to_string(),
        };

        let result = client_scope_mappings_client_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());

        let roles = result.expect("Failed to get client scope mappings");
        assert_eq!(roles.len(), 0);
    }

    #[tokio::test]
    async fn test_client_scope_mappings_client_list_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(format!(
                "/admin/realms/master/clients/nonexistent/scope-mappings/clients/{}",
                TEST_OTHER_CLIENT_ID
            )))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientScopeMappingsClientListParams {
            realm: "master".to_string(),
            id: "nonexistent".to_string(),
            client: TEST_OTHER_CLIENT_ID.to_string(),
        };

        let result = client_scope_mappings_client_list(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_client_scope_mappings_client_add_success() {
        let mock_server = MockServer::start().await;

        let roles = vec![sample_client_role()];

        Mock::given(method("POST"))
            .and(path(format!(
                "/admin/realms/master/clients/{}/scope-mappings/clients/{}",
                TEST_CLIENT_ID, TEST_OTHER_CLIENT_ID
            )))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .and(body_json(&roles))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientScopeMappingsClientAddParams {
            realm: "master".to_string(),
            id: TEST_CLIENT_ID.to_string(),
            client: TEST_OTHER_CLIENT_ID.to_string(),
            roles,
        };

        let result = client_scope_mappings_client_add(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_client_scope_mappings_client_add_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path(format!(
                "/admin/realms/master/clients/nonexistent/scope-mappings/clients/{}",
                TEST_OTHER_CLIENT_ID
            )))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientScopeMappingsClientAddParams {
            realm: "master".to_string(),
            id: "nonexistent".to_string(),
            client: TEST_OTHER_CLIENT_ID.to_string(),
            roles: vec![sample_client_role()],
        };

        let result = client_scope_mappings_client_add(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_client_scope_mappings_client_remove_success() {
        let mock_server = MockServer::start().await;

        let roles = vec![sample_client_role()];

        Mock::given(method("DELETE"))
            .and(path(format!(
                "/admin/realms/master/clients/{}/scope-mappings/clients/{}",
                TEST_CLIENT_ID, TEST_OTHER_CLIENT_ID
            )))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .and(body_json(&roles))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientScopeMappingsClientRemoveParams {
            realm: "master".to_string(),
            id: TEST_CLIENT_ID.to_string(),
            client: TEST_OTHER_CLIENT_ID.to_string(),
            roles,
        };

        let result = client_scope_mappings_client_remove(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_client_scope_mappings_client_remove_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path(format!(
                "/admin/realms/master/clients/nonexistent/scope-mappings/clients/{}",
                TEST_OTHER_CLIENT_ID
            )))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientScopeMappingsClientRemoveParams {
            realm: "master".to_string(),
            id: "nonexistent".to_string(),
            client: TEST_OTHER_CLIENT_ID.to_string(),
            roles: vec![sample_client_role()],
        };

        let result = client_scope_mappings_client_remove(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_client_scope_mappings_realm_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(format!(
                "/admin/realms/master/clients/{}/scope-mappings/realm",
                TEST_CLIENT_ID
            )))
            .respond_with(ResponseTemplate::new(401))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientScopeMappingsRealmListParams {
            realm: "master".to_string(),
            id: TEST_CLIENT_ID.to_string(),
        };

        let result = client_scope_mappings_realm_list(&client, "invalid-token", &params).await;
        assert!(matches!(result, Err(ApiError::Unauthorized)));
    }

    #[tokio::test]
    async fn test_client_scope_mappings_client_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(format!(
                "/admin/realms/master/clients/{}/scope-mappings/clients/{}",
                TEST_CLIENT_ID, TEST_OTHER_CLIENT_ID
            )))
            .respond_with(ResponseTemplate::new(401))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientScopeMappingsClientListParams {
            realm: "master".to_string(),
            id: TEST_CLIENT_ID.to_string(),
            client: TEST_OTHER_CLIENT_ID.to_string(),
        };

        let result = client_scope_mappings_client_list(&client, "invalid-token", &params).await;
        assert!(matches!(result, Err(ApiError::Unauthorized)));
    }

    #[tokio::test]
    async fn test_client_scope_mappings_realm_add_multiple_roles() {
        let mock_server = MockServer::start().await;

        let roles = vec![
            RoleRepresentation {
                id: Some("role1".to_string()),
                name: Some("admin".to_string()),
                ..Default::default()
            },
            RoleRepresentation {
                id: Some("role2".to_string()),
                name: Some("user".to_string()),
                ..Default::default()
            },
        ];

        Mock::given(method("POST"))
            .and(path(format!(
                "/admin/realms/master/clients/{}/scope-mappings/realm",
                TEST_CLIENT_ID
            )))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .and(body_json(&roles))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientScopeMappingsRealmAddParams {
            realm: "master".to_string(),
            id: TEST_CLIENT_ID.to_string(),
            roles,
        };

        let result = client_scope_mappings_realm_add(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_client_scope_mappings_with_url_encoding() {
        let mock_server = MockServer::start().await;

        let client_id_with_special = "client/with+special";
        let other_client_id_with_special = "other/client+id";

        let empty_roles: Vec<RoleRepresentation> = vec![];

        Mock::given(method("GET"))
            .and(path(format!(
                "/admin/realms/my-realm/clients/{}/scope-mappings/clients/{}",
                urlencoding::encode(client_id_with_special),
                urlencoding::encode(other_client_id_with_special)
            )))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&empty_roles))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientScopeMappingsClientListParams {
            realm: "my-realm".to_string(),
            id: client_id_with_special.to_string(),
            client: other_client_id_with_special.to_string(),
        };

        let result = client_scope_mappings_client_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }
}
