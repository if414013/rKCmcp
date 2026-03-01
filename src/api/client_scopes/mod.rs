//! Keycloak Client Scopes API module.
//!
//! Provides CRUD operations for managing client scopes in Keycloak realms.

pub mod protocol_mappers;
pub mod scope_mappings;
pub mod types;

pub use protocol_mappers::*;
pub use scope_mappings::*;
pub use types::*;

use crate::api::realms::ClientScopeRepresentation;
use crate::api::{ApiError, KeycloakClient};

/// GET /admin/realms/{realm}/client-scopes
pub async fn client_scope_list(
    client: &KeycloakClient,
    token: &str,
    params: &ClientScopeListParams,
) -> Result<Vec<ClientScopeRepresentation>, ApiError> {
    let path = format!(
        "/admin/realms/{}/client-scopes",
        urlencoding::encode(&params.realm)
    );

    client.get(&path, token).await
}

/// GET /admin/realms/{realm}/client-scopes/{id}
pub async fn client_scope_get(
    client: &KeycloakClient,
    token: &str,
    params: &ClientScopeGetParams,
) -> Result<ClientScopeRepresentation, ApiError> {
    let path = format!(
        "/admin/realms/{}/client-scopes/{}",
        urlencoding::encode(&params.realm),
        urlencoding::encode(&params.id)
    );

    client.get(&path, token).await
}

/// POST /admin/realms/{realm}/client-scopes
pub async fn client_scope_create(
    client: &KeycloakClient,
    token: &str,
    params: &ClientScopeCreateParams,
) -> Result<(), ApiError> {
    let path = format!(
        "/admin/realms/{}/client-scopes",
        urlencoding::encode(&params.realm)
    );

    client
        .post_no_response(&path, token, &params.client_scope)
        .await
}

/// PUT /admin/realms/{realm}/client-scopes/{id}
pub async fn client_scope_update(
    client: &KeycloakClient,
    token: &str,
    params: &ClientScopeUpdateParams,
) -> Result<(), ApiError> {
    let path = format!(
        "/admin/realms/{}/client-scopes/{}",
        urlencoding::encode(&params.realm),
        urlencoding::encode(&params.id)
    );

    client.put(&path, token, &params.client_scope).await
}

