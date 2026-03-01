//! User credential management tools for Keycloak Admin REST API.
//!
//! This module provides tools for managing user credentials including:
//! - Resetting passwords
//! - Listing credentials
//! - Deleting credentials
//! - Disabling credential types
//! - Sending verification emails
//! - Executing required actions via email

use serde::Deserialize;

use crate::api::{ApiError, KeycloakClient};

use super::types::CredentialRepresentation;

/// Parameters for resetting a user's password.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct UserResetPasswordParams {
    /// The realm name
    pub realm: String,
    /// The user ID
    pub user_id: String,
    /// The new password credential
    pub credential: CredentialRepresentation,
}

impl UserResetPasswordParams {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.realm.trim().is_empty() {
            return Err(ApiError::BadRequest("realm is required".to_string()));
        }
        if self.user_id.trim().is_empty() {
            return Err(ApiError::BadRequest("user_id is required".to_string()));
        }
        if self.credential.value.as_ref().is_none_or(|v| v.is_empty()) {
            return Err(ApiError::BadRequest(
                "credential value (password) is required".to_string(),
            ));
        }
        Ok(())
    }
}

/// Parameters for listing user credentials.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct UserCredentialsListParams {
    /// The realm name
    pub realm: String,
    /// The user ID
    pub user_id: String,
}

impl UserCredentialsListParams {
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

/// Parameters for deleting a user credential.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct UserCredentialDeleteParams {
    /// The realm name
    pub realm: String,
    /// The user ID
    pub user_id: String,
    /// The credential ID to delete
    pub credential_id: String,
}

impl UserCredentialDeleteParams {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.realm.trim().is_empty() {
            return Err(ApiError::BadRequest("realm is required".to_string()));
        }
        if self.user_id.trim().is_empty() {
            return Err(ApiError::BadRequest("user_id is required".to_string()));
        }
        if self.credential_id.trim().is_empty() {
            return Err(ApiError::BadRequest("credential_id is required".to_string()));
        }
        Ok(())
    }
}

/// Parameters for disabling credential types for a user.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct UserDisableCredentialsParams {
    /// The realm name
    pub realm: String,
    /// The user ID
    pub user_id: String,
    /// List of credential types to disable (e.g., ["password", "otp"])
    pub credential_types: Vec<String>,
}

impl UserDisableCredentialsParams {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.realm.trim().is_empty() {
            return Err(ApiError::BadRequest("realm is required".to_string()));
        }
        if self.user_id.trim().is_empty() {
            return Err(ApiError::BadRequest("user_id is required".to_string()));
        }
        if self.credential_types.is_empty() {
            return Err(ApiError::BadRequest(
                "at least one credential type is required".to_string(),
            ));
        }
        Ok(())
    }
}

/// Parameters for sending a verification email to a user.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct UserSendVerifyEmailParams {
    /// The realm name
    pub realm: String,
    /// The user ID
    pub user_id: String,
    /// Optional client ID to associate with the verification
    #[serde(default)]
    pub client_id: Option<String>,
    /// Optional redirect URI after verification
    #[serde(default)]
    pub redirect_uri: Option<String>,
}

impl UserSendVerifyEmailParams {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.realm.trim().is_empty() {
            return Err(ApiError::BadRequest("realm is required".to_string()));
        }
        if self.user_id.trim().is_empty() {
            return Err(ApiError::BadRequest("user_id is required".to_string()));
        }
        Ok(())
    }

    pub fn build_query_string(&self) -> String {
        let mut params = Vec::new();

        if let Some(ref client_id) = self.client_id {
            params.push(format!("client_id={}", urlencoding::encode(client_id)));
        }
        if let Some(ref redirect_uri) = self.redirect_uri {
            params.push(format!("redirect_uri={}", urlencoding::encode(redirect_uri)));
        }

        if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        }
    }
}

