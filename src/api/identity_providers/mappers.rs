//! Keycloak Identity Provider Mappers API module.
//!
//! Provides tools for managing mappers on identity providers.
//! Identity provider mappers define how external identity provider data is mapped
//! into Keycloak user attributes and roles.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::api::{ApiError, KeycloakClient};

/// Representation of an identity provider mapper in Keycloak.
///
/// Identity provider mappers configure how attributes and roles from external
/// identity providers are mapped to Keycloak users during authentication.
#[derive(Debug, Clone, Serialize, Deserialize, Default, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct IdentityProviderMapperRepresentation {
    /// Unique identifier for the mapper (UUID)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Name of the mapper
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Alias of the identity provider this mapper belongs to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_provider_alias: Option<String>,

    /// Type of identity provider mapper (e.g., "saml-user-attribute-idp-mapper")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_provider_mapper: Option<String>,

    /// Configuration properties for the mapper
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<HashMap<String, String>>,
}

/// Representation of an identity provider mapper type descriptor.
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct IdentityProviderMapperTypeRepresentation {
    /// Unique identifier for the mapper type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Display name of the mapper type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Category of the mapper type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,

    /// Help text describing the mapper type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub help_text: Option<String>,

    /// Configuration properties definition
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<HashMap<String, serde_json::Value>>>,
}

/// Parameters for listing identity provider mappers.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct IdpMappersListParams {
    /// The realm name
    pub realm: String,

    /// The identity provider alias
    pub alias: String,
}

/// Parameters for getting a single identity provider mapper.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct IdpMapperGetParams {
    /// The realm name
    pub realm: String,

    /// The identity provider alias
    pub alias: String,

    /// The mapper ID
    pub id: String,
}

/// Parameters for creating an identity provider mapper.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct IdpMapperCreateParams {
    /// The realm name
    pub realm: String,

    /// The identity provider alias
    pub alias: String,

    /// The mapper to create
    pub mapper: IdentityProviderMapperRepresentation,
}

/// Parameters for updating an identity provider mapper.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct IdpMapperUpdateParams {
    /// The realm name
    pub realm: String,

    /// The identity provider alias
    pub alias: String,

    /// The mapper ID
    pub id: String,

    /// The updated mapper representation
    pub mapper: IdentityProviderMapperRepresentation,
}

/// Parameters for deleting an identity provider mapper.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct IdpMapperDeleteParams {
    /// The realm name
    pub realm: String,

    /// The identity provider alias
    pub alias: String,

    /// The mapper ID
    pub id: String,
}

/// Parameters for listing identity provider mapper types.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct IdpMapperTypesListParams {
    /// The realm name
    pub realm: String,

    /// The identity provider alias
    pub alias: String,
}

/// List all mappers for an identity provider.
///
/// GET /admin/realms/{realm}/identity-provider/instances/{alias}/mappers
pub async fn idp_mappers_list(
    client: &KeycloakClient,
    token: &str,
    params: &IdpMappersListParams,
) -> Result<Vec<IdentityProviderMapperRepresentation>, ApiError> {
    let path = format!(
        "/admin/realms/{}/identity-provider/instances/{}/mappers",
        urlencoding::encode(&params.realm),
        urlencoding::encode(&params.alias)
    );

    client.get(&path, token).await
}

/// Get a single identity provider mapper by ID.
///
/// GET /admin/realms/{realm}/identity-provider/instances/{alias}/mappers/{id}
pub async fn idp_mapper_get(
    client: &KeycloakClient,
    token: &str,
    params: &IdpMapperGetParams,
) -> Result<IdentityProviderMapperRepresentation, ApiError> {
    let path = format!(
        "/admin/realms/{}/identity-provider/instances/{}/mappers/{}",
        urlencoding::encode(&params.realm),
        urlencoding::encode(&params.alias),
        params.id
    );

    client.get(&path, token).await
}

