//! Keycloak Client Scope Protocol Mappers API module.
//!
//! Provides tools for managing protocol mappers on client scopes.
//! Protocol mappers define how claims are mapped into tokens for a client scope.

use serde::Deserialize;

use crate::api::clients::protocol_mappers::ProtocolMapperRepresentation;
use crate::api::{ApiError, KeycloakClient};

/// Parameters for listing protocol mappers of a client scope.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClientScopeProtocolMappersListParams {
    /// The realm name
    pub realm: String,

    /// The client scope UUID
    pub id: String,
}

/// Parameters for getting a single protocol mapper.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClientScopeProtocolMapperGetParams {
    /// The realm name
    pub realm: String,

    /// The client scope UUID
    pub id: String,

    /// The protocol mapper ID
    pub mapper_id: String,
}

/// Parameters for creating a protocol mapper.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClientScopeProtocolMapperCreateParams {
    /// The realm name
    pub realm: String,

    /// The client scope UUID
    pub id: String,

    /// The protocol mapper to create
    pub mapper: ProtocolMapperRepresentation,
}

/// Parameters for updating a protocol mapper.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClientScopeProtocolMapperUpdateParams {
    /// The realm name
    pub realm: String,

    /// The client scope UUID
    pub id: String,

    /// The protocol mapper ID
    pub mapper_id: String,

    /// The updated protocol mapper representation
    pub mapper: ProtocolMapperRepresentation,
}

/// Parameters for deleting a protocol mapper.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClientScopeProtocolMapperDeleteParams {
    /// The realm name
    pub realm: String,

    /// The client scope UUID
    pub id: String,

    /// The protocol mapper ID
    pub mapper_id: String,
}

/// List all protocol mappers for a client scope.
///
/// GET /admin/realms/{realm}/client-scopes/{id}/protocol-mappers/models
pub async fn client_scope_protocol_mappers_list(
    client: &KeycloakClient,
    token: &str,
    params: &ClientScopeProtocolMappersListParams,
) -> Result<Vec<ProtocolMapperRepresentation>, ApiError> {
    let path = format!(
        "/admin/realms/{}/client-scopes/{}/protocol-mappers/models",
        params.realm,
        urlencoding::encode(&params.id)
    );

    client.get(&path, token).await
}

/// Get a single protocol mapper by ID.
///
/// GET /admin/realms/{realm}/client-scopes/{id}/protocol-mappers/models/{mapperId}
pub async fn client_scope_protocol_mapper_get(
    client: &KeycloakClient,
    token: &str,
    params: &ClientScopeProtocolMapperGetParams,
) -> Result<ProtocolMapperRepresentation, ApiError> {
    let path = format!(
        "/admin/realms/{}/client-scopes/{}/protocol-mappers/models/{}",
        params.realm,
        urlencoding::encode(&params.id),
        urlencoding::encode(&params.mapper_id)
    );

    client.get(&path, token).await
}

/// Create a new protocol mapper for a client scope.
///
/// POST /admin/realms/{realm}/client-scopes/{id}/protocol-mappers/models
pub async fn client_scope_protocol_mapper_create(
    client: &KeycloakClient,
    token: &str,
    params: &ClientScopeProtocolMapperCreateParams,
) -> Result<(), ApiError> {
    let path = format!(
        "/admin/realms/{}/client-scopes/{}/protocol-mappers/models",
        params.realm,
        urlencoding::encode(&params.id)
    );

    client.post_no_response(&path, token, &params.mapper).await
}

/// Update an existing protocol mapper.
///
/// PUT /admin/realms/{realm}/client-scopes/{id}/protocol-mappers/models/{mapperId}
pub async fn client_scope_protocol_mapper_update(
    client: &KeycloakClient,
    token: &str,
    params: &ClientScopeProtocolMapperUpdateParams,
) -> Result<(), ApiError> {
    let path = format!(
        "/admin/realms/{}/client-scopes/{}/protocol-mappers/models/{}",
        params.realm,
        urlencoding::encode(&params.id),
        urlencoding::encode(&params.mapper_id)
    );

    client.put(&path, token, &params.mapper).await
}

