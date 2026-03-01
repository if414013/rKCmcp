//! Client scope mappings API functions.
//!
//! This module provides functionality for managing scope mappings for client scopes,
//! allowing client scopes to have realm roles mapped to their scope.

use crate::api::roles::RoleRepresentation;
use crate::api::{ApiError, KeycloakClient};
use serde::Deserialize;

/// Parameters for listing realm-level scope mappings for a client scope.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClientScopeScopeMappingsRealmListParams {
    /// The realm name (not id!)
    pub realm: String,

    /// The client scope UUID
    pub id: String,
}

/// Parameters for adding realm-level scope mappings to a client scope.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClientScopeScopeMappingsRealmAddParams {
    /// The realm name (not id!)
    pub realm: String,

    /// The client scope UUID
    pub id: String,

    /// Array of roles to add to the scope
    pub roles: Vec<RoleRepresentation>,
}

/// List realm-level roles that are mapped to a client scope's scope.
///
/// GET /admin/realms/{realm}/client-scopes/{id}/scope-mappings/realm
pub async fn client_scope_scope_mappings_realm_list(
    client: &KeycloakClient,
    token: &str,
    params: &ClientScopeScopeMappingsRealmListParams,
) -> Result<Vec<RoleRepresentation>, ApiError> {
    let path = format!(
        "/admin/realms/{}/client-scopes/{}/scope-mappings/realm",
        params.realm,
        urlencoding::encode(&params.id)
    );

    client.get(&path, token).await
}

/// Add realm-level roles to a client scope's scope.
///
/// POST /admin/realms/{realm}/client-scopes/{id}/scope-mappings/realm
pub async fn client_scope_scope_mappings_realm_add(
    client: &KeycloakClient,
    token: &str,
    params: &ClientScopeScopeMappingsRealmAddParams,
) -> Result<(), ApiError> {
    let path = format!(
        "/admin/realms/{}/client-scopes/{}/scope-mappings/realm",
        params.realm,
        urlencoding::encode(&params.id)
    );

    client.post_no_response(&path, token, &params.roles).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{body_json, header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    const TEST_TOKEN: &str = "test-bearer-token";
    const TEST_SCOPE_ID: &str = "scope-uuid-123";

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

    #[tokio::test]
    async fn test_client_scope_scope_mappings_realm_list_success() {
        let mock_server = MockServer::start().await;

        let expected_roles = vec![sample_role()];

        Mock::given(method("GET"))
            .and(path(format!(
                "/admin/realms/master/client-scopes/{}/scope-mappings/realm",
                TEST_SCOPE_ID
            )))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_roles))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientScopeScopeMappingsRealmListParams {
            realm: "master".to_string(),
            id: TEST_SCOPE_ID.to_string(),
        };

        let result = client_scope_scope_mappings_realm_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());

        let roles = result.expect("Failed to get realm scope mappings");
        assert_eq!(roles.len(), 1);
        assert_eq!(roles[0].name, Some("test-role".to_string()));
    }

    #[tokio::test]
    async fn test_client_scope_scope_mappings_realm_list_empty() {
        let mock_server = MockServer::start().await;

        let empty_roles: Vec<RoleRepresentation> = vec![];

        Mock::given(method("GET"))
            .and(path(format!(
                "/admin/realms/master/client-scopes/{}/scope-mappings/realm",
                TEST_SCOPE_ID
            )))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&empty_roles))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientScopeScopeMappingsRealmListParams {
            realm: "master".to_string(),
            id: TEST_SCOPE_ID.to_string(),
        };

        let result = client_scope_scope_mappings_realm_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());

        let roles = result.expect("Failed to get realm scope mappings");
        assert_eq!(roles.len(), 0);
    }

    #[tokio::test]
    async fn test_client_scope_scope_mappings_realm_list_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/master/client-scopes/nonexistent/scope-mappings/realm",
            ))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientScopeScopeMappingsRealmListParams {
            realm: "master".to_string(),
            id: "nonexistent".to_string(),
        };

        let result = client_scope_scope_mappings_realm_list(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_client_scope_scope_mappings_realm_list_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(format!(
                "/admin/realms/master/client-scopes/{}/scope-mappings/realm",
                TEST_SCOPE_ID
            )))
            .respond_with(ResponseTemplate::new(401))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientScopeScopeMappingsRealmListParams {
            realm: "master".to_string(),
            id: TEST_SCOPE_ID.to_string(),
        };

        let result =
            client_scope_scope_mappings_realm_list(&client, "invalid-token", &params).await;
        assert!(matches!(result, Err(ApiError::Unauthorized)));
    }

    #[tokio::test]
    async fn test_client_scope_scope_mappings_realm_add_success() {
        let mock_server = MockServer::start().await;

        let roles = vec![sample_role()];

        Mock::given(method("POST"))
            .and(path(format!(
                "/admin/realms/master/client-scopes/{}/scope-mappings/realm",
                TEST_SCOPE_ID
            )))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .and(body_json(&roles))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientScopeScopeMappingsRealmAddParams {
            realm: "master".to_string(),
            id: TEST_SCOPE_ID.to_string(),
            roles,
        };

        let result = client_scope_scope_mappings_realm_add(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_client_scope_scope_mappings_realm_add_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path(
                "/admin/realms/master/client-scopes/nonexistent/scope-mappings/realm",
            ))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientScopeScopeMappingsRealmAddParams {
            realm: "master".to_string(),
            id: "nonexistent".to_string(),
            roles: vec![sample_role()],
        };

        let result = client_scope_scope_mappings_realm_add(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_client_scope_scope_mappings_realm_add_multiple_roles() {
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
                "/admin/realms/master/client-scopes/{}/scope-mappings/realm",
                TEST_SCOPE_ID
            )))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .and(body_json(&roles))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientScopeScopeMappingsRealmAddParams {
            realm: "master".to_string(),
            id: TEST_SCOPE_ID.to_string(),
            roles,
        };

        let result = client_scope_scope_mappings_realm_add(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_client_scope_scope_mappings_with_url_encoding() {
        let mock_server = MockServer::start().await;

        let scope_id_with_special = "scope/with+special";

        let empty_roles: Vec<RoleRepresentation> = vec![];

        Mock::given(method("GET"))
            .and(path(format!(
                "/admin/realms/my-realm/client-scopes/{}/scope-mappings/realm",
                urlencoding::encode(scope_id_with_special)
            )))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&empty_roles))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientScopeScopeMappingsRealmListParams {
            realm: "my-realm".to_string(),
            id: scope_id_with_special.to_string(),
        };

        let result = client_scope_scope_mappings_realm_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }
}