/// Parameters for sending an execute actions email to a user.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct UserExecuteActionsEmailParams {
    /// The realm name
    pub realm: String,
    /// The user ID
    pub user_id: String,
    /// List of required actions to execute (e.g., ["UPDATE_PASSWORD", "VERIFY_EMAIL"])
    pub actions: Vec<String>,
    /// Optional client ID to associate with the actions
    #[serde(default)]
    pub client_id: Option<String>,
    /// Optional redirect URI after actions are completed
    #[serde(default)]
    pub redirect_uri: Option<String>,
    /// Optional lifespan of the action token in seconds
    #[serde(default)]
    pub lifespan: Option<i32>,
}

impl UserExecuteActionsEmailParams {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.realm.trim().is_empty() {
            return Err(ApiError::BadRequest("realm is required".to_string()));
        }
        if self.user_id.trim().is_empty() {
            return Err(ApiError::BadRequest("user_id is required".to_string()));
        }
        if self.actions.is_empty() {
            return Err(ApiError::BadRequest(
                "at least one action is required".to_string(),
            ));
        }
        if let Some(lifespan) = self.lifespan {
            if lifespan <= 0 {
                return Err(ApiError::BadRequest(
                    "lifespan must be positive".to_string(),
                ));
            }
        }
        Ok(())
    }

    pub fn build_query_string(&self) -> String {
        let mut params = Vec::new();

        if let Some(ref client_id) = self.client_id {
            params.push(format!("client_id={}", urlencoding::encode(client_id)));
        }
        if let Some(ref redirect_uri) = self.redirect_uri {
            params.push(format!("redirect_uri={}", urlencoding::encode(redirect_uri)));
        }
        if let Some(lifespan) = self.lifespan {
            params.push(format!("lifespan={}", lifespan));
        }

        if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        }
    }
}

/// Reset a user's password.
///
/// Sets a new password for the user. The password credential should include:
/// - `value`: The new password
/// - `temporary`: Whether the user must change the password on next login
///
/// # Endpoint
/// `PUT /admin/realms/{realm}/users/{id}/reset-password`
pub async fn user_reset_password(
    client: &KeycloakClient,
    token: &str,
    params: &UserResetPasswordParams,
) -> Result<(), ApiError> {
    params.validate()?;

    let path = format!(
        "/admin/realms/{}/users/{}/reset-password",
        params.realm, params.user_id
    );
    client.put(&path, token, &params.credential).await
}

/// List all credentials for a user.
///
/// Returns the list of credentials associated with a user.
/// Note: Secret data is not returned in the response for security.
///
/// # Endpoint
/// `GET /admin/realms/{realm}/users/{id}/credentials`
pub async fn user_credentials_list(
    client: &KeycloakClient,
    token: &str,
    params: &UserCredentialsListParams,
) -> Result<Vec<CredentialRepresentation>, ApiError> {
    params.validate()?;

    let path = format!(
        "/admin/realms/{}/users/{}/credentials",
        params.realm, params.user_id
    );
    client.get(&path, token).await
}

/// Delete a specific credential for a user.
///
/// Removes a credential from the user. This is useful for removing
/// old passwords, OTP configurations, etc.
///
/// # Endpoint
/// `DELETE /admin/realms/{realm}/users/{id}/credentials/{credentialId}`
pub async fn user_credential_delete(
    client: &KeycloakClient,
    token: &str,
    params: &UserCredentialDeleteParams,
) -> Result<(), ApiError> {
    params.validate()?;

    let path = format!(
        "/admin/realms/{}/users/{}/credentials/{}",
        params.realm, params.user_id, params.credential_id
    );
    client.delete(&path, token).await
}

/// Disable credential types for a user.
///
/// Disables specified credential types for the user. The user will
/// not be able to use these credential types until they are re-enabled.
///
/// # Endpoint
/// `PUT /admin/realms/{realm}/users/{id}/disable-credential-types`
pub async fn user_disable_credentials(
    client: &KeycloakClient,
    token: &str,
    params: &UserDisableCredentialsParams,
) -> Result<(), ApiError> {
    params.validate()?;

    let path = format!(
        "/admin/realms/{}/users/{}/disable-credential-types",
        params.realm, params.user_id
    );
    client.put(&path, token, &params.credential_types).await
}