/// Delete a protocol mapper.
///
/// DELETE /admin/realms/{realm}/client-scopes/{id}/protocol-mappers/models/{mapperId}
pub async fn client_scope_protocol_mapper_delete(
    client: &KeycloakClient,
    token: &str,
    params: &ClientScopeProtocolMapperDeleteParams,
) -> Result<(), ApiError> {
    let path = format!(
        "/admin/realms/{}/client-scopes/{}/protocol-mappers/models/{}",
        params.realm,
        urlencoding::encode(&params.id),
        urlencoding::encode(&params.mapper_id)
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

    fn sample_mapper() -> ProtocolMapperRepresentation {
        let mut config = HashMap::new();
        config.insert("claim.name".to_string(), "email".to_string());
        config.insert("id.token.claim".to_string(), "true".to_string());
        config.insert("access.token.claim".to_string(), "true".to_string());

        ProtocolMapperRepresentation {
            id: Some("mapper-uuid-123".to_string()),
            name: Some("email-mapper".to_string()),
            protocol: Some("openid-connect".to_string()),
            protocol_mapper: Some("oidc-usermodel-attribute-mapper".to_string()),
            consent_required: Some(false),
            config: Some(config),
        }
    }

    #[tokio::test]
    async fn test_client_scope_protocol_mappers_list_success() {
        let mock_server = MockServer::start().await;

        let expected_mappers = vec![sample_mapper()];

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/master/client-scopes/scope-uuid/protocol-mappers/models",
            ))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_mappers))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientScopeProtocolMappersListParams {
            realm: "master".to_string(),
            id: "scope-uuid".to_string(),
        };

        let result = client_scope_protocol_mappers_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());

        let mappers = result.expect("Failed to get mappers");
        assert_eq!(mappers.len(), 1);
        assert_eq!(mappers[0].name, Some("email-mapper".to_string()));
    }

    #[tokio::test]
    async fn test_client_scope_protocol_mappers_list_empty() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/master/client-scopes/scope-uuid/protocol-mappers/models",
            ))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(Vec::<ProtocolMapperRepresentation>::new()),
            )
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientScopeProtocolMappersListParams {
            realm: "master".to_string(),
            id: "scope-uuid".to_string(),
        };

        let result = client_scope_protocol_mappers_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());

        let mappers = result.expect("Failed to get mappers");
        assert!(mappers.is_empty());
    }

    #[tokio::test]
    async fn test_client_scope_protocol_mappers_list_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/master/client-scopes/scope-uuid/protocol-mappers/models",
            ))
            .respond_with(ResponseTemplate::new(401))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientScopeProtocolMappersListParams {
            realm: "master".to_string(),
            id: "scope-uuid".to_string(),
        };

        let result = client_scope_protocol_mappers_list(&client, "invalid-token", &params).await;
        assert!(matches!(result, Err(ApiError::Unauthorized)));
    }

    #[tokio::test]
    async fn test_client_scope_protocol_mappers_list_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/master/client-scopes/nonexistent/protocol-mappers/models",
            ))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientScopeProtocolMappersListParams {
            realm: "master".to_string(),
            id: "nonexistent".to_string(),
        };

        let result = client_scope_protocol_mappers_list(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_client_scope_protocol_mapper_get_success() {
        let mock_server = MockServer::start().await;

        let expected_mapper = sample_mapper();

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/master/client-scopes/scope-uuid/protocol-mappers/models/mapper-uuid-123",
            ))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_mapper))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientScopeProtocolMapperGetParams {
            realm: "master".to_string(),
            id: "scope-uuid".to_string(),
            mapper_id: "mapper-uuid-123".to_string(),
        };

        let result = client_scope_protocol_mapper_get(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());

        let mapper = result.expect("Failed to get mapper");
        assert_eq!(mapper.id, Some("mapper-uuid-123".to_string()));
        assert_eq!(mapper.name, Some("email-mapper".to_string()));
        assert_eq!(mapper.protocol, Some("openid-connect".to_string()));
    }

    #[tokio::test]
    async fn test_client_scope_protocol_mapper_get_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/master/client-scopes/scope-uuid/protocol-mappers/models/nonexistent",
            ))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientScopeProtocolMapperGetParams {
            realm: "master".to_string(),
            id: "scope-uuid".to_string(),
            mapper_id: "nonexistent".to_string(),
        };

        let result = client_scope_protocol_mapper_get(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_client_scope_protocol_mapper_create_success() {
        let mock_server = MockServer::start().await;

        let new_mapper = ProtocolMapperRepresentation {
            name: Some("new-mapper".to_string()),
            protocol: Some("openid-connect".to_string()),
            protocol_mapper: Some("oidc-hardcoded-claim-mapper".to_string()),
            consent_required: Some(false),
            config: Some(HashMap::new()),
            ..Default::default()
        };

        Mock::given(method("POST"))
            .and(path(
                "/admin/realms/master/client-scopes/scope-uuid/protocol-mappers/models",
            ))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .and(body_json(&new_mapper))
            .respond_with(ResponseTemplate::new(201))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientScopeProtocolMapperCreateParams {
            realm: "master".to_string(),
            id: "scope-uuid".to_string(),
            mapper: new_mapper,
        };

        let result = client_scope_protocol_mapper_create(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_client_scope_protocol_mapper_create_conflict() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path(
                "/admin/realms/master/client-scopes/scope-uuid/protocol-mappers/models",
            ))
            .respond_with(ResponseTemplate::new(409).set_body_json(serde_json::json!({
                "errorMessage": "Protocol mapper already exists"
            })))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientScopeProtocolMapperCreateParams {
            realm: "master".to_string(),
            id: "scope-uuid".to_string(),
            mapper: ProtocolMapperRepresentation {
                name: Some("existing-mapper".to_string()),
                ..Default::default()
            },
        };

        let result = client_scope_protocol_mapper_create(&client, TEST_TOKEN, &params).await;
        match result {
            Err(ApiError::Conflict(msg)) => {
                assert!(msg.contains("Protocol mapper already exists"));
            }
            _ => panic!("Expected Conflict error"),
        }
    }

    #[tokio::test]
    async fn test_client_scope_protocol_mapper_update_success() {
        let mock_server = MockServer::start().await;

        let updated_mapper = ProtocolMapperRepresentation {
            id: Some("mapper-uuid-123".to_string()),
            name: Some("updated-mapper".to_string()),
            protocol: Some("openid-connect".to_string()),
            protocol_mapper: Some("oidc-usermodel-attribute-mapper".to_string()),
            consent_required: Some(true),
            config: Some(HashMap::new()),
        };

        Mock::given(method("PUT"))
            .and(path(
                "/admin/realms/master/client-scopes/scope-uuid/protocol-mappers/models/mapper-uuid-123",
            ))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .and(body_json(&updated_mapper))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientScopeProtocolMapperUpdateParams {
            realm: "master".to_string(),
            id: "scope-uuid".to_string(),
            mapper_id: "mapper-uuid-123".to_string(),
            mapper: updated_mapper,
        };

        let result = client_scope_protocol_mapper_update(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_client_scope_protocol_mapper_update_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path(
                "/admin/realms/master/client-scopes/scope-uuid/protocol-mappers/models/nonexistent",
            ))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientScopeProtocolMapperUpdateParams {
            realm: "master".to_string(),
            id: "scope-uuid".to_string(),
            mapper_id: "nonexistent".to_string(),
            mapper: ProtocolMapperRepresentation::default(),
        };

        let result = client_scope_protocol_mapper_update(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_client_scope_protocol_mapper_delete_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path(
                "/admin/realms/master/client-scopes/scope-uuid/protocol-mappers/models/mapper-uuid-123",
            ))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientScopeProtocolMapperDeleteParams {
            realm: "master".to_string(),
            id: "scope-uuid".to_string(),
            mapper_id: "mapper-uuid-123".to_string(),
        };

        let result = client_scope_protocol_mapper_delete(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_client_scope_protocol_mapper_delete_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path(
                "/admin/realms/master/client-scopes/scope-uuid/protocol-mappers/models/nonexistent",
            ))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientScopeProtocolMapperDeleteParams {
            realm: "master".to_string(),
            id: "scope-uuid".to_string(),
            mapper_id: "nonexistent".to_string(),
        };

        let result = client_scope_protocol_mapper_delete(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_protocol_mapper_with_special_characters_in_id() {
        let mock_server = MockServer::start().await;

        let expected_mapper = sample_mapper();

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/my-realm/client-scopes/scope%2Fwith%2Bspecial/protocol-mappers/models/mapper%2Fid",
            ))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_mapper))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientScopeProtocolMapperGetParams {
            realm: "my-realm".to_string(),
            id: "scope/with+special".to_string(),
            mapper_id: "mapper/id".to_string(),
        };

        let result = client_scope_protocol_mapper_get(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }
}
