//! Keycloak Client Protocol Mappers API module.
//!
//! Provides tools for managing protocol mappers on clients.
//! Protocol mappers define how claims are mapped into tokens for a client.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::api::{ApiError, KeycloakClient};

/// Representation of a protocol mapper in Keycloak.
///
/// Protocol mappers are used to configure how claims and tokens are created
/// for a particular client. They can add, modify, or remove claims.
#[derive(Debug, Clone, Serialize, Deserialize, Default, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ProtocolMapperRepresentation {
    /// Unique identifier for the protocol mapper (UUID)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Name of the protocol mapper
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Protocol this mapper applies to (e.g., "openid-connect", "saml")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,

    /// Type of protocol mapper (e.g., "oidc-usermodel-attribute-mapper")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_mapper: Option<String>,

    /// Whether consent is required for this mapper
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent_required: Option<bool>,

    /// Configuration properties for the mapper
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<HashMap<String, String>>,
}

/// Parameters for listing protocol mappers of a client.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClientProtocolMappersListParams {
    /// The realm name
    pub realm: String,

    /// The client UUID (not client_id)
    pub id: String,
}

/// Parameters for getting a single protocol mapper.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClientProtocolMapperGetParams {
    /// The realm name
    pub realm: String,

    /// The client UUID (not client_id)
    pub id: String,

    /// The protocol mapper ID
    pub mapper_id: String,
}

/// Parameters for creating a protocol mapper.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClientProtocolMapperCreateParams {
    /// The realm name
    pub realm: String,

    /// The client UUID (not client_id)
    pub id: String,

    /// The protocol mapper to create
    pub mapper: ProtocolMapperRepresentation,
}

/// Parameters for updating a protocol mapper.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClientProtocolMapperUpdateParams {
    /// The realm name
    pub realm: String,

    /// The client UUID (not client_id)
    pub id: String,

    /// The protocol mapper ID
    pub mapper_id: String,

    /// The updated protocol mapper representation
    pub mapper: ProtocolMapperRepresentation,
}

/// Parameters for deleting a protocol mapper.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClientProtocolMapperDeleteParams {
    /// The realm name
    pub realm: String,

    /// The client UUID (not client_id)
    pub id: String,

    /// The protocol mapper ID
    pub mapper_id: String,
}

/// List all protocol mappers for a client.
///
/// GET /admin/realms/{realm}/clients/{id}/protocol-mappers/models
pub async fn client_protocol_mappers_list(
    client: &KeycloakClient,
    token: &str,
    params: &ClientProtocolMappersListParams,
) -> Result<Vec<ProtocolMapperRepresentation>, ApiError> {
    let path = format!(
        "/admin/realms/{}/clients/{}/protocol-mappers/models",
        params.realm,
        urlencoding::encode(&params.id)
    );

    client.get(&path, token).await
}

/// Get a single protocol mapper by ID.
///
/// GET /admin/realms/{realm}/clients/{id}/protocol-mappers/models/{mapperId}
pub async fn client_protocol_mapper_get(
    client: &KeycloakClient,
    token: &str,
    params: &ClientProtocolMapperGetParams,
) -> Result<ProtocolMapperRepresentation, ApiError> {
    let path = format!(
        "/admin/realms/{}/clients/{}/protocol-mappers/models/{}",
        params.realm,
        urlencoding::encode(&params.id),
        urlencoding::encode(&params.mapper_id)
    );

    client.get(&path, token).await
}

/// Create a new protocol mapper for a client.
///
/// POST /admin/realms/{realm}/clients/{id}/protocol-mappers/models
pub async fn client_protocol_mapper_create(
    client: &KeycloakClient,
    token: &str,
    params: &ClientProtocolMapperCreateParams,
) -> Result<(), ApiError> {
    let path = format!(
        "/admin/realms/{}/clients/{}/protocol-mappers/models",
        params.realm,
        urlencoding::encode(&params.id)
    );

    client.post_no_response(&path, token, &params.mapper).await
}

