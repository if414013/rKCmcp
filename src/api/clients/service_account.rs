//! Keycloak Client Service Account API module.
//!
//! Provides tools for managing service account users associated with clients.

use serde::Deserialize;

use crate::api::users::types::UserRepresentation;
use crate::api::{ApiError, KeycloakClient};

/// Parameters for getting the service account user for a client.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClientServiceAccountUserGetParams {
    /// The realm name
    pub realm: String,

    /// The client UUID (not client_id)
    pub id: String,
}

/// Get the service account user for a client.
///
/// Returns the user that represents this client's service account.
/// This is only applicable for clients with serviceAccountsEnabled=true.
///
/// GET /admin/realms/{realm}/clients/{id}/service-account-user
pub async fn client_service_account_user_get(
    client: &KeycloakClient,
    token: &str,
    params: &ClientServiceAccountUserGetParams,
) -> Result<UserRepresentation, ApiError> {
    let path = format!(
        "/admin/realms/{}/clients/{}/service-account-user",
        params.realm,
        urlencoding::encode(&params.id)
    );

    client.get(&path, token).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    const TEST_TOKEN: &str = "test-bearer-token";

    fn sample_service_account_user() -> UserRepresentation {
        UserRepresentation {
            id: Some("service-account-uuid".to_string()),
            username: Some("service-account-my-client".to_string()),
            enabled: Some(true),
            service_account_client_id: Some("my-client".to_string()),
            ..Default::default()
        }
    }

    #[tokio::test]
    async fn test_client_service_account_user_get_success() {
        let mock_server = MockServer::start().await;

        let expected_user = sample_service_account_user();

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/master/clients/client-uuid/service-account-user",
            ))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_user))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientServiceAccountUserGetParams {
            realm: "master".to_string(),
            id: "client-uuid".to_string(),
        };

        let result = client_service_account_user_get(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());

        let user = result.expect("Failed to get service account user");
        assert_eq!(user.id, Some("service-account-uuid".to_string()));
        assert_eq!(user.username, Some("service-account-my-client".to_string()));
        assert_eq!(
            user.service_account_client_id,
            Some("my-client".to_string())
        );
    }

    #[tokio::test]
    async fn test_client_service_account_user_get_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/master/clients/nonexistent/service-account-user",
            ))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientServiceAccountUserGetParams {
            realm: "master".to_string(),
            id: "nonexistent".to_string(),
        };

        let result = client_service_account_user_get(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_client_service_account_user_get_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/master/clients/client-uuid/service-account-user",
            ))
            .respond_with(ResponseTemplate::new(401))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientServiceAccountUserGetParams {
            realm: "master".to_string(),
            id: "client-uuid".to_string(),
        };

        let result = client_service_account_user_get(&client, "invalid-token", &params).await;
        assert!(matches!(result, Err(ApiError::Unauthorized)));
    }

    #[tokio::test]
    async fn test_client_service_account_user_get_forbidden() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/master/clients/client-uuid/service-account-user",
            ))
            .respond_with(ResponseTemplate::new(403))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientServiceAccountUserGetParams {
            realm: "master".to_string(),
            id: "client-uuid".to_string(),
        };

        let result = client_service_account_user_get(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::Forbidden)));
    }

    #[tokio::test]
    async fn test_client_service_account_user_get_with_special_characters() {
        let mock_server = MockServer::start().await;

        let expected_user = sample_service_account_user();

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/my-realm/clients/client%2Fwith%2Bspecial/service-account-user",
            ))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_user))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientServiceAccountUserGetParams {
            realm: "my-realm".to_string(),
            id: "client/with+special".to_string(),
        };

        let result = client_service_account_user_get(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_client_service_account_user_get_public_client_error() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/master/clients/public-client-uuid/service-account-user",
            ))
            .respond_with(ResponseTemplate::new(400).set_body_json(serde_json::json!({
                "errorMessage": "Client does not have service accounts enabled"
            })))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientServiceAccountUserGetParams {
            realm: "master".to_string(),
            id: "public-client-uuid".to_string(),
        };

        let result = client_service_account_user_get(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::BadRequest(_))));
    }
}