/// DELETE /admin/realms/{realm}/client-scopes/{id}
pub async fn client_scope_delete(
    client: &KeycloakClient,
    token: &str,
    params: &ClientScopeDeleteParams,
) -> Result<(), ApiError> {
    let path = format!(
        "/admin/realms/{}/client-scopes/{}",
        urlencoding::encode(&params.realm),
        urlencoding::encode(&params.id)
    );

    client.delete(&path, token).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use wiremock::matchers::{body_json, header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    const TEST_TOKEN: &str = "test-bearer-token";

    fn sample_client_scope() -> ClientScopeRepresentation {
        ClientScopeRepresentation {
            id: Some("scope-abc-123".to_string()),
            name: Some("custom-scope".to_string()),
            description: Some("A custom scope for testing".to_string()),
            protocol: Some("openid-connect".to_string()),
            attributes: Some(HashMap::new()),
            protocol_mappers: None,
        }
    }

    // ==================== client_scope_list tests ====================

    #[tokio::test]
    async fn test_client_scope_list_success() {
        let mock_server = MockServer::start().await;

        let expected_scopes = vec![sample_client_scope()];

        Mock::given(method("GET"))
            .and(path("/admin/realms/master/client-scopes"))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_scopes))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientScopeListParams {
            realm: "master".to_string(),
        };

        let result = client_scope_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());

        let scopes = result.expect("Failed to get client scopes");
        assert_eq!(scopes.len(), 1);
        assert_eq!(scopes[0].name, Some("custom-scope".to_string()));
    }

    #[tokio::test]
    async fn test_client_scope_list_empty() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/admin/realms/test-realm/client-scopes"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(Vec::<ClientScopeRepresentation>::new()),
            )
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientScopeListParams {
            realm: "test-realm".to_string(),
        };

        let result = client_scope_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_client_scope_list_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/admin/realms/master/client-scopes"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientScopeListParams {
            realm: "master".to_string(),
        };

        let result = client_scope_list(&client, "invalid-token", &params).await;
        assert!(matches!(result, Err(ApiError::Unauthorized)));
    }

    #[tokio::test]
    async fn test_client_scope_list_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/admin/realms/nonexistent/client-scopes"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientScopeListParams {
            realm: "nonexistent".to_string(),
        };

        let result = client_scope_list(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_client_scope_list_with_special_characters() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/admin/realms/my%20realm/client-scopes"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(Vec::<ClientScopeRepresentation>::new()),
            )
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientScopeListParams {
            realm: "my realm".to_string(),
        };

        let result = client_scope_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    // ==================== client_scope_get tests ====================

    #[tokio::test]
    async fn test_client_scope_get_success() {
        let mock_server = MockServer::start().await;

        let expected_scope = sample_client_scope();

        Mock::given(method("GET"))
            .and(path("/admin/realms/master/client-scopes/scope-abc-123"))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_scope))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientScopeGetParams {
            realm: "master".to_string(),
            id: "scope-abc-123".to_string(),
        };

        let result = client_scope_get(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());

        let scope = result.expect("Failed to get client scope");
        assert_eq!(scope.id, Some("scope-abc-123".to_string()));
        assert_eq!(scope.name, Some("custom-scope".to_string()));
    }

    #[tokio::test]
    async fn test_client_scope_get_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/admin/realms/master/client-scopes/nonexistent"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientScopeGetParams {
            realm: "master".to_string(),
            id: "nonexistent".to_string(),
        };

        let result = client_scope_get(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_client_scope_get_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/admin/realms/master/client-scopes/scope-123"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientScopeGetParams {
            realm: "master".to_string(),
            id: "scope-123".to_string(),
        };

        let result = client_scope_get(&client, "invalid-token", &params).await;
        assert!(matches!(result, Err(ApiError::Unauthorized)));
    }

    // ==================== client_scope_create tests ====================

    #[tokio::test]
    async fn test_client_scope_create_success() {
        let mock_server = MockServer::start().await;

        let new_scope = ClientScopeRepresentation {
            name: Some("new-scope".to_string()),
            protocol: Some("openid-connect".to_string()),
            description: Some("A new custom scope".to_string()),
            ..Default::default()
        };

        Mock::given(method("POST"))
            .and(path("/admin/realms/master/client-scopes"))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .and(body_json(&new_scope))
            .respond_with(ResponseTemplate::new(201))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientScopeCreateParams {
            realm: "master".to_string(),
            client_scope: new_scope,
        };

        let result = client_scope_create(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_client_scope_create_conflict() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/admin/realms/master/client-scopes"))
            .respond_with(ResponseTemplate::new(409).set_body_json(serde_json::json!({
                "errorMessage": "Client Scope already exists"
            })))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientScopeCreateParams {
            realm: "master".to_string(),
            client_scope: ClientScopeRepresentation {
                name: Some("existing-scope".to_string()),
                ..Default::default()
            },
        };

        let result = client_scope_create(&client, TEST_TOKEN, &params).await;
        match result {
            Err(ApiError::Conflict(msg)) => {
                assert!(msg.contains("Client Scope already exists"));
            }
            _ => panic!("Expected Conflict error"),
        }
    }

    #[tokio::test]
    async fn test_client_scope_create_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/admin/realms/master/client-scopes"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientScopeCreateParams {
            realm: "master".to_string(),
            client_scope: ClientScopeRepresentation {
                name: Some("test-scope".to_string()),
                ..Default::default()
            },
        };

        let result = client_scope_create(&client, "invalid-token", &params).await;
        assert!(matches!(result, Err(ApiError::Unauthorized)));
    }

    #[tokio::test]
    async fn test_client_scope_create_not_found_realm() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/admin/realms/nonexistent/client-scopes"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientScopeCreateParams {
            realm: "nonexistent".to_string(),
            client_scope: ClientScopeRepresentation {
                name: Some("test-scope".to_string()),
                ..Default::default()
            },
        };

        let result = client_scope_create(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    // ==================== client_scope_update tests ====================

    #[tokio::test]
    async fn test_client_scope_update_success() {
        let mock_server = MockServer::start().await;

        let updated_scope = ClientScopeRepresentation {
            id: Some("scope-abc-123".to_string()),
            name: Some("updated-scope".to_string()),
            description: Some("Updated description".to_string()),
            protocol: Some("openid-connect".to_string()),
            ..Default::default()
        };

        Mock::given(method("PUT"))
            .and(path("/admin/realms/master/client-scopes/scope-abc-123"))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .and(body_json(&updated_scope))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientScopeUpdateParams {
            realm: "master".to_string(),
            id: "scope-abc-123".to_string(),
            client_scope: updated_scope,
        };

        let result = client_scope_update(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_client_scope_update_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path("/admin/realms/master/client-scopes/nonexistent"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientScopeUpdateParams {
            realm: "master".to_string(),
            id: "nonexistent".to_string(),
            client_scope: ClientScopeRepresentation::default(),
        };

        let result = client_scope_update(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_client_scope_update_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path("/admin/realms/master/client-scopes/scope-123"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientScopeUpdateParams {
            realm: "master".to_string(),
            id: "scope-123".to_string(),
            client_scope: ClientScopeRepresentation::default(),
        };

        let result = client_scope_update(&client, "invalid-token", &params).await;
        assert!(matches!(result, Err(ApiError::Unauthorized)));
    }

    #[tokio::test]
    async fn test_client_scope_update_conflict() {
        let mock_server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path("/admin/realms/master/client-scopes/scope-123"))
            .respond_with(ResponseTemplate::new(409).set_body_json(serde_json::json!({
                "errorMessage": "Client Scope with same name exists"
            })))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientScopeUpdateParams {
            realm: "master".to_string(),
            id: "scope-123".to_string(),
            client_scope: ClientScopeRepresentation {
                name: Some("duplicate-name".to_string()),
                ..Default::default()
            },
        };

        let result = client_scope_update(&client, TEST_TOKEN, &params).await;
        match result {
            Err(ApiError::Conflict(msg)) => {
                assert!(msg.contains("Client Scope with same name exists"));
            }
            _ => panic!("Expected Conflict error"),
        }
    }

    // ==================== client_scope_delete tests ====================

    #[tokio::test]
    async fn test_client_scope_delete_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/admin/realms/master/client-scopes/scope-abc-123"))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientScopeDeleteParams {
            realm: "master".to_string(),
            id: "scope-abc-123".to_string(),
        };

        let result = client_scope_delete(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_client_scope_delete_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/admin/realms/master/client-scopes/nonexistent"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientScopeDeleteParams {
            realm: "master".to_string(),
            id: "nonexistent".to_string(),
        };

        let result = client_scope_delete(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_client_scope_delete_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/admin/realms/master/client-scopes/scope-123"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientScopeDeleteParams {
            realm: "master".to_string(),
            id: "scope-123".to_string(),
        };

        let result = client_scope_delete(&client, "invalid-token", &params).await;
        assert!(matches!(result, Err(ApiError::Unauthorized)));
    }

    #[tokio::test]
    async fn test_client_scope_delete_conflict_builtin() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/admin/realms/master/client-scopes/builtin-scope-id"))
            .respond_with(ResponseTemplate::new(409).set_body_json(serde_json::json!({
                "errorMessage": "Cannot delete built-in scope"
            })))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientScopeDeleteParams {
            realm: "master".to_string(),
            id: "builtin-scope-id".to_string(),
        };

        let result = client_scope_delete(&client, TEST_TOKEN, &params).await;
        match result {
            Err(ApiError::Conflict(msg)) => {
                assert!(msg.contains("Cannot delete built-in scope"));
            }
            _ => panic!("Expected Conflict error for built-in scope deletion attempt"),
        }
    }

    #[tokio::test]
    async fn test_client_scope_delete_with_special_characters() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/admin/realms/my%20realm/client-scopes/scope%2F123"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientScopeDeleteParams {
            realm: "my realm".to_string(),
            id: "scope/123".to_string(),
        };

        let result = client_scope_delete(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }
}