/// Update an existing protocol mapper.
///
/// PUT /admin/realms/{realm}/clients/{id}/protocol-mappers/models/{mapperId}
pub async fn client_protocol_mapper_update(
    client: &KeycloakClient,
    token: &str,
    params: &ClientProtocolMapperUpdateParams,
) -> Result<(), ApiError> {
    let path = format!(
        "/admin/realms/{}/clients/{}/protocol-mappers/models/{}",
        params.realm,
        urlencoding::encode(&params.id),
        urlencoding::encode(&params.mapper_id)
    );

    client.put(&path, token, &params.mapper).await
}

/// Delete a protocol mapper.
///
/// DELETE /admin/realms/{realm}/clients/{id}/protocol-mappers/models/{mapperId}
pub async fn client_protocol_mapper_delete(
    client: &KeycloakClient,
    token: &str,
    params: &ClientProtocolMapperDeleteParams,
) -> Result<(), ApiError> {
    let path = format!(
        "/admin/realms/{}/clients/{}/protocol-mappers/models/{}",
        params.realm,
        urlencoding::encode(&params.id),
        urlencoding::encode(&params.mapper_id)
    );

    client.delete(&path, token).await
}

#[cfg(test)]
mod tests {
    use super::*;
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
    async fn test_client_protocol_mappers_list_success() {
        let mock_server = MockServer::start().await;

        let expected_mappers = vec![sample_mapper()];

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/master/clients/client-uuid/protocol-mappers/models",
            ))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_mappers))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientProtocolMappersListParams {
            realm: "master".to_string(),
            id: "client-uuid".to_string(),
        };

        let result = client_protocol_mappers_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());

        let mappers = result.expect("Failed to get mappers");
        assert_eq!(mappers.len(), 1);
        assert_eq!(mappers[0].name, Some("email-mapper".to_string()));
    }

    #[tokio::test]
    async fn test_client_protocol_mappers_list_empty() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/master/clients/client-uuid/protocol-mappers/models",
            ))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(Vec::<ProtocolMapperRepresentation>::new()),
            )
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientProtocolMappersListParams {
            realm: "master".to_string(),
            id: "client-uuid".to_string(),
        };

        let result = client_protocol_mappers_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());

        let mappers = result.expect("Failed to get mappers");
        assert!(mappers.is_empty());
    }

    #[tokio::test]
    async fn test_client_protocol_mappers_list_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/master/clients/client-uuid/protocol-mappers/models",
            ))
            .respond_with(ResponseTemplate::new(401))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientProtocolMappersListParams {
            realm: "master".to_string(),
            id: "client-uuid".to_string(),
        };

        let result = client_protocol_mappers_list(&client, "invalid-token", &params).await;
        assert!(matches!(result, Err(ApiError::Unauthorized)));
    }

    #[tokio::test]
    async fn test_client_protocol_mappers_list_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/master/clients/nonexistent/protocol-mappers/models",
            ))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientProtocolMappersListParams {
            realm: "master".to_string(),
            id: "nonexistent".to_string(),
        };

        let result = client_protocol_mappers_list(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_client_protocol_mapper_get_success() {
        let mock_server = MockServer::start().await;

        let expected_mapper = sample_mapper();

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/master/clients/client-uuid/protocol-mappers/models/mapper-uuid-123",
            ))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_mapper))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientProtocolMapperGetParams {
            realm: "master".to_string(),
            id: "client-uuid".to_string(),
            mapper_id: "mapper-uuid-123".to_string(),
        };

        let result = client_protocol_mapper_get(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());

        let mapper = result.expect("Failed to get mapper");
        assert_eq!(mapper.id, Some("mapper-uuid-123".to_string()));
        assert_eq!(mapper.name, Some("email-mapper".to_string()));
        assert_eq!(mapper.protocol, Some("openid-connect".to_string()));
    }

    #[tokio::test]
    async fn test_client_protocol_mapper_get_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/master/clients/client-uuid/protocol-mappers/models/nonexistent",
            ))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientProtocolMapperGetParams {
            realm: "master".to_string(),
            id: "client-uuid".to_string(),
            mapper_id: "nonexistent".to_string(),
        };

        let result = client_protocol_mapper_get(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_client_protocol_mapper_create_success() {
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
                "/admin/realms/master/clients/client-uuid/protocol-mappers/models",
            ))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .and(body_json(&new_mapper))
            .respond_with(ResponseTemplate::new(201))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientProtocolMapperCreateParams {
            realm: "master".to_string(),
            id: "client-uuid".to_string(),
            mapper: new_mapper,
        };

        let result = client_protocol_mapper_create(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_client_protocol_mapper_create_conflict() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path(
                "/admin/realms/master/clients/client-uuid/protocol-mappers/models",
            ))
            .respond_with(ResponseTemplate::new(409).set_body_json(serde_json::json!({
                "errorMessage": "Protocol mapper already exists"
            })))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientProtocolMapperCreateParams {
            realm: "master".to_string(),
            id: "client-uuid".to_string(),
            mapper: ProtocolMapperRepresentation {
                name: Some("existing-mapper".to_string()),
                ..Default::default()
            },
        };

        let result = client_protocol_mapper_create(&client, TEST_TOKEN, &params).await;
        match result {
            Err(ApiError::Conflict(msg)) => {
                assert!(msg.contains("Protocol mapper already exists"));
            }
            _ => panic!("Expected Conflict error"),
        }
    }

    #[tokio::test]
    async fn test_client_protocol_mapper_create_forbidden() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path(
                "/admin/realms/master/clients/client-uuid/protocol-mappers/models",
            ))
            .respond_with(ResponseTemplate::new(403))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientProtocolMapperCreateParams {
            realm: "master".to_string(),
            id: "client-uuid".to_string(),
            mapper: ProtocolMapperRepresentation::default(),
        };

        let result = client_protocol_mapper_create(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::Forbidden)));
    }

    #[tokio::test]
    async fn test_client_protocol_mapper_update_success() {
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
                "/admin/realms/master/clients/client-uuid/protocol-mappers/models/mapper-uuid-123",
            ))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .and(body_json(&updated_mapper))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientProtocolMapperUpdateParams {
            realm: "master".to_string(),
            id: "client-uuid".to_string(),
            mapper_id: "mapper-uuid-123".to_string(),
            mapper: updated_mapper,
        };

        let result = client_protocol_mapper_update(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_client_protocol_mapper_update_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path(
                "/admin/realms/master/clients/client-uuid/protocol-mappers/models/nonexistent",
            ))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientProtocolMapperUpdateParams {
            realm: "master".to_string(),
            id: "client-uuid".to_string(),
            mapper_id: "nonexistent".to_string(),
            mapper: ProtocolMapperRepresentation::default(),
        };

        let result = client_protocol_mapper_update(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_client_protocol_mapper_delete_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path(
                "/admin/realms/master/clients/client-uuid/protocol-mappers/models/mapper-uuid-123",
            ))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientProtocolMapperDeleteParams {
            realm: "master".to_string(),
            id: "client-uuid".to_string(),
            mapper_id: "mapper-uuid-123".to_string(),
        };

        let result = client_protocol_mapper_delete(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_client_protocol_mapper_delete_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path(
                "/admin/realms/master/clients/client-uuid/protocol-mappers/models/nonexistent",
            ))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientProtocolMapperDeleteParams {
            realm: "master".to_string(),
            id: "client-uuid".to_string(),
            mapper_id: "nonexistent".to_string(),
        };

        let result = client_protocol_mapper_delete(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_client_protocol_mapper_delete_forbidden() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path(
                "/admin/realms/master/clients/client-uuid/protocol-mappers/models/mapper-uuid-123",
            ))
            .respond_with(ResponseTemplate::new(403))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientProtocolMapperDeleteParams {
            realm: "master".to_string(),
            id: "client-uuid".to_string(),
            mapper_id: "mapper-uuid-123".to_string(),
        };

        let result = client_protocol_mapper_delete(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::Forbidden)));
    }

    #[tokio::test]
    async fn test_protocol_mapper_with_special_characters_in_id() {
        let mock_server = MockServer::start().await;

        let expected_mapper = sample_mapper();

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/my-realm/clients/client%2Fwith%2Bspecial/protocol-mappers/models/mapper%2Fid",
            ))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_mapper))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientProtocolMapperGetParams {
            realm: "my-realm".to_string(),
            id: "client/with+special".to_string(),
            mapper_id: "mapper/id".to_string(),
        };

        let result = client_protocol_mapper_get(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_protocol_mapper_representation_default() {
        let mapper = ProtocolMapperRepresentation::default();
        assert!(mapper.id.is_none());
        assert!(mapper.name.is_none());
        assert!(mapper.protocol.is_none());
        assert!(mapper.protocol_mapper.is_none());
        assert!(mapper.consent_required.is_none());
        assert!(mapper.config.is_none());
    }

    #[test]
    fn test_protocol_mapper_representation_serialization() {
        let mut config = HashMap::new();
        config.insert("claim.name".to_string(), "sub".to_string());

        let mapper = ProtocolMapperRepresentation {
            name: Some("subject-mapper".to_string()),
            protocol: Some("openid-connect".to_string()),
            protocol_mapper: Some("oidc-sub-mapper".to_string()),
            consent_required: Some(false),
            config: Some(config),
            ..Default::default()
        };

        let json = serde_json::to_string(&mapper).expect("Failed to serialize");
        assert!(json.contains("\"name\":\"subject-mapper\""));
        assert!(json.contains("\"protocol\":\"openid-connect\""));
        assert!(json.contains("\"protocolMapper\":\"oidc-sub-mapper\""));
        assert!(json.contains("\"consentRequired\":false"));
        assert!(json.contains("\"config\""));
        // Verify None fields are not serialized
        assert!(!json.contains("\"id\""));
    }

    #[test]
    fn test_protocol_mapper_representation_deserialization() {
        let json = r#"{
            "id": "mapper-123",
            "name": "email-mapper",
            "protocol": "openid-connect",
            "protocolMapper": "oidc-usermodel-attribute-mapper",
            "consentRequired": true,
            "config": {
                "claim.name": "email",
                "jsonType.label": "String"
            }
        }"#;

        let mapper: ProtocolMapperRepresentation =
            serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(mapper.id, Some("mapper-123".to_string()));
        assert_eq!(mapper.name, Some("email-mapper".to_string()));
        assert_eq!(mapper.protocol, Some("openid-connect".to_string()));
        assert_eq!(
            mapper.protocol_mapper,
            Some("oidc-usermodel-attribute-mapper".to_string())
        );
        assert_eq!(mapper.consent_required, Some(true));

        let config = mapper.config.expect("Config should be present");
        assert_eq!(config.get("claim.name"), Some(&"email".to_string()));
        assert_eq!(config.get("jsonType.label"), Some(&"String".to_string()));
    }

    #[test]
    fn test_protocol_mapper_representation_empty_config() {
        let mapper = ProtocolMapperRepresentation {
            name: Some("no-config-mapper".to_string()),
            config: Some(HashMap::new()),
            ..Default::default()
        };

        let json = serde_json::to_string(&mapper).expect("Failed to serialize");
        assert!(json.contains("\"config\":{}"));
    }

    #[test]
    fn test_empty_mapper_serializes_to_empty_object() {
        let mapper = ProtocolMapperRepresentation::default();
        let json = serde_json::to_string(&mapper).expect("Failed to serialize");
        assert_eq!(json, "{}");
    }

    #[test]
    fn test_deserialization_with_unknown_fields() {
        let json = r#"{
            "id": "mapper-123",
            "name": "test-mapper",
            "someUnknownField": "value",
            "anotherUnknown": 42
        }"#;

        let mapper: ProtocolMapperRepresentation =
            serde_json::from_str(json).expect("Failed to deserialize");
        assert_eq!(mapper.id, Some("mapper-123".to_string()));
        assert_eq!(mapper.name, Some("test-mapper".to_string()));
    }
}