/// Create a new mapper for an identity provider.
///
/// POST /admin/realms/{realm}/identity-provider/instances/{alias}/mappers
pub async fn idp_mapper_create(
    client: &KeycloakClient,
    token: &str,
    params: &IdpMapperCreateParams,
) -> Result<(), ApiError> {
    let path = format!(
        "/admin/realms/{}/identity-provider/instances/{}/mappers",
        urlencoding::encode(&params.realm),
        urlencoding::encode(&params.alias)
    );

    client.post_no_response(&path, token, &params.mapper).await
}

/// Update an existing identity provider mapper.
///
/// PUT /admin/realms/{realm}/identity-provider/instances/{alias}/mappers/{id}
pub async fn idp_mapper_update(
    client: &KeycloakClient,
    token: &str,
    params: &IdpMapperUpdateParams,
) -> Result<(), ApiError> {
    let path = format!(
        "/admin/realms/{}/identity-provider/instances/{}/mappers/{}",
        urlencoding::encode(&params.realm),
        urlencoding::encode(&params.alias),
        params.id
    );

    client.put(&path, token, &params.mapper).await
}

/// Delete an identity provider mapper.
///
/// DELETE /admin/realms/{realm}/identity-provider/instances/{alias}/mappers/{id}
pub async fn idp_mapper_delete(
    client: &KeycloakClient,
    token: &str,
    params: &IdpMapperDeleteParams,
) -> Result<(), ApiError> {
    let path = format!(
        "/admin/realms/{}/identity-provider/instances/{}/mappers/{}",
        urlencoding::encode(&params.realm),
        urlencoding::encode(&params.alias),
        params.id
    );

    client.delete(&path, token).await
}

