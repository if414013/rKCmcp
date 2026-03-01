//! Keycloak Realms Sessions Admin API module.
//!
//! Provides operations for managing active sessions and logout operations.

use crate::api::{ApiError, KeycloakClient};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Client session statistics representation.
///
/// Statistics about active and offline sessions for a specific client.
#[derive(Debug, Clone, Serialize, Deserialize, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClientSessionStats {
    /// Client UUID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Client ID string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,

    /// Number of active sessions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<i64>,

    /// Number of offline sessions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offline: Option<i64>,
}

/// Global request result representation.
///
/// Result of a realm-wide operation showing success/failure counts.
#[derive(Debug, Clone, Serialize, Deserialize, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GlobalRequestResult {
    /// Number of successful requests
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_requests: Option<Vec<String>>,

    /// Number of failed requests
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_requests: Option<Vec<String>>,
}

/// Parameters for listing client session statistics.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RealmSessionsListParams {
    /// The realm name
    pub realm: String,
}

/// Parameters for logging out all sessions in a realm.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RealmLogoutAllParams {
    /// The realm name
    pub realm: String,
}

/// Parameters for pushing revocation policy to clients.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RealmPushRevocationParams {
    /// The realm name
    pub realm: String,
}

/// Get client session statistics for a realm.
///
/// GET /admin/realms/{realm}/client-session-stats
pub async fn realm_sessions_list(
    client: &KeycloakClient,
    token: &str,
    params: &RealmSessionsListParams,
) -> Result<Vec<ClientSessionStats>, ApiError> {
    let path = format!(
        "/admin/realms/{}/client-session-stats",
        urlencoding::encode(&params.realm)
    );

    client.get(&path, token).await
}

/// Logout all active sessions in a realm.
///
/// POST /admin/realms/{realm}/logout-all
pub async fn realm_logout_all(
    client: &KeycloakClient,
    token: &str,
    params: &RealmLogoutAllParams,
) -> Result<GlobalRequestResult, ApiError> {
    let path = format!(
        "/admin/realms/{}/logout-all",
        urlencoding::encode(&params.realm)
    );

    client.post(&path, token, &serde_json::Value::Null).await
}

/// Push revocation policy to all clients in a realm.
///
/// POST /admin/realms/{realm}/push-revocation
pub async fn realm_push_revocation(
    client: &KeycloakClient,
    token: &str,
    params: &RealmPushRevocationParams,
) -> Result<GlobalRequestResult, ApiError> {
    let path = format!(
        "/admin/realms/{}/push-revocation",
        urlencoding::encode(&params.realm)
    );

    client.post(&path, token, &serde_json::Value::Null).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    const TEST_TOKEN: &str = "test-bearer-token";

    fn sample_session_stats() -> ClientSessionStats {
        ClientSessionStats {
            id: Some("client-uuid-123".to_string()),
            client_id: Some("my-app".to_string()),
            active: Some(42),
            offline: Some(5),
        }
    }

    // ==================== realm_sessions_list tests ====================

    #[tokio::test]
    async fn test_realm_sessions_list_success() {
        let mock_server = MockServer::start().await;

        let expected_stats = vec![sample_session_stats()];

        Mock::given(method("GET"))
            .and(path("/admin/realms/test-realm/client-session-stats"))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_stats))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmSessionsListParams {
            realm: "test-realm".to_string(),
        };

        let result = realm_sessions_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());

        let stats = result.expect("Failed to get session stats");
        assert_eq!(stats.len(), 1);
        assert_eq!(stats[0].client_id, Some("my-app".to_string()));
        assert_eq!(stats[0].active, Some(42));
        assert_eq!(stats[0].offline, Some(5));
    }

    #[tokio::test]
    async fn test_realm_sessions_list_empty() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/admin/realms/test-realm/client-session-stats"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(Vec::<ClientSessionStats>::new()),
            )
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmSessionsListParams {
            realm: "test-realm".to_string(),
        };

        let result = realm_sessions_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_realm_sessions_list_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/admin/realms/nonexistent/client-session-stats"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmSessionsListParams {
            realm: "nonexistent".to_string(),
        };

        let result = realm_sessions_list(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_realm_sessions_list_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/admin/realms/test-realm/client-session-stats"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmSessionsListParams {
            realm: "test-realm".to_string(),
        };

        let result = realm_sessions_list(&client, "invalid-token", &params).await;
        assert!(matches!(result, Err(ApiError::Unauthorized)));
    }

    #[tokio::test]
    async fn test_realm_sessions_list_with_special_characters() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/admin/realms/my%20realm/client-session-stats"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(Vec::<ClientSessionStats>::new()),
            )
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmSessionsListParams {
            realm: "my realm".to_string(),
        };

        let result = realm_sessions_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    // ==================== realm_logout_all tests ====================

    #[tokio::test]
    async fn test_realm_logout_all_success() {
        let mock_server = MockServer::start().await;

        let expected_result = GlobalRequestResult {
            success_requests: Some(vec!["client-1".to_string(), "client-2".to_string()]),
            failed_requests: Some(vec![]),
        };

        Mock::given(method("POST"))
            .and(path("/admin/realms/test-realm/logout-all"))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_result))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmLogoutAllParams {
            realm: "test-realm".to_string(),
        };

        let result = realm_logout_all(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());

        let logout_result = result.expect("Failed to logout all");
        assert_eq!(
            logout_result.success_requests,
            Some(vec!["client-1".to_string(), "client-2".to_string()])
        );
    }

    #[tokio::test]
    async fn test_realm_logout_all_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/admin/realms/nonexistent/logout-all"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmLogoutAllParams {
            realm: "nonexistent".to_string(),
        };

        let result = realm_logout_all(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_realm_logout_all_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/admin/realms/test-realm/logout-all"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmLogoutAllParams {
            realm: "test-realm".to_string(),
        };

        let result = realm_logout_all(&client, "invalid-token", &params).await;
        assert!(matches!(result, Err(ApiError::Unauthorized)));
    }

    // ==================== realm_push_revocation tests ====================

    #[tokio::test]
    async fn test_realm_push_revocation_success() {
        let mock_server = MockServer::start().await;

        let expected_result = GlobalRequestResult {
            success_requests: Some(vec!["client-1".to_string()]),
            failed_requests: Some(vec!["client-2".to_string()]),
        };

        Mock::given(method("POST"))
            .and(path("/admin/realms/test-realm/push-revocation"))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_result))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmPushRevocationParams {
            realm: "test-realm".to_string(),
        };

        let result = realm_push_revocation(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());

        let revocation_result = result.expect("Failed to push revocation");
        assert_eq!(
            revocation_result.success_requests,
            Some(vec!["client-1".to_string()])
        );
        assert_eq!(
            revocation_result.failed_requests,
            Some(vec!["client-2".to_string()])
        );
    }

    #[tokio::test]
    async fn test_realm_push_revocation_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/admin/realms/nonexistent/push-revocation"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmPushRevocationParams {
            realm: "nonexistent".to_string(),
        };

        let result = realm_push_revocation(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_realm_push_revocation_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/admin/realms/test-realm/push-revocation"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmPushRevocationParams {
            realm: "test-realm".to_string(),
        };

        let result = realm_push_revocation(&client, "invalid-token", &params).await;
        assert!(matches!(result, Err(ApiError::Unauthorized)));
    }
}
