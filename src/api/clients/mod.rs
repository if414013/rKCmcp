//! Keycloak Clients API module.
//!
//! Provides CRUD operations for managing OAuth 2.0 clients in Keycloak realms.

pub mod types;

pub use types::*;

use crate::api::{ApiError, KeycloakClient};

/// List clients in a realm.
///
/// GET /admin/realms/{realm}/clients
pub async fn client_list(
    client: &KeycloakClient,
    token: &str,
    params: &ClientListParams,
) -> Result<Vec<ClientRepresentation>, ApiError> {
    let mut path = format!("/admin/realms/{}/clients", params.realm);
    let mut query_parts = Vec::new();

    if let Some(ref client_id) = params.client_id {
        query_parts.push(format!("clientId={}", urlencoding::encode(client_id)));
    }

    if let Some(search) = params.search {
        query_parts.push(format!("search={}", search));
    }

    if let Some(viewable_only) = params.viewable_only {
        query_parts.push(format!("viewableOnly={}", viewable_only));
    }

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

/// Get a single client by ID.
///
/// GET /admin/realms/{realm}/clients/{id}
pub async fn client_get(
    client: &KeycloakClient,
    token: &str,
    params: &ClientGetParams,
) -> Result<ClientRepresentation, ApiError> {
    let path = format!(
        "/admin/realms/{}/clients/{}",
        params.realm,
        urlencoding::encode(&params.id)
    );

    client.get(&path, token).await
}

/// Create a new client.
///
/// POST /admin/realms/{realm}/clients
pub async fn client_create(
    client: &KeycloakClient,
    token: &str,
    params: &ClientCreateParams,
) -> Result<(), ApiError> {
    let path = format!("/admin/realms/{}/clients", params.realm);

    client.post_no_response(&path, token, &params.client).await
}

/// Update an existing client.
///
/// PUT /admin/realms/{realm}/clients/{id}
pub async fn client_update(
    client: &KeycloakClient,
    token: &str,
    params: &ClientUpdateParams,
) -> Result<(), ApiError> {
    let path = format!(
        "/admin/realms/{}/clients/{}",
        params.realm,
        urlencoding::encode(&params.id)
    );

    client.put(&path, token, &params.client).await
}

/// Delete a client.
///
/// DELETE /admin/realms/{realm}/clients/{id}
pub async fn client_delete(
    client: &KeycloakClient,
    token: &str,
    params: &ClientDeleteParams,
) -> Result<(), ApiError> {
    let path = format!(
        "/admin/realms/{}/clients/{}",
        params.realm,
        urlencoding::encode(&params.id)
    );

    client.delete(&path, token).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{body_json, header, method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    const TEST_TOKEN: &str = "test-bearer-token";

    fn sample_client() -> ClientRepresentation {
        ClientRepresentation {
            id: Some("abc-123".to_string()),
            client_id: Some("my-test-client".to_string()),
            name: Some("My Test Client".to_string()),
            enabled: Some(true),
            protocol: Some("openid-connect".to_string()),
            public_client: Some(true),
            ..Default::default()
        }
    }

    #[tokio::test]
    async fn test_client_list_success() {
        let mock_server = MockServer::start().await;

        let expected_clients = vec![sample_client()];

        Mock::given(method("GET"))
            .and(path("/admin/realms/master/clients"))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_clients))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientListParams {
            realm: "master".to_string(),
            client_id: None,
            search: None,
            viewable_only: None,
            first: None,
            max: None,
        };

        let result = client_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());

        let clients = result.expect("Failed to get clients");
        assert_eq!(clients.len(), 1);
        assert_eq!(clients[0].client_id, Some("my-test-client".to_string()));
    }

    #[tokio::test]
    async fn test_client_list_with_filters() {
        let mock_server = MockServer::start().await;

        let expected_clients = vec![sample_client()];

        Mock::given(method("GET"))
            .and(path("/admin/realms/test-realm/clients"))
            .and(query_param("clientId", "my-app"))
            .and(query_param("search", "true"))
            .and(query_param("first", "0"))
            .and(query_param("max", "10"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_clients))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientListParams {
            realm: "test-realm".to_string(),
            client_id: Some("my-app".to_string()),
            search: Some(true),
            viewable_only: None,
            first: Some(0),
            max: Some(10),
        };

        let result = client_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_client_list_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/admin/realms/master/clients"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientListParams {
            realm: "master".to_string(),
            client_id: None,
            search: None,
            viewable_only: None,
            first: None,
            max: None,
        };

        let result = client_list(&client, "invalid-token", &params).await;
        assert!(matches!(result, Err(ApiError::Unauthorized)));
    }

    #[tokio::test]
    async fn test_client_get_success() {
        let mock_server = MockServer::start().await;

        let expected_client = sample_client();

        Mock::given(method("GET"))
            .and(path("/admin/realms/master/clients/abc-123"))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_client))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientGetParams {
            realm: "master".to_string(),
            id: "abc-123".to_string(),
        };

        let result = client_get(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());

        let returned_client = result.expect("Failed to get client");
        assert_eq!(returned_client.id, Some("abc-123".to_string()));
        assert_eq!(returned_client.client_id, Some("my-test-client".to_string()));
    }

    #[tokio::test]
    async fn test_client_get_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/admin/realms/master/clients/nonexistent"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientGetParams {
            realm: "master".to_string(),
            id: "nonexistent".to_string(),
        };

        let result = client_get(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_client_create_success() {
        let mock_server = MockServer::start().await;

        let new_client = ClientRepresentation {
            client_id: Some("new-client".to_string()),
            enabled: Some(true),
            public_client: Some(true),
            ..Default::default()
        };

        Mock::given(method("POST"))
            .and(path("/admin/realms/master/clients"))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .and(body_json(&new_client))
            .respond_with(ResponseTemplate::new(201))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientCreateParams {
            realm: "master".to_string(),
            client: new_client,
        };

        let result = client_create(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_client_create_conflict() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/admin/realms/master/clients"))
            .respond_with(ResponseTemplate::new(409).set_body_json(serde_json::json!({
                "errorMessage": "Client already exists"
            })))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientCreateParams {
            realm: "master".to_string(),
            client: ClientRepresentation {
                client_id: Some("existing-client".to_string()),
                ..Default::default()
            },
        };

        let result = client_create(&client, TEST_TOKEN, &params).await;
        match result {
            Err(ApiError::Conflict(msg)) => {
                assert!(msg.contains("Client already exists"));
            }
            _ => panic!("Expected Conflict error"),
        }
    }

    #[tokio::test]
    async fn test_client_update_success() {
        let mock_server = MockServer::start().await;

        let updated_client = ClientRepresentation {
            id: Some("abc-123".to_string()),
            client_id: Some("updated-client".to_string()),
            name: Some("Updated Client Name".to_string()),
            enabled: Some(true),
            ..Default::default()
        };

        Mock::given(method("PUT"))
            .and(path("/admin/realms/master/clients/abc-123"))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .and(body_json(&updated_client))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientUpdateParams {
            realm: "master".to_string(),
            id: "abc-123".to_string(),
            client: updated_client,
        };

        let result = client_update(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_client_update_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path("/admin/realms/master/clients/nonexistent"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientUpdateParams {
            realm: "master".to_string(),
            id: "nonexistent".to_string(),
            client: ClientRepresentation::default(),
        };

        let result = client_update(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_client_delete_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/admin/realms/master/clients/abc-123"))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientDeleteParams {
            realm: "master".to_string(),
            id: "abc-123".to_string(),
        };

        let result = client_delete(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_client_delete_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/admin/realms/master/clients/nonexistent"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientDeleteParams {
            realm: "master".to_string(),
            id: "nonexistent".to_string(),
        };

        let result = client_delete(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_client_list_with_special_characters() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/admin/realms/my-realm/clients"))
            .and(query_param("clientId", "client/with+special"))
            .respond_with(ResponseTemplate::new(200).set_body_json(Vec::<ClientRepresentation>::new()))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientListParams {
            realm: "my-realm".to_string(),
            client_id: Some("client/with+special".to_string()),
            search: None,
            viewable_only: None,
            first: None,
            max: None,
        };

        let result = client_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }
}