/// Get available mapper types for an identity provider.
///
/// GET /admin/realms/{realm}/identity-provider/instances/{alias}/mapper-types
pub async fn idp_mapper_types_list(
    client: &KeycloakClient,
    token: &str,
    params: &IdpMapperTypesListParams,
) -> Result<HashMap<String, IdentityProviderMapperTypeRepresentation>, ApiError> {
    let path = format!(
        "/admin/realms/{}/identity-provider/instances/{}/mapper-types",
        urlencoding::encode(&params.realm),
        urlencoding::encode(&params.alias)
    );

    client.get(&path, token).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{body_json, header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    const TEST_TOKEN: &str = "test-bearer-token";

    fn sample_mapper() -> IdentityProviderMapperRepresentation {
        let mut config = HashMap::new();
        config.insert("attribute.name".to_string(), "email".to_string());
        config.insert("user.attribute".to_string(), "email".to_string());

        IdentityProviderMapperRepresentation {
            id: Some("mapper-uuid-123".to_string()),
            name: Some("email-mapper".to_string()),
            identity_provider_alias: Some("google".to_string()),
            identity_provider_mapper: Some("saml-user-attribute-idp-mapper".to_string()),
            config: Some(config),
        }
    }

    fn sample_mapper_type() -> IdentityProviderMapperTypeRepresentation {
        IdentityProviderMapperTypeRepresentation {
            id: Some("saml-user-attribute-idp-mapper".to_string()),
            name: Some("Attribute Importer".to_string()),
            category: Some("Attribute Importer".to_string()),
            help_text: Some("Import user attribute from SAML assertion".to_string()),
            properties: Some(vec![]),
        }
    }

    #[tokio::test]
    async fn test_idp_mappers_list_success() {
        let mock_server = MockServer::start().await;

        let expected_mappers = vec![sample_mapper()];

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/master/identity-provider/instances/google/mappers",
            ))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_mappers))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = IdpMappersListParams {
            realm: "master".to_string(),
            alias: "google".to_string(),
        };

        let result = idp_mappers_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());

        let mappers = result.expect("Failed to get mappers");
        assert_eq!(mappers.len(), 1);
        assert_eq!(mappers[0].name, Some("email-mapper".to_string()));
    }

    #[tokio::test]
    async fn test_idp_mappers_list_empty() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/master/identity-provider/instances/google/mappers",
            ))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(Vec::<IdentityProviderMapperRepresentation>::new()),
            )
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = IdpMappersListParams {
            realm: "master".to_string(),
            alias: "google".to_string(),
        };

        let result = idp_mappers_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());

        let mappers = result.expect("Failed to get mappers");
        assert!(mappers.is_empty());
    }

    #[tokio::test]
    async fn test_idp_mappers_list_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/master/identity-provider/instances/google/mappers",
            ))
            .respond_with(ResponseTemplate::new(401))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = IdpMappersListParams {
            realm: "master".to_string(),
            alias: "google".to_string(),
        };

        let result = idp_mappers_list(&client, "invalid-token", &params).await;
        assert!(matches!(result, Err(ApiError::Unauthorized)));
    }

    #[tokio::test]
    async fn test_idp_mappers_list_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/master/identity-provider/instances/nonexistent/mappers",
            ))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = IdpMappersListParams {
            realm: "master".to_string(),
            alias: "nonexistent".to_string(),
        };

        let result = idp_mappers_list(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_idp_mapper_get_success() {
        let mock_server = MockServer::start().await;

        let expected_mapper = sample_mapper();

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/master/identity-provider/instances/google/mappers/mapper-uuid-123",
            ))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_mapper))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = IdpMapperGetParams {
            realm: "master".to_string(),
            alias: "google".to_string(),
            id: "mapper-uuid-123".to_string(),
        };

        let result = idp_mapper_get(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());

        let mapper = result.expect("Failed to get mapper");
        assert_eq!(mapper.id, Some("mapper-uuid-123".to_string()));
        assert_eq!(mapper.name, Some("email-mapper".to_string()));
        assert_eq!(mapper.identity_provider_alias, Some("google".to_string()));
    }

    #[tokio::test]
    async fn test_idp_mapper_get_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/master/identity-provider/instances/google/mappers/nonexistent",
            ))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = IdpMapperGetParams {
            realm: "master".to_string(),
            alias: "google".to_string(),
            id: "nonexistent".to_string(),
        };

        let result = idp_mapper_get(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_idp_mapper_create_success() {
        let mock_server = MockServer::start().await;

        let new_mapper = IdentityProviderMapperRepresentation {
            name: Some("new-mapper".to_string()),
            identity_provider_alias: Some("google".to_string()),
            identity_provider_mapper: Some("saml-user-attribute-idp-mapper".to_string()),
            config: Some(HashMap::new()),
            ..Default::default()
        };

        Mock::given(method("POST"))
            .and(path(
                "/admin/realms/master/identity-provider/instances/google/mappers",
            ))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .and(body_json(&new_mapper))
            .respond_with(ResponseTemplate::new(201))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = IdpMapperCreateParams {
            realm: "master".to_string(),
            alias: "google".to_string(),
            mapper: new_mapper,
        };

        let result = idp_mapper_create(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_idp_mapper_create_conflict() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path(
                "/admin/realms/master/identity-provider/instances/google/mappers",
            ))
            .respond_with(ResponseTemplate::new(409).set_body_json(serde_json::json!({
                "errorMessage": "Mapper already exists"
            })))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = IdpMapperCreateParams {
            realm: "master".to_string(),
            alias: "google".to_string(),
            mapper: IdentityProviderMapperRepresentation {
                name: Some("existing-mapper".to_string()),
                ..Default::default()
            },
        };

        let result = idp_mapper_create(&client, TEST_TOKEN, &params).await;
        match result {
            Err(ApiError::Conflict(msg)) => {
                assert!(msg.contains("Mapper already exists"));
            }
            _ => panic!("Expected Conflict error"),
        }
    }

    #[tokio::test]
    async fn test_idp_mapper_create_forbidden() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path(
                "/admin/realms/master/identity-provider/instances/google/mappers",
            ))
            .respond_with(ResponseTemplate::new(403))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = IdpMapperCreateParams {
            realm: "master".to_string(),
            alias: "google".to_string(),
            mapper: IdentityProviderMapperRepresentation::default(),
        };

        let result = idp_mapper_create(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::Forbidden)));
    }

    #[tokio::test]
    async fn test_idp_mapper_update_success() {
        let mock_server = MockServer::start().await;

        let updated_mapper = IdentityProviderMapperRepresentation {
            id: Some("mapper-uuid-123".to_string()),
            name: Some("updated-mapper".to_string()),
            identity_provider_alias: Some("google".to_string()),
            identity_provider_mapper: Some("saml-user-attribute-idp-mapper".to_string()),
            config: Some(HashMap::new()),
        };

        Mock::given(method("PUT"))
            .and(path(
                "/admin/realms/master/identity-provider/instances/google/mappers/mapper-uuid-123",
            ))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .and(body_json(&updated_mapper))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = IdpMapperUpdateParams {
            realm: "master".to_string(),
            alias: "google".to_string(),
            id: "mapper-uuid-123".to_string(),
            mapper: updated_mapper,
        };

        let result = idp_mapper_update(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_idp_mapper_update_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path(
                "/admin/realms/master/identity-provider/instances/google/mappers/nonexistent",
            ))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = IdpMapperUpdateParams {
            realm: "master".to_string(),
            alias: "google".to_string(),
            id: "nonexistent".to_string(),
            mapper: IdentityProviderMapperRepresentation::default(),
        };

        let result = idp_mapper_update(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_idp_mapper_delete_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path(
                "/admin/realms/master/identity-provider/instances/google/mappers/mapper-uuid-123",
            ))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = IdpMapperDeleteParams {
            realm: "master".to_string(),
            alias: "google".to_string(),
            id: "mapper-uuid-123".to_string(),
        };

        let result = idp_mapper_delete(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_idp_mapper_delete_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path(
                "/admin/realms/master/identity-provider/instances/google/mappers/nonexistent",
            ))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = IdpMapperDeleteParams {
            realm: "master".to_string(),
            alias: "google".to_string(),
            id: "nonexistent".to_string(),
        };

        let result = idp_mapper_delete(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_idp_mapper_delete_forbidden() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path(
                "/admin/realms/master/identity-provider/instances/google/mappers/mapper-uuid-123",
            ))
            .respond_with(ResponseTemplate::new(403))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = IdpMapperDeleteParams {
            realm: "master".to_string(),
            alias: "google".to_string(),
            id: "mapper-uuid-123".to_string(),
        };

        let result = idp_mapper_delete(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::Forbidden)));
    }

    #[tokio::test]
    async fn test_idp_mapper_types_list_success() {
        let mock_server = MockServer::start().await;

        let mut expected_types = HashMap::new();
        expected_types.insert(
            "saml-user-attribute-idp-mapper".to_string(),
            sample_mapper_type(),
        );

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/master/identity-provider/instances/google/mapper-types",
            ))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_types))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = IdpMapperTypesListParams {
            realm: "master".to_string(),
            alias: "google".to_string(),
        };

        let result = idp_mapper_types_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());

        let types = result.expect("Failed to get mapper types");
        assert_eq!(types.len(), 1);
        assert!(types.contains_key("saml-user-attribute-idp-mapper"));
    }

    #[tokio::test]
    async fn test_idp_mapper_types_list_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/master/identity-provider/instances/google/mapper-types",
            ))
            .respond_with(ResponseTemplate::new(401))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = IdpMapperTypesListParams {
            realm: "master".to_string(),
            alias: "google".to_string(),
        };

        let result = idp_mapper_types_list(&client, "invalid-token", &params).await;
        assert!(matches!(result, Err(ApiError::Unauthorized)));
    }

    #[tokio::test]
    async fn test_idp_mapper_types_list_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/master/identity-provider/instances/nonexistent/mapper-types",
            ))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = IdpMapperTypesListParams {
            realm: "master".to_string(),
            alias: "nonexistent".to_string(),
        };

        let result = idp_mapper_types_list(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_idp_mapper_with_special_characters() {
        let mock_server = MockServer::start().await;

        let expected_mapper = sample_mapper();

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/my%2Frealm/identity-provider/instances/provider%2Bspecial/mappers/mapper-uuid-123",
            ))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_mapper))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = IdpMapperGetParams {
            realm: "my/realm".to_string(),
            alias: "provider+special".to_string(),
            id: "mapper-uuid-123".to_string(),
        };

        let result = idp_mapper_get(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_idp_mapper_representation_default() {
        let mapper = IdentityProviderMapperRepresentation::default();
        assert!(mapper.id.is_none());
        assert!(mapper.name.is_none());
        assert!(mapper.identity_provider_alias.is_none());
        assert!(mapper.identity_provider_mapper.is_none());
        assert!(mapper.config.is_none());
    }

    #[test]
    fn test_idp_mapper_representation_serialization() {
        let mut config = HashMap::new();
        config.insert("attribute.name".to_string(), "email".to_string());

        let mapper = IdentityProviderMapperRepresentation {
            name: Some("email-mapper".to_string()),
            identity_provider_alias: Some("google".to_string()),
            identity_provider_mapper: Some("saml-user-attribute-idp-mapper".to_string()),
            config: Some(config),
            ..Default::default()
        };

        let json = serde_json::to_string(&mapper).expect("Failed to serialize");
        assert!(json.contains("\"name\":\"email-mapper\""));
        assert!(json.contains("\"identityProviderAlias\":\"google\""));
        assert!(json.contains("\"identityProviderMapper\":\"saml-user-attribute-idp-mapper\""));
        assert!(json.contains("\"config\""));
        // Verify None fields are not serialized
        assert!(!json.contains("\"id\""));
    }

    #[test]
    fn test_idp_mapper_representation_deserialization() {
        let json = r#"{
            "id": "mapper-123",
            "name": "email-mapper",
            "identityProviderAlias": "google",
            "identityProviderMapper": "saml-user-attribute-idp-mapper",
            "config": {
                "attribute.name": "email",
                "user.attribute": "email"
            }
        }"#;

        let mapper: IdentityProviderMapperRepresentation =
            serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(mapper.id, Some("mapper-123".to_string()));
        assert_eq!(mapper.name, Some("email-mapper".to_string()));
        assert_eq!(mapper.identity_provider_alias, Some("google".to_string()));
        assert_eq!(
            mapper.identity_provider_mapper,
            Some("saml-user-attribute-idp-mapper".to_string())
        );

        let config = mapper.config.expect("Config should be present");
        assert_eq!(config.get("attribute.name"), Some(&"email".to_string()));
        assert_eq!(config.get("user.attribute"), Some(&"email".to_string()));
    }

    #[test]
    fn test_idp_mapper_representation_empty_config() {
        let mapper = IdentityProviderMapperRepresentation {
            name: Some("no-config-mapper".to_string()),
            config: Some(HashMap::new()),
            ..Default::default()
        };

        let json = serde_json::to_string(&mapper).expect("Failed to serialize");
        assert!(json.contains("\"config\":{}"));
    }

    #[test]
    fn test_empty_mapper_serializes_to_empty_object() {
        let mapper = IdentityProviderMapperRepresentation::default();
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

        let mapper: IdentityProviderMapperRepresentation =
            serde_json::from_str(json).expect("Failed to deserialize");
        assert_eq!(mapper.id, Some("mapper-123".to_string()));
        assert_eq!(mapper.name, Some("test-mapper".to_string()));
    }
}
