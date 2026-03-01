//! Keycloak Realms Admin API module.
//!
//! Provides CRUD operations for managing Keycloak realms.

pub mod defaults;
pub mod events;
pub mod keys;
pub mod sessions;
pub mod types;

pub use defaults::*;
pub use events::*;
pub use keys::*;
pub use sessions::*;
pub use types::*;

use crate::api::{ApiError, KeycloakClient};

/// List all realms.
///
/// GET /admin/realms
pub async fn realm_list(
    client: &KeycloakClient,
    token: &str,
    params: &RealmListParams,
) -> Result<Vec<RealmRepresentation>, ApiError> {
    let mut path = "/admin/realms".to_string();

    if let Some(brief) = params.brief_representation {
        path = format!("{}?briefRepresentation={}", path, brief);
    }

    client.get(&path, token).await
}

/// Get a single realm by name.
///
/// GET /admin/realms/{realm}
pub async fn realm_get(
    client: &KeycloakClient,
    token: &str,
    params: &RealmGetParams,
) -> Result<RealmRepresentation, ApiError> {
    let path = format!("/admin/realms/{}", urlencoding::encode(&params.realm));

    client.get(&path, token).await
}

/// Create a new realm.
///
/// POST /admin/realms
pub async fn realm_create(
    client: &KeycloakClient,
    token: &str,
    params: &RealmCreateParams,
) -> Result<(), ApiError> {
    let path = "/admin/realms";

    client.post_no_response(path, token, &params.realm).await
}

/// Update an existing realm.
///
/// PUT /admin/realms/{realm}
pub async fn realm_update(
    client: &KeycloakClient,
    token: &str,
    params: &RealmUpdateParams,
) -> Result<(), ApiError> {
    let path = format!("/admin/realms/{}", urlencoding::encode(&params.realm_name));

    client.put(&path, token, &params.realm).await
}

