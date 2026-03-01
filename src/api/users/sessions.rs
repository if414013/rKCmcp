//! User sessions, consents, and federated identity management tools for Keycloak Admin REST API.
//!
//! This module provides tools for managing user sessions, consents, and federated identities:
//! - Listing active user sessions
//! - Logging out users (invalidating all sessions)
//! - Managing user consents for client applications
//! - Managing federated identity provider links

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::api::{ApiError, KeycloakClient};

use super::types::FederatedIdentityRepresentation;

/// Represents a user session in Keycloak.
///
/// Contains information about an active user session including client sessions.
#[derive(Debug, Clone, Serialize, Deserialize, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserSessionRepresentation {
    /// Session ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Username of the session owner
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    /// User ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,

    /// IP address of the client
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,

    /// When the session started (milliseconds since epoch)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<i64>,

    /// Last access time (milliseconds since epoch)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_access: Option<i64>,

    /// Client sessions associated with this user session
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clients: Option<HashMap<String, String>>,
}

/// Represents a user consent in Keycloak.
///
/// Contains information about a user's consent to share data with a client application.
#[derive(Debug, Clone, Serialize, Deserialize, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserConsentRepresentation {
    /// Client ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,

    /// When the consent was granted (milliseconds since epoch)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<i64>,

    /// When the consent was last updated (milliseconds since epoch)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date: Option<i64>,

    /// Granted client scopes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granted_client_scopes: Option<Vec<String>>,

    /// Granted protocol mappers (DEPRECATED)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granted_protocol_mappers: Option<HashMap<String, String>>,
}

/// Parameters for listing user sessions.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct UserSessionsListParams {
    /// The realm name
    pub realm: String,
    /// The user ID
    pub user_id: String,
}

impl UserSessionsListParams {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.realm.trim().is_empty() {
            return Err(ApiError::BadRequest("realm is required".to_string()));
        }
        if self.user_id.trim().is_empty() {
            return Err(ApiError::BadRequest("user_id is required".to_string()));
        }
        Ok(())
    }
}

/// Parameters for logging out a user.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct UserLogoutParams {
    /// The realm name
    pub realm: String,
    /// The user ID
    pub user_id: String,
}

impl UserLogoutParams {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.realm.trim().is_empty() {
            return Err(ApiError::BadRequest("realm is required".to_string()));
        }
        if self.user_id.trim().is_empty() {
            return Err(ApiError::BadRequest("user_id is required".to_string()));
        }
        Ok(())
    }
}

/// Parameters for listing user consents.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct UserConsentsListParams {
    /// The realm name
    pub realm: String,
    /// The user ID
    pub user_id: String,
}

impl UserConsentsListParams {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.realm.trim().is_empty() {
            return Err(ApiError::BadRequest("realm is required".to_string()));
        }
        if self.user_id.trim().is_empty() {
            return Err(ApiError::BadRequest("user_id is required".to_string()));
        }
        Ok(())
    }
}

/// Parameters for revoking a user consent.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct UserConsentRevokeParams {
    /// The realm name
    pub realm: String,
    /// The user ID
    pub user_id: String,
    /// The client ID whose consent to revoke
    pub client_id: String,
}

impl UserConsentRevokeParams {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.realm.trim().is_empty() {
            return Err(ApiError::BadRequest("realm is required".to_string()));
        }
        if self.user_id.trim().is_empty() {
            return Err(ApiError::BadRequest("user_id is required".to_string()));
        }
        if self.client_id.trim().is_empty() {
            return Err(ApiError::BadRequest("client_id is required".to_string()));
        }
        Ok(())
    }
}

/// Parameters for listing federated identities.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct UserFederatedIdentityListParams {
    /// The realm name
    pub realm: String,
    /// The user ID
    pub user_id: String,
}

impl UserFederatedIdentityListParams {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.realm.trim().is_empty() {
            return Err(ApiError::BadRequest("realm is required".to_string()));
        }
        if self.user_id.trim().is_empty() {
            return Err(ApiError::BadRequest("user_id is required".to_string()));
        }
        Ok(())
    }
}

