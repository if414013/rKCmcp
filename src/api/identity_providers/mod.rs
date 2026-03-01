//! Keycloak Identity Providers API module.

pub mod mappers;
pub mod types;

pub use mappers::*;
pub use types::*;

use crate::api::{ApiError, KeycloakClient};

pub async fn idp_list(
    client: &KeycloakClient,
    token: &str,
    params: &IdpListParams,
) -> Result<Vec<IdentityProviderRepresentation>, ApiError> {
    let path = format!(
        "/admin/realms/{}/identity-provider/instances",
        urlencoding::encode(&params.realm)
    );

    client.get(&path, token).await
}

pub async fn idp_get(
    client: &KeycloakClient,
    token: &str,
    params: &IdpGetParams,
) -> Result<IdentityProviderRepresentation, ApiError> {
    let path = format!(
        "/admin/realms/{}/identity-provider/instances/{}",
        urlencoding::encode(&params.realm),
        urlencoding::encode(&params.alias)
    );

    client.get(&path, token).await
}

pub async fn idp_create(
    client: &KeycloakClient,
    token: &str,
    params: &IdpCreateParams,
) -> Result<(), ApiError> {
    let path = format!(
        "/admin/realms/{}/identity-provider/instances",
        urlencoding::encode(&params.realm)
    );

    client
        .post_no_response(&path, token, &params.identity_provider)
        .await
}

pub async fn idp_update(
    client: &KeycloakClient,
    token: &str,
    params: &IdpUpdateParams,
) -> Result<(), ApiError> {
    let path = format!(
        "/admin/realms/{}/identity-provider/instances/{}",
        urlencoding::encode(&params.realm),
        urlencoding::encode(&params.alias)
    );

    client.put(&path, token, &params.identity_provider).await
}

pub async fn idp_delete(
    client: &KeycloakClient,
    token: &str,
    params: &IdpDeleteParams,
) -> Result<(), ApiError> {
    let path = format!(
        "/admin/realms/{}/identity-provider/instances/{}",
        urlencoding::encode(&params.realm),
        urlencoding::encode(&params.alias)
    );

    client.delete(&path, token).await
}