/// Send a verification email to a user.
///
/// Sends an email to the user with a link to verify their email address.
/// The user must have an email address configured.
///
/// # Endpoint
/// `PUT /admin/realms/{realm}/users/{id}/send-verify-email`
pub async fn user_send_verify_email(
    client: &KeycloakClient,
    token: &str,
    params: &UserSendVerifyEmailParams,
) -> Result<(), ApiError> {
    params.validate()?;

    let query = params.build_query_string();
    let path = format!(
        "/admin/realms/{}/users/{}/send-verify-email{}",
        params.realm, params.user_id, query
    );
    // Keycloak expects an empty body for this PUT request
    client.put(&path, token, &serde_json::Value::Null).await
}

/// Send an execute actions email to a user.
///
/// Sends an email to the user with a link to perform the specified
/// required actions. Common actions include:
/// - `UPDATE_PASSWORD`: Force password change
/// - `VERIFY_EMAIL`: Verify email address
/// - `UPDATE_PROFILE`: Update user profile
/// - `CONFIGURE_TOTP`: Configure two-factor authentication
///
/// # Endpoint
/// `PUT /admin/realms/{realm}/users/{id}/execute-actions-email`
pub async fn user_execute_actions_email(
    client: &KeycloakClient,
    token: &str,
    params: &UserExecuteActionsEmailParams,
) -> Result<(), ApiError> {
    params.validate()?;

    let query = params.build_query_string();
    let path = format!(
        "/admin/realms/{}/users/{}/execute-actions-email{}",
        params.realm, params.user_id, query
    );
    client.put(&path, token, &params.actions).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{body_json, header, method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    const TEST_TOKEN: &str = "test-bearer-token";

    mod user_reset_password_params_tests {
        use super::*;

        #[test]
        fn test_validate_success() {
            let params = UserResetPasswordParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                credential: CredentialRepresentation {
                    value: Some("newpassword".to_string()),
                    temporary: Some(false),
                    ..Default::default()
                },
            };
            assert!(params.validate().is_ok());
        }

        #[test]
        fn test_validate_empty_realm() {
            let params = UserResetPasswordParams {
                realm: "".to_string(),
                user_id: "123".to_string(),
                credential: CredentialRepresentation {
                    value: Some("newpassword".to_string()),
                    ..Default::default()
                },
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_whitespace_realm() {
            let params = UserResetPasswordParams {
                realm: "   ".to_string(),
                user_id: "123".to_string(),
                credential: CredentialRepresentation {
                    value: Some("newpassword".to_string()),
                    ..Default::default()
                },
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_user_id() {
            let params = UserResetPasswordParams {
                realm: "master".to_string(),
                user_id: "".to_string(),
                credential: CredentialRepresentation {
                    value: Some("newpassword".to_string()),
                    ..Default::default()
                },
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_missing_password() {
            let params = UserResetPasswordParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                credential: CredentialRepresentation::default(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_password() {
            let params = UserResetPasswordParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                credential: CredentialRepresentation {
                    value: Some("".to_string()),
                    ..Default::default()
                },
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }
    }

    mod user_credentials_list_params_tests {
        use super::*;

        #[test]
        fn test_validate_success() {
            let params = UserCredentialsListParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
            };
            assert!(params.validate().is_ok());
        }

        #[test]
        fn test_validate_empty_realm() {
            let params = UserCredentialsListParams {
                realm: "".to_string(),
                user_id: "123".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_user_id() {
            let params = UserCredentialsListParams {
                realm: "master".to_string(),
                user_id: "".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }
    }

    mod user_credential_delete_params_tests {
        use super::*;

        #[test]
        fn test_validate_success() {
            let params = UserCredentialDeleteParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                credential_id: "cred-456".to_string(),
            };
            assert!(params.validate().is_ok());
        }

        #[test]
        fn test_validate_empty_realm() {
            let params = UserCredentialDeleteParams {
                realm: "".to_string(),
                user_id: "123".to_string(),
                credential_id: "cred-456".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_user_id() {
            let params = UserCredentialDeleteParams {
                realm: "master".to_string(),
                user_id: "".to_string(),
                credential_id: "cred-456".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_credential_id() {
            let params = UserCredentialDeleteParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                credential_id: "".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }
    }

    mod user_disable_credentials_params_tests {
        use super::*;

        #[test]
        fn test_validate_success() {
            let params = UserDisableCredentialsParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                credential_types: vec!["password".to_string()],
            };
            assert!(params.validate().is_ok());
        }

        #[test]
        fn test_validate_empty_realm() {
            let params = UserDisableCredentialsParams {
                realm: "".to_string(),
                user_id: "123".to_string(),
                credential_types: vec!["password".to_string()],
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_user_id() {
            let params = UserDisableCredentialsParams {
                realm: "master".to_string(),
                user_id: "".to_string(),
                credential_types: vec!["password".to_string()],
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_credential_types() {
            let params = UserDisableCredentialsParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                credential_types: vec![],
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }
    }

    mod user_send_verify_email_params_tests {
        use super::*;

        #[test]
        fn test_validate_success() {
            let params = UserSendVerifyEmailParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                client_id: None,
                redirect_uri: None,
            };
            assert!(params.validate().is_ok());
        }

        #[test]
        fn test_validate_empty_realm() {
            let params = UserSendVerifyEmailParams {
                realm: "".to_string(),
                user_id: "123".to_string(),
                client_id: None,
                redirect_uri: None,
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_user_id() {
            let params = UserSendVerifyEmailParams {
                realm: "master".to_string(),
                user_id: "".to_string(),
                client_id: None,
                redirect_uri: None,
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_build_query_string_empty() {
            let params = UserSendVerifyEmailParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                client_id: None,
                redirect_uri: None,
            };
            assert_eq!(params.build_query_string(), "");
        }

        #[test]
        fn test_build_query_string_with_client_id() {
            let params = UserSendVerifyEmailParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                client_id: Some("my-client".to_string()),
                redirect_uri: None,
            };
            assert_eq!(params.build_query_string(), "?client_id=my-client");
        }

        #[test]
        fn test_build_query_string_with_all_params() {
            let params = UserSendVerifyEmailParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                client_id: Some("my-client".to_string()),
                redirect_uri: Some("http://localhost/callback".to_string()),
            };
            let query = params.build_query_string();
            assert!(query.contains("client_id=my-client"));
            assert!(query.contains("redirect_uri=http%3A%2F%2Flocalhost%2Fcallback"));
        }
    }

    mod user_execute_actions_email_params_tests {
        use super::*;

        #[test]
        fn test_validate_success() {
            let params = UserExecuteActionsEmailParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                actions: vec!["UPDATE_PASSWORD".to_string()],
                client_id: None,
                redirect_uri: None,
                lifespan: None,
            };
            assert!(params.validate().is_ok());
        }

        #[test]
        fn test_validate_empty_realm() {
            let params = UserExecuteActionsEmailParams {
                realm: "".to_string(),
                user_id: "123".to_string(),
                actions: vec!["UPDATE_PASSWORD".to_string()],
                client_id: None,
                redirect_uri: None,
                lifespan: None,
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_user_id() {
            let params = UserExecuteActionsEmailParams {
                realm: "master".to_string(),
                user_id: "".to_string(),
                actions: vec!["UPDATE_PASSWORD".to_string()],
                client_id: None,
                redirect_uri: None,
                lifespan: None,
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_actions() {
            let params = UserExecuteActionsEmailParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                actions: vec![],
                client_id: None,
                redirect_uri: None,
                lifespan: None,
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_negative_lifespan() {
            let params = UserExecuteActionsEmailParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                actions: vec!["UPDATE_PASSWORD".to_string()],
                client_id: None,
                redirect_uri: None,
                lifespan: Some(-1),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_zero_lifespan() {
            let params = UserExecuteActionsEmailParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                actions: vec!["UPDATE_PASSWORD".to_string()],
                client_id: None,
                redirect_uri: None,
                lifespan: Some(0),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_build_query_string_empty() {
            let params = UserExecuteActionsEmailParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                actions: vec!["UPDATE_PASSWORD".to_string()],
                client_id: None,
                redirect_uri: None,
                lifespan: None,
            };
            assert_eq!(params.build_query_string(), "");
        }

        #[test]
        fn test_build_query_string_with_lifespan() {
            let params = UserExecuteActionsEmailParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                actions: vec!["UPDATE_PASSWORD".to_string()],
                client_id: None,
                redirect_uri: None,
                lifespan: Some(3600),
            };
            assert_eq!(params.build_query_string(), "?lifespan=3600");
        }

        #[test]
        fn test_build_query_string_with_all_params() {
            let params = UserExecuteActionsEmailParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                actions: vec!["UPDATE_PASSWORD".to_string()],
                client_id: Some("my-client".to_string()),
                redirect_uri: Some("http://localhost/callback".to_string()),
                lifespan: Some(3600),
            };
            let query = params.build_query_string();
            assert!(query.contains("client_id=my-client"));
            assert!(query.contains("redirect_uri=http%3A%2F%2Flocalhost%2Fcallback"));
            assert!(query.contains("lifespan=3600"));
        }
    }

    mod user_reset_password_tests {
        use super::*;

        #[tokio::test]
        async fn test_reset_password_success() {
            let mock_server = MockServer::start().await;

            Mock::given(method("PUT"))
                .and(path("/admin/realms/master/users/123/reset-password"))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .respond_with(ResponseTemplate::new(204))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserResetPasswordParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                credential: CredentialRepresentation {
                    credential_type: Some("password".to_string()),
                    value: Some("newpassword123".to_string()),
                    temporary: Some(false),
                    ..Default::default()
                },
            };

            let result = user_reset_password(&client, TEST_TOKEN, &params).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_reset_password_user_not_found() {
            let mock_server = MockServer::start().await;

            Mock::given(method("PUT"))
                .and(path("/admin/realms/master/users/999/reset-password"))
                .respond_with(ResponseTemplate::new(404))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserResetPasswordParams {
                realm: "master".to_string(),
                user_id: "999".to_string(),
                credential: CredentialRepresentation {
                    value: Some("newpassword".to_string()),
                    ..Default::default()
                },
            };

            let result = user_reset_password(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::NotFound)));
        }

        #[tokio::test]
        async fn test_reset_password_validation_fails() {
            let mock_server = MockServer::start().await;
            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserResetPasswordParams {
                realm: "".to_string(),
                user_id: "123".to_string(),
                credential: CredentialRepresentation {
                    value: Some("newpassword".to_string()),
                    ..Default::default()
                },
            };

            let result = user_reset_password(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::BadRequest(_))));
        }
    }

    mod user_credentials_list_tests {
        use super::*;

        #[tokio::test]
        async fn test_credentials_list_success() {
            let mock_server = MockServer::start().await;

            let expected_credentials = vec![
                CredentialRepresentation {
                    id: Some("cred-1".to_string()),
                    credential_type: Some("password".to_string()),
                    user_label: Some("My Password".to_string()),
                    created_date: Some(1609459200000),
                    ..Default::default()
                },
                CredentialRepresentation {
                    id: Some("cred-2".to_string()),
                    credential_type: Some("otp".to_string()),
                    user_label: Some("Google Authenticator".to_string()),
                    ..Default::default()
                },
            ];

            Mock::given(method("GET"))
                .and(path("/admin/realms/master/users/123/credentials"))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .respond_with(ResponseTemplate::new(200).set_body_json(&expected_credentials))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserCredentialsListParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
            };

            let credentials = user_credentials_list(&client, TEST_TOKEN, &params)
                .await
                .expect("Failed to list credentials");
            assert_eq!(credentials.len(), 2);
            assert_eq!(credentials[0].id.as_deref(), Some("cred-1"));
            assert_eq!(credentials[0].credential_type.as_deref(), Some("password"));
            assert_eq!(credentials[1].credential_type.as_deref(), Some("otp"));
        }

        #[tokio::test]
        async fn test_credentials_list_empty() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path("/admin/realms/master/users/123/credentials"))
                .respond_with(
                    ResponseTemplate::new(200)
                        .set_body_json::<Vec<CredentialRepresentation>>(vec![]),
                )
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserCredentialsListParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
            };

            let credentials = user_credentials_list(&client, TEST_TOKEN, &params)
                .await
                .expect("Failed to list credentials");
            assert!(credentials.is_empty());
        }

        #[tokio::test]
        async fn test_credentials_list_user_not_found() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path("/admin/realms/master/users/999/credentials"))
                .respond_with(ResponseTemplate::new(404))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserCredentialsListParams {
                realm: "master".to_string(),
                user_id: "999".to_string(),
            };

            let result = user_credentials_list(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::NotFound)));
        }
    }

    mod user_credential_delete_tests {
        use super::*;

        #[tokio::test]
        async fn test_credential_delete_success() {
            let mock_server = MockServer::start().await;

            Mock::given(method("DELETE"))
                .and(path("/admin/realms/master/users/123/credentials/cred-456"))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .respond_with(ResponseTemplate::new(204))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserCredentialDeleteParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                credential_id: "cred-456".to_string(),
            };

            let result = user_credential_delete(&client, TEST_TOKEN, &params).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_credential_delete_not_found() {
            let mock_server = MockServer::start().await;

            Mock::given(method("DELETE"))
                .and(path(
                    "/admin/realms/master/users/123/credentials/nonexistent",
                ))
                .respond_with(ResponseTemplate::new(404))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserCredentialDeleteParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                credential_id: "nonexistent".to_string(),
            };

            let result = user_credential_delete(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::NotFound)));
        }
    }

    mod user_disable_credentials_tests {
        use super::*;

        #[tokio::test]
        async fn test_disable_credentials_success() {
            let mock_server = MockServer::start().await;

            Mock::given(method("PUT"))
                .and(path(
                    "/admin/realms/master/users/123/disable-credential-types",
                ))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .and(body_json(vec!["password"]))
                .respond_with(ResponseTemplate::new(204))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserDisableCredentialsParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                credential_types: vec!["password".to_string()],
            };

            let result = user_disable_credentials(&client, TEST_TOKEN, &params).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_disable_credentials_multiple_types() {
            let mock_server = MockServer::start().await;

            Mock::given(method("PUT"))
                .and(path(
                    "/admin/realms/master/users/123/disable-credential-types",
                ))
                .and(body_json(vec!["password", "otp"]))
                .respond_with(ResponseTemplate::new(204))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserDisableCredentialsParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                credential_types: vec!["password".to_string(), "otp".to_string()],
            };

            let result = user_disable_credentials(&client, TEST_TOKEN, &params).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_disable_credentials_user_not_found() {
            let mock_server = MockServer::start().await;

            Mock::given(method("PUT"))
                .and(path(
                    "/admin/realms/master/users/999/disable-credential-types",
                ))
                .respond_with(ResponseTemplate::new(404))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserDisableCredentialsParams {
                realm: "master".to_string(),
                user_id: "999".to_string(),
                credential_types: vec!["password".to_string()],
            };

            let result = user_disable_credentials(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::NotFound)));
        }
    }

    mod user_send_verify_email_tests {
        use super::*;

        #[tokio::test]
        async fn test_send_verify_email_success() {
            let mock_server = MockServer::start().await;

            Mock::given(method("PUT"))
                .and(path("/admin/realms/master/users/123/send-verify-email"))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .respond_with(ResponseTemplate::new(204))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserSendVerifyEmailParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                client_id: None,
                redirect_uri: None,
            };

            let result = user_send_verify_email(&client, TEST_TOKEN, &params).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_send_verify_email_with_client_id() {
            let mock_server = MockServer::start().await;

            Mock::given(method("PUT"))
                .and(path("/admin/realms/master/users/123/send-verify-email"))
                .and(query_param("client_id", "my-client"))
                .respond_with(ResponseTemplate::new(204))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserSendVerifyEmailParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                client_id: Some("my-client".to_string()),
                redirect_uri: None,
            };

            let result = user_send_verify_email(&client, TEST_TOKEN, &params).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_send_verify_email_user_not_found() {
            let mock_server = MockServer::start().await;

            Mock::given(method("PUT"))
                .and(path("/admin/realms/master/users/999/send-verify-email"))
                .respond_with(ResponseTemplate::new(404))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserSendVerifyEmailParams {
                realm: "master".to_string(),
                user_id: "999".to_string(),
                client_id: None,
                redirect_uri: None,
            };

            let result = user_send_verify_email(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::NotFound)));
        }
    }

    mod user_execute_actions_email_tests {
        use super::*;

        #[tokio::test]
        async fn test_execute_actions_email_success() {
            let mock_server = MockServer::start().await;

            Mock::given(method("PUT"))
                .and(path("/admin/realms/master/users/123/execute-actions-email"))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .and(body_json(vec!["UPDATE_PASSWORD"]))
                .respond_with(ResponseTemplate::new(204))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserExecuteActionsEmailParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                actions: vec!["UPDATE_PASSWORD".to_string()],
                client_id: None,
                redirect_uri: None,
                lifespan: None,
            };

            let result = user_execute_actions_email(&client, TEST_TOKEN, &params).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_execute_actions_email_multiple_actions() {
            let mock_server = MockServer::start().await;

            Mock::given(method("PUT"))
                .and(path("/admin/realms/master/users/123/execute-actions-email"))
                .and(body_json(vec!["UPDATE_PASSWORD", "VERIFY_EMAIL"]))
                .respond_with(ResponseTemplate::new(204))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserExecuteActionsEmailParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                actions: vec!["UPDATE_PASSWORD".to_string(), "VERIFY_EMAIL".to_string()],
                client_id: None,
                redirect_uri: None,
                lifespan: None,
            };

            let result = user_execute_actions_email(&client, TEST_TOKEN, &params).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_execute_actions_email_with_lifespan() {
            let mock_server = MockServer::start().await;

            Mock::given(method("PUT"))
                .and(path("/admin/realms/master/users/123/execute-actions-email"))
                .and(query_param("lifespan", "3600"))
                .respond_with(ResponseTemplate::new(204))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserExecuteActionsEmailParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                actions: vec!["UPDATE_PASSWORD".to_string()],
                client_id: None,
                redirect_uri: None,
                lifespan: Some(3600),
            };

            let result = user_execute_actions_email(&client, TEST_TOKEN, &params).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_execute_actions_email_user_not_found() {
            let mock_server = MockServer::start().await;

            Mock::given(method("PUT"))
                .and(path("/admin/realms/master/users/999/execute-actions-email"))
                .respond_with(ResponseTemplate::new(404))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserExecuteActionsEmailParams {
                realm: "master".to_string(),
                user_id: "999".to_string(),
                actions: vec!["UPDATE_PASSWORD".to_string()],
                client_id: None,
                redirect_uri: None,
                lifespan: None,
            };

            let result = user_execute_actions_email(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::NotFound)));
        }
    }
}