/// Parameters for adding a federated identity.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct UserFederatedIdentityAddParams {
    /// The realm name
    pub realm: String,
    /// The user ID
    pub user_id: String,
    /// The identity provider alias
    pub provider: String,
    /// The federated identity to add
    pub identity: FederatedIdentityRepresentation,
}

impl UserFederatedIdentityAddParams {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.realm.trim().is_empty() {
            return Err(ApiError::BadRequest("realm is required".to_string()));
        }
        if self.user_id.trim().is_empty() {
            return Err(ApiError::BadRequest("user_id is required".to_string()));
        }
        if self.provider.trim().is_empty() {
            return Err(ApiError::BadRequest("provider is required".to_string()));
        }
        Ok(())
    }
}

/// Parameters for removing a federated identity.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct UserFederatedIdentityRemoveParams {
    /// The realm name
    pub realm: String,
    /// The user ID
    pub user_id: String,
    /// The identity provider alias
    pub provider: String,
}

impl UserFederatedIdentityRemoveParams {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.realm.trim().is_empty() {
            return Err(ApiError::BadRequest("realm is required".to_string()));
        }
        if self.user_id.trim().is_empty() {
            return Err(ApiError::BadRequest("user_id is required".to_string()));
        }
        if self.provider.trim().is_empty() {
            return Err(ApiError::BadRequest("provider is required".to_string()));
        }
        Ok(())
    }
}

/// List active user sessions.
///
/// Returns a list of all active sessions for the user.
///
/// # Endpoint
/// `GET /admin/realms/{realm}/users/{id}/sessions`
pub async fn user_sessions_list(
    client: &KeycloakClient,
    token: &str,
    params: &UserSessionsListParams,
) -> Result<Vec<UserSessionRepresentation>, ApiError> {
    params.validate()?;

    let path = format!(
        "/admin/realms/{}/users/{}/sessions",
        params.realm, params.user_id
    );

    client.get(&path, token).await
}

/// Log out a user.
///
/// Invalidates all active sessions for the user.
///
/// # Endpoint
/// `POST /admin/realms/{realm}/users/{id}/logout`
pub async fn user_logout(
    client: &KeycloakClient,
    token: &str,
    params: &UserLogoutParams,
) -> Result<(), ApiError> {
    params.validate()?;

    let path = format!(
        "/admin/realms/{}/users/{}/logout",
        params.realm, params.user_id
    );

    client.post_no_response(&path, token, &serde_json::Value::Null).await
}

/// List user consents.
///
/// Returns all consents the user has granted to client applications.
///
/// # Endpoint
/// `GET /admin/realms/{realm}/users/{id}/consents`
pub async fn user_consents_list(
    client: &KeycloakClient,
    token: &str,
    params: &UserConsentsListParams,
) -> Result<Vec<UserConsentRepresentation>, ApiError> {
    params.validate()?;

    let path = format!(
        "/admin/realms/{}/users/{}/consents",
        params.realm, params.user_id
    );

    client.get(&path, token).await
}

/// Revoke a user consent for a client.
///
/// Removes the user's consent for the specified client application.
///
/// # Endpoint
/// `DELETE /admin/realms/{realm}/users/{id}/consents/{client}`
pub async fn user_consent_revoke(
    client: &KeycloakClient,
    token: &str,
    params: &UserConsentRevokeParams,
) -> Result<(), ApiError> {
    params.validate()?;

    let path = format!(
        "/admin/realms/{}/users/{}/consents/{}",
        params.realm, params.user_id, params.client_id
    );

    client.delete(&path, token).await
}

/// List federated identities linked to a user.
///
/// Returns all federated identity provider links for the user.
///
/// # Endpoint
/// `GET /admin/realms/{realm}/users/{id}/federated-identity`
pub async fn user_federated_identity_list(
    client: &KeycloakClient,
    token: &str,
    params: &UserFederatedIdentityListParams,
) -> Result<Vec<FederatedIdentityRepresentation>, ApiError> {
    params.validate()?;

    let path = format!(
        "/admin/realms/{}/users/{}/federated-identity",
        params.realm, params.user_id
    );

    client.get(&path, token).await
}