/// Delete a realm.
///
/// DELETE /admin/realms/{realm}
pub async fn realm_delete(
    client: &KeycloakClient,
    token: &str,
    params: &RealmDeleteParams,
) -> Result<(), ApiError> {
    params.validate().map_err(ApiError::BadRequest)?;

    let path = format!("/admin/realms/{}", urlencoding::encode(&params.realm));

    client.delete(&path, token).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{body_json, header, method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    const TEST_TOKEN: &str = "test-bearer-token";

    fn sample_realm() -> RealmRepresentation {
        RealmRepresentation {
            id: Some("realm-123".to_string()),
            realm: Some("test-realm".to_string()),
            display_name: Some("Test Realm".to_string()),
            enabled: Some(true),
            ssl_required: Some("external".to_string()),
            ..Default::default()
        }
    }

    // ==================== realm_list tests ====================

    #[tokio::test]
    async fn test_realm_list_success() {
        let mock_server = MockServer::start().await;

        let expected_realms = vec![sample_realm()];

        Mock::given(method("GET"))
            .and(path("/admin/realms"))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_realms))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmListParams {
            brief_representation: None,
        };

        let result = realm_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());

        let realms = result.expect("Failed to get realms");
        assert_eq!(realms.len(), 1);
        assert_eq!(realms[0].realm, Some("test-realm".to_string()));
    }

    #[tokio::test]
    async fn test_realm_list_with_brief_representation() {
        let mock_server = MockServer::start().await;

        let expected_realms = vec![sample_realm()];

        Mock::given(method("GET"))
            .and(path("/admin/realms"))
            .and(query_param("briefRepresentation", "true"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_realms))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmListParams {
            brief_representation: Some(true),
        };

        let result = realm_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_realm_list_empty() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/admin/realms"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(Vec::<RealmRepresentation>::new()),
            )
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmListParams {
            brief_representation: None,
        };

        let result = realm_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_realm_list_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/admin/realms"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmListParams {
            brief_representation: None,
        };

        let result = realm_list(&client, "invalid-token", &params).await;
        assert!(matches!(result, Err(ApiError::Unauthorized)));
    }

    #[tokio::test]
    async fn test_realm_list_forbidden() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/admin/realms"))
            .respond_with(ResponseTemplate::new(403))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmListParams {
            brief_representation: None,
        };

        let result = realm_list(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::Forbidden)));
    }

    // ==================== realm_get tests ====================

    #[tokio::test]
    async fn test_realm_get_success() {
        let mock_server = MockServer::start().await;

        let expected_realm = sample_realm();

        Mock::given(method("GET"))
            .and(path("/admin/realms/test-realm"))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_realm))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmGetParams {
            realm: "test-realm".to_string(),
        };

        let result = realm_get(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());

        let realm = result.expect("Failed to get realm");
        assert_eq!(realm.realm, Some("test-realm".to_string()));
        assert_eq!(realm.display_name, Some("Test Realm".to_string()));
    }

    #[tokio::test]
    async fn test_realm_get_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/admin/realms/nonexistent"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmGetParams {
            realm: "nonexistent".to_string(),
        };

        let result = realm_get(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_realm_get_with_special_characters() {
        let mock_server = MockServer::start().await;

        let expected_realm = RealmRepresentation {
            realm: Some("my realm/test".to_string()),
            ..Default::default()
        };

        Mock::given(method("GET"))
            .and(path("/admin/realms/my%20realm%2Ftest"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_realm))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmGetParams {
            realm: "my realm/test".to_string(),
        };

        let result = realm_get(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_realm_get_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/admin/realms/test-realm"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmGetParams {
            realm: "test-realm".to_string(),
        };

        let result = realm_get(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::Unauthorized)));
    }

    // ==================== realm_create tests ====================

    #[tokio::test]
    async fn test_realm_create_success() {
        let mock_server = MockServer::start().await;

        let new_realm = RealmRepresentation {
            realm: Some("new-realm".to_string()),
            enabled: Some(true),
            display_name: Some("New Realm".to_string()),
            ..Default::default()
        };

        Mock::given(method("POST"))
            .and(path("/admin/realms"))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .and(body_json(&new_realm))
            .respond_with(ResponseTemplate::new(201))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmCreateParams { realm: new_realm };

        let result = realm_create(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_realm_create_conflict() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/admin/realms"))
            .respond_with(ResponseTemplate::new(409).set_body_json(serde_json::json!({
                "errorMessage": "Realm with same name exists"
            })))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmCreateParams {
            realm: RealmRepresentation {
                realm: Some("existing-realm".to_string()),
                ..Default::default()
            },
        };

        let result = realm_create(&client, TEST_TOKEN, &params).await;
        match result {
            Err(ApiError::Conflict(msg)) => {
                assert!(msg.contains("Realm with same name exists"));
            }
            _ => panic!("Expected Conflict error"),
        }
    }

    #[tokio::test]
    async fn test_realm_create_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/admin/realms"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmCreateParams {
            realm: RealmRepresentation {
                realm: Some("test".to_string()),
                ..Default::default()
            },
        };

        let result = realm_create(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::Unauthorized)));
    }

    #[tokio::test]
    async fn test_realm_create_forbidden() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/admin/realms"))
            .respond_with(ResponseTemplate::new(403))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmCreateParams {
            realm: RealmRepresentation {
                realm: Some("test".to_string()),
                ..Default::default()
            },
        };

        let result = realm_create(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::Forbidden)));
    }

    // ==================== realm_update tests ====================

    #[tokio::test]
    async fn test_realm_update_success() {
        let mock_server = MockServer::start().await;

        let updated_realm = RealmRepresentation {
            display_name: Some("Updated Display Name".to_string()),
            enabled: Some(false),
            ..Default::default()
        };

        Mock::given(method("PUT"))
            .and(path("/admin/realms/test-realm"))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .and(body_json(&updated_realm))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmUpdateParams {
            realm_name: "test-realm".to_string(),
            realm: updated_realm,
        };

        let result = realm_update(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_realm_update_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path("/admin/realms/nonexistent"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmUpdateParams {
            realm_name: "nonexistent".to_string(),
            realm: RealmRepresentation::default(),
        };

        let result = realm_update(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_realm_update_partial() {
        let mock_server = MockServer::start().await;

        let partial_update = RealmRepresentation {
            ssl_required: Some("all".to_string()),
            ..Default::default()
        };

        Mock::given(method("PUT"))
            .and(path("/admin/realms/test-realm"))
            .and(body_json(&partial_update))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmUpdateParams {
            realm_name: "test-realm".to_string(),
            realm: partial_update,
        };

        let result = realm_update(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_realm_update_with_special_characters() {
        let mock_server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path("/admin/realms/my%20realm"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmUpdateParams {
            realm_name: "my realm".to_string(),
            realm: RealmRepresentation::default(),
        };

        let result = realm_update(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    // ==================== realm_delete tests ====================

    #[tokio::test]
    async fn test_realm_delete_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/admin/realms/test-realm"))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmDeleteParams {
            realm: "test-realm".to_string(),
        };

        let result = realm_delete(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_realm_delete_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/admin/realms/nonexistent"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmDeleteParams {
            realm: "nonexistent".to_string(),
        };

        let result = realm_delete(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_realm_delete_master_realm_prevented() {
        let mock_server = MockServer::start().await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmDeleteParams {
            realm: "master".to_string(),
        };

        let result = realm_delete(&client, TEST_TOKEN, &params).await;
        match result {
            Err(ApiError::BadRequest(msg)) => {
                assert!(msg.contains("Cannot delete the master realm"));
            }
            _ => panic!("Expected BadRequest error for master realm deletion"),
        }
    }

    #[tokio::test]
    async fn test_realm_delete_master_realm_case_insensitive() {
        let mock_server = MockServer::start().await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");

        let params = RealmDeleteParams {
            realm: "MASTER".to_string(),
        };
        let result = realm_delete(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::BadRequest(_))));

        let params = RealmDeleteParams {
            realm: "Master".to_string(),
        };
        let result = realm_delete(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::BadRequest(_))));
    }

    #[tokio::test]
    async fn test_realm_delete_with_special_characters() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/admin/realms/my%20realm%2Ftest"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmDeleteParams {
            realm: "my realm/test".to_string(),
        };

        let result = realm_delete(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_realm_delete_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/admin/realms/test-realm"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmDeleteParams {
            realm: "test-realm".to_string(),
        };

        let result = realm_delete(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::Unauthorized)));
    }

    #[tokio::test]
    async fn test_realm_delete_forbidden() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/admin/realms/test-realm"))
            .respond_with(ResponseTemplate::new(403))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmDeleteParams {
            realm: "test-realm".to_string(),
        };

        let result = realm_delete(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::Forbidden)));
    }
}