pub async fn idp_providers_list(
    client: &KeycloakClient,
    token: &str,
    params: &IdpProvidersGetParams,
) -> Result<IdentityProviderTypeRepresentation, ApiError> {
    let path = format!(
        "/admin/realms/{}/identity-provider/providers/{}",
        urlencoding::encode(&params.realm),
        urlencoding::encode(&params.provider_id)
    );

    client.get(&path, token).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use wiremock::matchers::{body_json, header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    const TEST_TOKEN: &str = "test-bearer-token";

    fn sample_idp() -> IdentityProviderRepresentation {
        let mut config = HashMap::new();
        config.insert("clientId".to_string(), "test-client-id".to_string());
        config.insert(
            "authorizationUrl".to_string(),
            "https://example.com/auth".to_string(),
        );

        IdentityProviderRepresentation {
            alias: Some("google".to_string()),
            display_name: Some("Google".to_string()),
            provider_id: Some("google".to_string()),
            enabled: Some(true),
            trust_email: Some(true),
            store_token: Some(false),
            link_only: Some(false),
            first_broker_login_flow_alias: Some("first broker login".to_string()),
            config: Some(config),
            ..Default::default()
        }
    }

    #[tokio::test]
    async fn test_idp_list_success() {
        let mock_server = MockServer::start().await;

        let expected_idps = vec![sample_idp()];

        Mock::given(method("GET"))
            .and(path("/admin/realms/master/identity-provider/instances"))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_idps))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = IdpListParams {
            realm: "master".to_string(),
        };

        let result = idp_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());

        let idps = result.expect("Failed to get identity providers");
        assert_eq!(idps.len(), 1);
        assert_eq!(idps[0].alias, Some("google".to_string()));
    }

    #[tokio::test]
    async fn test_idp_list_empty() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/admin/realms/test-realm/identity-provider/instances"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(Vec::<IdentityProviderRepresentation>::new()),
            )
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = IdpListParams {
            realm: "test-realm".to_string(),
        };

        let result = idp_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
        assert!(result.expect("Failed to get identity providers").is_empty());
    }

    #[tokio::test]
    async fn test_idp_list_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/admin/realms/master/identity-provider/instances"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = IdpListParams {
            realm: "master".to_string(),
        };

        let result = idp_list(&client, "invalid-token", &params).await;
        assert!(matches!(result, Err(ApiError::Unauthorized)));
    }

    #[tokio::test]
    async fn test_idp_list_forbidden() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/admin/realms/master/identity-provider/instances"))
            .respond_with(ResponseTemplate::new(403))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = IdpListParams {
            realm: "master".to_string(),
        };

        let result = idp_list(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::Forbidden)));
    }

    #[tokio::test]
    async fn test_idp_get_success() {
        let mock_server = MockServer::start().await;

        let expected_idp = sample_idp();

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/master/identity-provider/instances/google",
            ))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_idp))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = IdpGetParams {
            realm: "master".to_string(),
            alias: "google".to_string(),
        };

        let result = idp_get(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());

        let idp = result.expect("Failed to get identity provider");
        assert_eq!(idp.alias, Some("google".to_string()));
        assert_eq!(idp.provider_id, Some("google".to_string()));
    }

    #[tokio::test]
    async fn test_idp_get_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/master/identity-provider/instances/nonexistent",
            ))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = IdpGetParams {
            realm: "master".to_string(),
            alias: "nonexistent".to_string(),
        };

        let result = idp_get(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_idp_get_with_special_characters() {
        let mock_server = MockServer::start().await;

        let expected_idp = IdentityProviderRepresentation {
            alias: Some("my-idp/special".to_string()),
            provider_id: Some("oidc".to_string()),
            ..Default::default()
        };

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/test%20realm/identity-provider/instances/my-idp%2Fspecial",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_idp))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = IdpGetParams {
            realm: "test realm".to_string(),
            alias: "my-idp/special".to_string(),
        };

        let result = idp_get(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_idp_create_success() {
        let mock_server = MockServer::start().await;

        let new_idp = IdentityProviderRepresentation {
            alias: Some("new-oidc".to_string()),
            provider_id: Some("oidc".to_string()),
            enabled: Some(true),
            ..Default::default()
        };

        Mock::given(method("POST"))
            .and(path("/admin/realms/master/identity-provider/instances"))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .and(body_json(&new_idp))
            .respond_with(ResponseTemplate::new(201))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = IdpCreateParams {
            realm: "master".to_string(),
            identity_provider: new_idp,
        };

        let result = idp_create(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_idp_create_conflict() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/admin/realms/master/identity-provider/instances"))
            .respond_with(ResponseTemplate::new(409).set_body_json(serde_json::json!({
                "errorMessage": "Identity Provider already exists"
            })))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = IdpCreateParams {
            realm: "master".to_string(),
            identity_provider: IdentityProviderRepresentation {
                alias: Some("existing-idp".to_string()),
                ..Default::default()
            },
        };

        let result = idp_create(&client, TEST_TOKEN, &params).await;
        match result {
            Err(ApiError::Conflict(msg)) => {
                assert!(msg.contains("Identity Provider already exists"));
            }
            _ => panic!("Expected Conflict error"),
        }
    }

    #[tokio::test]
    async fn test_idp_update_success() {
        let mock_server = MockServer::start().await;

        let updated_idp = IdentityProviderRepresentation {
            alias: Some("google".to_string()),
            display_name: Some("Updated Google".to_string()),
            enabled: Some(false),
            ..Default::default()
        };

        Mock::given(method("PUT"))
            .and(path(
                "/admin/realms/master/identity-provider/instances/google",
            ))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .and(body_json(&updated_idp))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = IdpUpdateParams {
            realm: "master".to_string(),
            alias: "google".to_string(),
            identity_provider: updated_idp,
        };

        let result = idp_update(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_idp_update_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path(
                "/admin/realms/master/identity-provider/instances/nonexistent",
            ))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = IdpUpdateParams {
            realm: "master".to_string(),
            alias: "nonexistent".to_string(),
            identity_provider: IdentityProviderRepresentation::default(),
        };

        let result = idp_update(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_idp_delete_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path(
                "/admin/realms/master/identity-provider/instances/google",
            ))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = IdpDeleteParams {
            realm: "master".to_string(),
            alias: "google".to_string(),
        };

        let result = idp_delete(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_idp_delete_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path(
                "/admin/realms/master/identity-provider/instances/nonexistent",
            ))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = IdpDeleteParams {
            realm: "master".to_string(),
            alias: "nonexistent".to_string(),
        };

        let result = idp_delete(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_idp_providers_list_success() {
        let mock_server = MockServer::start().await;

        let provider_type = IdentityProviderTypeRepresentation {
            id: Some("oidc".to_string()),
            name: Some("OpenID Connect v1.0".to_string()),
        };

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/master/identity-provider/providers/oidc",
            ))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&provider_type))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = IdpProvidersGetParams {
            realm: "master".to_string(),
            provider_id: "oidc".to_string(),
        };

        let result = idp_providers_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());

        let provider = result.expect("Failed to get provider");
        assert_eq!(provider.id, Some("oidc".to_string()));
        assert_eq!(provider.name, Some("OpenID Connect v1.0".to_string()));
    }

    #[tokio::test]
    async fn test_idp_providers_list_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/master/identity-provider/providers/unknown",
            ))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = IdpProvidersGetParams {
            realm: "master".to_string(),
            provider_id: "unknown".to_string(),
        };

        let result = idp_providers_list(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_idp_list_with_special_characters_in_realm() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/realm%20with%20spaces/identity-provider/instances",
            ))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(Vec::<IdentityProviderRepresentation>::new()),
            )
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = IdpListParams {
            realm: "realm with spaces".to_string(),
        };

        let result = idp_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }
}