/// Add a federated identity to a user.
///
/// Links a federated identity provider to the user's account.
///
/// # Endpoint
/// `POST /admin/realms/{realm}/users/{id}/federated-identity/{provider}`
pub async fn user_federated_identity_add(
    client: &KeycloakClient,
    token: &str,
    params: &UserFederatedIdentityAddParams,
) -> Result<(), ApiError> {
    params.validate()?;

    let path = format!(
        "/admin/realms/{}/users/{}/federated-identity/{}",
        params.realm, params.user_id, params.provider
    );

    client.post_no_response(&path, token, &params.identity).await
}

/// Remove a federated identity from a user.
///
/// Unlinks a federated identity provider from the user's account.
///
/// # Endpoint
/// `DELETE /admin/realms/{realm}/users/{id}/federated-identity/{provider}`
pub async fn user_federated_identity_remove(
    client: &KeycloakClient,
    token: &str,
    params: &UserFederatedIdentityRemoveParams,
) -> Result<(), ApiError> {
    params.validate()?;

    let path = format!(
        "/admin/realms/{}/users/{}/federated-identity/{}",
        params.realm, params.user_id, params.provider
    );

    client.delete(&path, token).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    const TEST_TOKEN: &str = "test-bearer-token";

    mod user_sessions_list_params_tests {
        use super::*;

        #[test]
        fn test_validate_success() {
            let params = UserSessionsListParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
            };
            assert!(params.validate().is_ok());
        }

        #[test]
        fn test_validate_empty_realm() {
            let params = UserSessionsListParams {
                realm: "".to_string(),
                user_id: "123".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_user_id() {
            let params = UserSessionsListParams {
                realm: "master".to_string(),
                user_id: "".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }
    }

    mod user_logout_params_tests {
        use super::*;

        #[test]
        fn test_validate_success() {
            let params = UserLogoutParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
            };
            assert!(params.validate().is_ok());
        }

        #[test]
        fn test_validate_empty_realm() {
            let params = UserLogoutParams {
                realm: "".to_string(),
                user_id: "123".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_user_id() {
            let params = UserLogoutParams {
                realm: "master".to_string(),
                user_id: "".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }
    }

    mod user_consents_list_params_tests {
        use super::*;

        #[test]
        fn test_validate_success() {
            let params = UserConsentsListParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
            };
            assert!(params.validate().is_ok());
        }

        #[test]
        fn test_validate_empty_realm() {
            let params = UserConsentsListParams {
                realm: "".to_string(),
                user_id: "123".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_user_id() {
            let params = UserConsentsListParams {
                realm: "master".to_string(),
                user_id: "".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }
    }

    mod user_consent_revoke_params_tests {
        use super::*;

        #[test]
        fn test_validate_success() {
            let params = UserConsentRevokeParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                client_id: "my-client".to_string(),
            };
            assert!(params.validate().is_ok());
        }

        #[test]
        fn test_validate_empty_realm() {
            let params = UserConsentRevokeParams {
                realm: "".to_string(),
                user_id: "123".to_string(),
                client_id: "my-client".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_user_id() {
            let params = UserConsentRevokeParams {
                realm: "master".to_string(),
                user_id: "".to_string(),
                client_id: "my-client".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_client_id() {
            let params = UserConsentRevokeParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                client_id: "".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }
    }

    mod user_federated_identity_list_params_tests {
        use super::*;

        #[test]
        fn test_validate_success() {
            let params = UserFederatedIdentityListParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
            };
            assert!(params.validate().is_ok());
        }

        #[test]
        fn test_validate_empty_realm() {
            let params = UserFederatedIdentityListParams {
                realm: "".to_string(),
                user_id: "123".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_user_id() {
            let params = UserFederatedIdentityListParams {
                realm: "master".to_string(),
                user_id: "".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }
    }

    mod user_federated_identity_add_params_tests {
        use super::*;

        #[test]
        fn test_validate_success() {
            let params = UserFederatedIdentityAddParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                provider: "google".to_string(),
                identity: FederatedIdentityRepresentation::default(),
            };
            assert!(params.validate().is_ok());
        }

        #[test]
        fn test_validate_empty_realm() {
            let params = UserFederatedIdentityAddParams {
                realm: "".to_string(),
                user_id: "123".to_string(),
                provider: "google".to_string(),
                identity: FederatedIdentityRepresentation::default(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_user_id() {
            let params = UserFederatedIdentityAddParams {
                realm: "master".to_string(),
                user_id: "".to_string(),
                provider: "google".to_string(),
                identity: FederatedIdentityRepresentation::default(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_provider() {
            let params = UserFederatedIdentityAddParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                provider: "".to_string(),
                identity: FederatedIdentityRepresentation::default(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }
    }

    mod user_federated_identity_remove_params_tests {
        use super::*;

        #[test]
        fn test_validate_success() {
            let params = UserFederatedIdentityRemoveParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                provider: "google".to_string(),
            };
            assert!(params.validate().is_ok());
        }

        #[test]
        fn test_validate_empty_realm() {
            let params = UserFederatedIdentityRemoveParams {
                realm: "".to_string(),
                user_id: "123".to_string(),
                provider: "google".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_user_id() {
            let params = UserFederatedIdentityRemoveParams {
                realm: "master".to_string(),
                user_id: "".to_string(),
                provider: "google".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_provider() {
            let params = UserFederatedIdentityRemoveParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                provider: "".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }
    }

    mod user_sessions_list_tests {
        use super::*;

        #[tokio::test]
        async fn test_sessions_list_success() {
            let mock_server = MockServer::start().await;

            let expected_sessions = vec![
                UserSessionRepresentation {
                    id: Some("session-1".to_string()),
                    username: Some("testuser".to_string()),
                    user_id: Some("123".to_string()),
                    ip_address: Some("192.168.1.100".to_string()),
                    start: Some(1609459200000),
                    last_access: Some(1609459800000),
                    clients: None,
                },
                UserSessionRepresentation {
                    id: Some("session-2".to_string()),
                    username: Some("testuser".to_string()),
                    user_id: Some("123".to_string()),
                    ip_address: Some("192.168.1.101".to_string()),
                    start: Some(1609460000000),
                    last_access: Some(1609460600000),
                    clients: None,
                },
            ];

            Mock::given(method("GET"))
                .and(path("/admin/realms/master/users/123/sessions"))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .respond_with(ResponseTemplate::new(200).set_body_json(&expected_sessions))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserSessionsListParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
            };

            let sessions = user_sessions_list(&client, TEST_TOKEN, &params)
                .await
                .expect("Failed to list sessions");
            assert_eq!(sessions.len(), 2);
            assert_eq!(sessions[0].id.as_deref(), Some("session-1"));
        }

        #[tokio::test]
        async fn test_sessions_list_empty() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path("/admin/realms/master/users/123/sessions"))
                .respond_with(
                    ResponseTemplate::new(200).set_body_json::<Vec<UserSessionRepresentation>>(vec![]),
                )
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserSessionsListParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
            };

            let sessions = user_sessions_list(&client, TEST_TOKEN, &params)
                .await
                .expect("Failed to list sessions");
            assert!(sessions.is_empty());
        }

        #[tokio::test]
        async fn test_sessions_list_user_not_found() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path("/admin/realms/master/users/999/sessions"))
                .respond_with(ResponseTemplate::new(404))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserSessionsListParams {
                realm: "master".to_string(),
                user_id: "999".to_string(),
            };

            let result = user_sessions_list(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::NotFound)));
        }

        #[tokio::test]
        async fn test_sessions_list_validation_fails() {
            let mock_server = MockServer::start().await;
            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserSessionsListParams {
                realm: "".to_string(),
                user_id: "123".to_string(),
            };

            let result = user_sessions_list(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::BadRequest(_))));
        }
    }

    mod user_logout_tests {
        use super::*;

        #[tokio::test]
        async fn test_logout_success() {
            let mock_server = MockServer::start().await;

            Mock::given(method("POST"))
                .and(path("/admin/realms/master/users/123/logout"))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .respond_with(ResponseTemplate::new(204))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserLogoutParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
            };

            let result = user_logout(&client, TEST_TOKEN, &params).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_logout_user_not_found() {
            let mock_server = MockServer::start().await;

            Mock::given(method("POST"))
                .and(path("/admin/realms/master/users/999/logout"))
                .respond_with(ResponseTemplate::new(404))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserLogoutParams {
                realm: "master".to_string(),
                user_id: "999".to_string(),
            };

            let result = user_logout(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::NotFound)));
        }

        #[tokio::test]
        async fn test_logout_validation_fails() {
            let mock_server = MockServer::start().await;
            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserLogoutParams {
                realm: "master".to_string(),
                user_id: "".to_string(),
            };

            let result = user_logout(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::BadRequest(_))));
        }
    }

    mod user_consents_list_tests {
        use super::*;

        #[tokio::test]
        async fn test_consents_list_success() {
            let mock_server = MockServer::start().await;

            let expected_consents = vec![
                UserConsentRepresentation {
                    client_id: Some("client-1".to_string()),
                    created_date: Some(1609459200000),
                    last_updated_date: Some(1609459800000),
                    granted_client_scopes: Some(vec!["profile".to_string(), "email".to_string()]),
                    granted_protocol_mappers: None,
                },
            ];

            Mock::given(method("GET"))
                .and(path("/admin/realms/master/users/123/consents"))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .respond_with(ResponseTemplate::new(200).set_body_json(&expected_consents))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserConsentsListParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
            };

            let consents = user_consents_list(&client, TEST_TOKEN, &params)
                .await
                .expect("Failed to list consents");
            assert_eq!(consents.len(), 1);
            assert_eq!(consents[0].client_id.as_deref(), Some("client-1"));
        }

        #[tokio::test]
        async fn test_consents_list_empty() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path("/admin/realms/master/users/123/consents"))
                .respond_with(
                    ResponseTemplate::new(200).set_body_json::<Vec<UserConsentRepresentation>>(vec![]),
                )
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserConsentsListParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
            };

            let consents = user_consents_list(&client, TEST_TOKEN, &params)
                .await
                .expect("Failed to list consents");
            assert!(consents.is_empty());
        }

        #[tokio::test]
        async fn test_consents_list_validation_fails() {
            let mock_server = MockServer::start().await;
            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserConsentsListParams {
                realm: "".to_string(),
                user_id: "123".to_string(),
            };

            let result = user_consents_list(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::BadRequest(_))));
        }
    }

    mod user_consent_revoke_tests {
        use super::*;

        #[tokio::test]
        async fn test_consent_revoke_success() {
            let mock_server = MockServer::start().await;

            Mock::given(method("DELETE"))
                .and(path("/admin/realms/master/users/123/consents/my-client"))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .respond_with(ResponseTemplate::new(204))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserConsentRevokeParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                client_id: "my-client".to_string(),
            };

            let result = user_consent_revoke(&client, TEST_TOKEN, &params).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_consent_revoke_user_not_found() {
            let mock_server = MockServer::start().await;

            Mock::given(method("DELETE"))
                .and(path("/admin/realms/master/users/999/consents/my-client"))
                .respond_with(ResponseTemplate::new(404))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserConsentRevokeParams {
                realm: "master".to_string(),
                user_id: "999".to_string(),
                client_id: "my-client".to_string(),
            };

            let result = user_consent_revoke(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::NotFound)));
        }

        #[tokio::test]
        async fn test_consent_revoke_validation_fails() {
            let mock_server = MockServer::start().await;
            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserConsentRevokeParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                client_id: "".to_string(),
            };

            let result = user_consent_revoke(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::BadRequest(_))));
        }
    }

    mod user_federated_identity_list_tests {
        use super::*;

        #[tokio::test]
        async fn test_federated_identity_list_success() {
            let mock_server = MockServer::start().await;

            let expected_identities = vec![
                FederatedIdentityRepresentation {
                    identity_provider: Some("google".to_string()),
                    user_id: Some("google-123".to_string()),
                    user_name: Some("googleuser".to_string()),
                },
            ];

            Mock::given(method("GET"))
                .and(path("/admin/realms/master/users/123/federated-identity"))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .respond_with(ResponseTemplate::new(200).set_body_json(&expected_identities))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserFederatedIdentityListParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
            };

            let identities = user_federated_identity_list(&client, TEST_TOKEN, &params)
                .await
                .expect("Failed to list federated identities");
            assert_eq!(identities.len(), 1);
            assert_eq!(identities[0].identity_provider.as_deref(), Some("google"));
        }

        #[tokio::test]
        async fn test_federated_identity_list_empty() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path("/admin/realms/master/users/123/federated-identity"))
                .respond_with(
                    ResponseTemplate::new(200).set_body_json::<Vec<FederatedIdentityRepresentation>>(vec![]),
                )
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserFederatedIdentityListParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
            };

            let identities = user_federated_identity_list(&client, TEST_TOKEN, &params)
                .await
                .expect("Failed to list federated identities");
            assert!(identities.is_empty());
        }

        #[tokio::test]
        async fn test_federated_identity_list_validation_fails() {
            let mock_server = MockServer::start().await;
            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserFederatedIdentityListParams {
                realm: "".to_string(),
                user_id: "123".to_string(),
            };

            let result = user_federated_identity_list(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::BadRequest(_))));
        }
    }

    mod user_federated_identity_add_tests {
        use super::*;

        #[tokio::test]
        async fn test_federated_identity_add_success() {
            let mock_server = MockServer::start().await;

            Mock::given(method("POST"))
                .and(path("/admin/realms/master/users/123/federated-identity/google"))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .respond_with(ResponseTemplate::new(204))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserFederatedIdentityAddParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                provider: "google".to_string(),
                identity: FederatedIdentityRepresentation {
                    identity_provider: Some("google".to_string()),
                    user_id: Some("google-123".to_string()),
                    user_name: Some("googleuser".to_string()),
                },
            };

            let result = user_federated_identity_add(&client, TEST_TOKEN, &params).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_federated_identity_add_user_not_found() {
            let mock_server = MockServer::start().await;

            Mock::given(method("POST"))
                .and(path("/admin/realms/master/users/999/federated-identity/google"))
                .respond_with(ResponseTemplate::new(404))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserFederatedIdentityAddParams {
                realm: "master".to_string(),
                user_id: "999".to_string(),
                provider: "google".to_string(),
                identity: FederatedIdentityRepresentation::default(),
            };

            let result = user_federated_identity_add(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::NotFound)));
        }

        #[tokio::test]
        async fn test_federated_identity_add_validation_fails() {
            let mock_server = MockServer::start().await;
            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserFederatedIdentityAddParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                provider: "".to_string(),
                identity: FederatedIdentityRepresentation::default(),
            };

            let result = user_federated_identity_add(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::BadRequest(_))));
        }
    }

    mod user_federated_identity_remove_tests {
        use super::*;

        #[tokio::test]
        async fn test_federated_identity_remove_success() {
            let mock_server = MockServer::start().await;

            Mock::given(method("DELETE"))
                .and(path("/admin/realms/master/users/123/federated-identity/google"))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .respond_with(ResponseTemplate::new(204))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserFederatedIdentityRemoveParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                provider: "google".to_string(),
            };

            let result = user_federated_identity_remove(&client, TEST_TOKEN, &params).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_federated_identity_remove_user_not_found() {
            let mock_server = MockServer::start().await;

            Mock::given(method("DELETE"))
                .and(path("/admin/realms/master/users/999/federated-identity/google"))
                .respond_with(ResponseTemplate::new(404))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserFederatedIdentityRemoveParams {
                realm: "master".to_string(),
                user_id: "999".to_string(),
                provider: "google".to_string(),
            };

            let result = user_federated_identity_remove(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::NotFound)));
        }

        #[tokio::test]
        async fn test_federated_identity_remove_validation_fails() {
            let mock_server = MockServer::start().await;
            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserFederatedIdentityRemoveParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                provider: "".to_string(),
            };

            let result = user_federated_identity_remove(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::BadRequest(_))));
        }
    }
}
