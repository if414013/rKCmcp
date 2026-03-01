//! Keycloak Authentication Required Actions API module.
//!
//! Provides operations for managing required actions in Keycloak realms.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::api::{ApiError, KeycloakClient};

/// Represents a required action provider in Keycloak.
///
/// Required actions are actions that a user must perform during authentication
/// (e.g., update password, configure OTP, verify email).
#[derive(Debug, Clone, Serialize, Deserialize, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RequiredActionProviderRepresentation {
    /// The unique alias for this required action
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,

    /// Display name for this required action
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The provider ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_id: Option<String>,

    /// Whether this required action is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// Whether this is a default required action
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_action: Option<bool>,

    /// Priority order for this required action
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,

    /// Configuration for this required action
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<HashMap<String, String>>,
}

/// Parameters for listing required actions.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AuthRequiredActionsListParams {
    pub realm: String,
}

impl AuthRequiredActionsListParams {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.realm.trim().is_empty() {
            return Err(ApiError::BadRequest("realm is required".to_string()));
        }
        Ok(())
    }
}

/// Parameters for getting a single required action by alias.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AuthRequiredActionGetParams {
    pub realm: String,
    pub alias: String,
}

impl AuthRequiredActionGetParams {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.realm.trim().is_empty() {
            return Err(ApiError::BadRequest("realm is required".to_string()));
        }
        if self.alias.trim().is_empty() {
            return Err(ApiError::BadRequest("alias is required".to_string()));
        }
        Ok(())
    }
}

/// Parameters for updating a required action.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AuthRequiredActionUpdateParams {
    pub realm: String,
    pub alias: String,
    pub action: RequiredActionProviderRepresentation,
}

impl AuthRequiredActionUpdateParams {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.realm.trim().is_empty() {
            return Err(ApiError::BadRequest("realm is required".to_string()));
        }
        if self.alias.trim().is_empty() {
            return Err(ApiError::BadRequest("alias is required".to_string()));
        }
        Ok(())
    }
}

/// Parameters for raising the priority of a required action.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AuthRequiredActionRaisePriorityParams {
    pub realm: String,
    pub alias: String,
}

impl AuthRequiredActionRaisePriorityParams {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.realm.trim().is_empty() {
            return Err(ApiError::BadRequest("realm is required".to_string()));
        }
        if self.alias.trim().is_empty() {
            return Err(ApiError::BadRequest("alias is required".to_string()));
        }
        Ok(())
    }
}

/// Parameters for lowering the priority of a required action.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AuthRequiredActionLowerPriorityParams {
    pub realm: String,
    pub alias: String,
}

impl AuthRequiredActionLowerPriorityParams {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.realm.trim().is_empty() {
            return Err(ApiError::BadRequest("realm is required".to_string()));
        }
        if self.alias.trim().is_empty() {
            return Err(ApiError::BadRequest("alias is required".to_string()));
        }
        Ok(())
    }
}

/// Parameters for listing unregistered required actions.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AuthUnregisteredRequiredActionsListParams {
    pub realm: String,
}

impl AuthUnregisteredRequiredActionsListParams {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.realm.trim().is_empty() {
            return Err(ApiError::BadRequest("realm is required".to_string()));
        }
        Ok(())
    }
}

/// Parameters for registering a required action.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AuthRequiredActionRegisterParams {
    pub realm: String,
    pub provider_id: String,
    pub name: String,
}

impl AuthRequiredActionRegisterParams {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.realm.trim().is_empty() {
            return Err(ApiError::BadRequest("realm is required".to_string()));
        }
        if self.provider_id.trim().is_empty() {
            return Err(ApiError::BadRequest("provider_id is required".to_string()));
        }
        if self.name.trim().is_empty() {
            return Err(ApiError::BadRequest("name is required".to_string()));
        }
        Ok(())
    }
}

/// Request body for registering a required action.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct RegisterRequiredActionRequest {
    provider_id: String,
    name: String,
}

/// List all required actions in a realm.
///
/// GET /admin/realms/{realm}/authentication/required-actions
pub async fn auth_required_actions_list(
    client: &KeycloakClient,
    token: &str,
    params: &AuthRequiredActionsListParams,
) -> Result<Vec<RequiredActionProviderRepresentation>, ApiError> {
    params.validate()?;

    let path = format!(
        "/admin/realms/{}/authentication/required-actions",
        urlencoding::encode(&params.realm)
    );

    client.get(&path, token).await
}

/// Get a single required action by alias.
///
/// GET /admin/realms/{realm}/authentication/required-actions/{alias}
pub async fn auth_required_action_get(
    client: &KeycloakClient,
    token: &str,
    params: &AuthRequiredActionGetParams,
) -> Result<RequiredActionProviderRepresentation, ApiError> {
    params.validate()?;

    let path = format!(
        "/admin/realms/{}/authentication/required-actions/{}",
        urlencoding::encode(&params.realm),
        urlencoding::encode(&params.alias)
    );

    client.get(&path, token).await
}

/// Update a required action.
///
/// PUT /admin/realms/{realm}/authentication/required-actions/{alias}
pub async fn auth_required_action_update(
    client: &KeycloakClient,
    token: &str,
    params: &AuthRequiredActionUpdateParams,
) -> Result<(), ApiError> {
    params.validate()?;

    let path = format!(
        "/admin/realms/{}/authentication/required-actions/{}",
        urlencoding::encode(&params.realm),
        urlencoding::encode(&params.alias)
    );

    client.put(&path, token, &params.action).await
}

/// Raise the priority of a required action.
///
/// POST /admin/realms/{realm}/authentication/required-actions/{alias}/raise-priority
pub async fn auth_required_action_raise_priority(
    client: &KeycloakClient,
    token: &str,
    params: &AuthRequiredActionRaisePriorityParams,
) -> Result<(), ApiError> {
    params.validate()?;

    let path = format!(
        "/admin/realms/{}/authentication/required-actions/{}/raise-priority",
        urlencoding::encode(&params.realm),
        urlencoding::encode(&params.alias)
    );

    client
        .post_no_response(&path, token, &serde_json::Value::Null)
        .await
}

/// Lower the priority of a required action.
///
/// POST /admin/realms/{realm}/authentication/required-actions/{alias}/lower-priority
pub async fn auth_required_action_lower_priority(
    client: &KeycloakClient,
    token: &str,
    params: &AuthRequiredActionLowerPriorityParams,
) -> Result<(), ApiError> {
    params.validate()?;

    let path = format!(
        "/admin/realms/{}/authentication/required-actions/{}/lower-priority",
        urlencoding::encode(&params.realm),
        urlencoding::encode(&params.alias)
    );

    client
        .post_no_response(&path, token, &serde_json::Value::Null)
        .await
}

/// List all unregistered required actions.
///
/// GET /admin/realms/{realm}/authentication/unregistered-required-actions
pub async fn auth_unregistered_required_actions_list(
    client: &KeycloakClient,
    token: &str,
    params: &AuthUnregisteredRequiredActionsListParams,
) -> Result<Vec<HashMap<String, serde_json::Value>>, ApiError> {
    params.validate()?;

    let path = format!(
        "/admin/realms/{}/authentication/unregistered-required-actions",
        urlencoding::encode(&params.realm)
    );

    client.get(&path, token).await
}

/// Register a required action.
///
/// POST /admin/realms/{realm}/authentication/register-required-action
pub async fn auth_required_action_register(
    client: &KeycloakClient,
    token: &str,
    params: &AuthRequiredActionRegisterParams,
) -> Result<(), ApiError> {
    params.validate()?;

    let path = format!(
        "/admin/realms/{}/authentication/register-required-action",
        urlencoding::encode(&params.realm)
    );

    let body = RegisterRequiredActionRequest {
        provider_id: params.provider_id.clone(),
        name: params.name.clone(),
    };

    client.post_no_response(&path, token, &body).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{body_json, header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    const TEST_TOKEN: &str = "test-bearer-token";

    fn sample_required_action() -> RequiredActionProviderRepresentation {
        RequiredActionProviderRepresentation {
            alias: Some("CONFIGURE_TOTP".to_string()),
            name: Some("Configure OTP".to_string()),
            provider_id: Some("CONFIGURE_TOTP".to_string()),
            enabled: Some(true),
            default_action: Some(false),
            priority: Some(10),
            config: Some(HashMap::new()),
        }
    }

    mod validation_tests {
        use super::*;

        #[test]
        fn test_auth_required_actions_list_params_valid() {
            let params = AuthRequiredActionsListParams {
                realm: "master".to_string(),
            };
            assert!(params.validate().is_ok());
        }

        #[test]
        fn test_auth_required_actions_list_params_empty_realm() {
            let params = AuthRequiredActionsListParams {
                realm: "".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_auth_required_action_get_params_valid() {
            let params = AuthRequiredActionGetParams {
                realm: "master".to_string(),
                alias: "CONFIGURE_TOTP".to_string(),
            };
            assert!(params.validate().is_ok());
        }

        #[test]
        fn test_auth_required_action_get_params_empty_realm() {
            let params = AuthRequiredActionGetParams {
                realm: "".to_string(),
                alias: "CONFIGURE_TOTP".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_auth_required_action_get_params_empty_alias() {
            let params = AuthRequiredActionGetParams {
                realm: "master".to_string(),
                alias: "".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_auth_required_action_update_params_valid() {
            let params = AuthRequiredActionUpdateParams {
                realm: "master".to_string(),
                alias: "CONFIGURE_TOTP".to_string(),
                action: sample_required_action(),
            };
            assert!(params.validate().is_ok());
        }

        #[test]
        fn test_auth_required_action_update_params_empty_realm() {
            let params = AuthRequiredActionUpdateParams {
                realm: "".to_string(),
                alias: "CONFIGURE_TOTP".to_string(),
                action: sample_required_action(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_auth_required_action_update_params_empty_alias() {
            let params = AuthRequiredActionUpdateParams {
                realm: "master".to_string(),
                alias: "".to_string(),
                action: sample_required_action(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_auth_required_action_raise_priority_params_valid() {
            let params = AuthRequiredActionRaisePriorityParams {
                realm: "master".to_string(),
                alias: "CONFIGURE_TOTP".to_string(),
            };
            assert!(params.validate().is_ok());
        }

        #[test]
        fn test_auth_required_action_raise_priority_params_empty_realm() {
            let params = AuthRequiredActionRaisePriorityParams {
                realm: "".to_string(),
                alias: "CONFIGURE_TOTP".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_auth_required_action_raise_priority_params_empty_alias() {
            let params = AuthRequiredActionRaisePriorityParams {
                realm: "master".to_string(),
                alias: "".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_auth_required_action_lower_priority_params_valid() {
            let params = AuthRequiredActionLowerPriorityParams {
                realm: "master".to_string(),
                alias: "CONFIGURE_TOTP".to_string(),
            };
            assert!(params.validate().is_ok());
        }

        #[test]
        fn test_auth_required_action_lower_priority_params_empty_realm() {
            let params = AuthRequiredActionLowerPriorityParams {
                realm: "".to_string(),
                alias: "CONFIGURE_TOTP".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_auth_required_action_lower_priority_params_empty_alias() {
            let params = AuthRequiredActionLowerPriorityParams {
                realm: "master".to_string(),
                alias: "".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_auth_unregistered_required_actions_list_params_valid() {
            let params = AuthUnregisteredRequiredActionsListParams {
                realm: "master".to_string(),
            };
            assert!(params.validate().is_ok());
        }

        #[test]
        fn test_auth_unregistered_required_actions_list_params_empty_realm() {
            let params = AuthUnregisteredRequiredActionsListParams {
                realm: "".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_auth_required_action_register_params_valid() {
            let params = AuthRequiredActionRegisterParams {
                realm: "master".to_string(),
                provider_id: "webauthn-register".to_string(),
                name: "Webauthn Register".to_string(),
            };
            assert!(params.validate().is_ok());
        }

        #[test]
        fn test_auth_required_action_register_params_empty_realm() {
            let params = AuthRequiredActionRegisterParams {
                realm: "".to_string(),
                provider_id: "webauthn-register".to_string(),
                name: "Webauthn Register".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_auth_required_action_register_params_empty_provider_id() {
            let params = AuthRequiredActionRegisterParams {
                realm: "master".to_string(),
                provider_id: "".to_string(),
                name: "Webauthn Register".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_auth_required_action_register_params_empty_name() {
            let params = AuthRequiredActionRegisterParams {
                realm: "master".to_string(),
                provider_id: "webauthn-register".to_string(),
                name: "".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }
    }

    mod auth_required_actions_list_tests {
        use super::*;

        #[tokio::test]
        async fn test_auth_required_actions_list_success() {
            let mock_server = MockServer::start().await;

            let expected_actions = vec![sample_required_action()];

            Mock::given(method("GET"))
                .and(path("/admin/realms/master/authentication/required-actions"))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .respond_with(ResponseTemplate::new(200).set_body_json(&expected_actions))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthRequiredActionsListParams {
                realm: "master".to_string(),
            };

            let actions = auth_required_actions_list(&client, TEST_TOKEN, &params)
                .await
                .unwrap();
            assert_eq!(actions.len(), 1);
            assert_eq!(actions[0].alias, Some("CONFIGURE_TOTP".to_string()));
        }

        #[tokio::test]
        async fn test_auth_required_actions_list_empty() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path("/admin/realms/master/authentication/required-actions"))
                .respond_with(
                    ResponseTemplate::new(200)
                        .set_body_json(Vec::<RequiredActionProviderRepresentation>::new()),
                )
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthRequiredActionsListParams {
                realm: "master".to_string(),
            };

            let actions = auth_required_actions_list(&client, TEST_TOKEN, &params)
                .await
                .unwrap();
            assert!(actions.is_empty());
        }

        #[tokio::test]
        async fn test_auth_required_actions_list_unauthorized() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path("/admin/realms/master/authentication/required-actions"))
                .respond_with(ResponseTemplate::new(401))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthRequiredActionsListParams {
                realm: "master".to_string(),
            };

            let result = auth_required_actions_list(&client, "invalid", &params).await;
            assert!(matches!(result, Err(ApiError::Unauthorized)));
        }
    }

    mod auth_required_action_get_tests {
        use super::*;

        #[tokio::test]
        async fn test_auth_required_action_get_success() {
            let mock_server = MockServer::start().await;

            let expected_action = sample_required_action();

            Mock::given(method("GET"))
                .and(path(
                    "/admin/realms/master/authentication/required-actions/CONFIGURE_TOTP",
                ))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .respond_with(ResponseTemplate::new(200).set_body_json(&expected_action))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthRequiredActionGetParams {
                realm: "master".to_string(),
                alias: "CONFIGURE_TOTP".to_string(),
            };

            let action = auth_required_action_get(&client, TEST_TOKEN, &params)
                .await
                .unwrap();
            assert_eq!(action.alias, Some("CONFIGURE_TOTP".to_string()));
            assert_eq!(action.name, Some("Configure OTP".to_string()));
        }

        #[tokio::test]
        async fn test_auth_required_action_get_not_found() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path(
                    "/admin/realms/master/authentication/required-actions/nonexistent",
                ))
                .respond_with(ResponseTemplate::new(404))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthRequiredActionGetParams {
                realm: "master".to_string(),
                alias: "nonexistent".to_string(),
            };

            let result = auth_required_action_get(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::NotFound)));
        }

        #[tokio::test]
        async fn test_auth_required_action_get_special_characters() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path(
                    "/admin/realms/master/authentication/required-actions/action%2Fwith%2Fslash",
                ))
                .respond_with(ResponseTemplate::new(200).set_body_json(sample_required_action()))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthRequiredActionGetParams {
                realm: "master".to_string(),
                alias: "action/with/slash".to_string(),
            };

            let result = auth_required_action_get(&client, TEST_TOKEN, &params).await;
            assert!(result.is_ok());
        }
    }

    mod auth_required_action_update_tests {
        use super::*;

        #[tokio::test]
        async fn test_auth_required_action_update_success() {
            let mock_server = MockServer::start().await;

            let updated_action = RequiredActionProviderRepresentation {
                alias: Some("CONFIGURE_TOTP".to_string()),
                name: Some("Configure OTP".to_string()),
                provider_id: Some("CONFIGURE_TOTP".to_string()),
                enabled: Some(false),
                default_action: Some(true),
                priority: Some(20),
                config: Some(HashMap::new()),
            };

            Mock::given(method("PUT"))
                .and(path(
                    "/admin/realms/master/authentication/required-actions/CONFIGURE_TOTP",
                ))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .and(body_json(&updated_action))
                .respond_with(ResponseTemplate::new(204))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthRequiredActionUpdateParams {
                realm: "master".to_string(),
                alias: "CONFIGURE_TOTP".to_string(),
                action: updated_action,
            };

            let result = auth_required_action_update(&client, TEST_TOKEN, &params).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_auth_required_action_update_not_found() {
            let mock_server = MockServer::start().await;

            Mock::given(method("PUT"))
                .and(path(
                    "/admin/realms/master/authentication/required-actions/nonexistent",
                ))
                .respond_with(ResponseTemplate::new(404))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthRequiredActionUpdateParams {
                realm: "master".to_string(),
                alias: "nonexistent".to_string(),
                action: sample_required_action(),
            };

            let result = auth_required_action_update(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::NotFound)));
        }
    }

    mod auth_required_action_raise_priority_tests {
        use super::*;

        #[tokio::test]
        async fn test_auth_required_action_raise_priority_success() {
            let mock_server = MockServer::start().await;

            Mock::given(method("POST"))
                .and(path(
                    "/admin/realms/master/authentication/required-actions/CONFIGURE_TOTP/raise-priority",
                ))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .and(body_json(serde_json::Value::Null))
                .respond_with(ResponseTemplate::new(204))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthRequiredActionRaisePriorityParams {
                realm: "master".to_string(),
                alias: "CONFIGURE_TOTP".to_string(),
            };

            let result = auth_required_action_raise_priority(&client, TEST_TOKEN, &params).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_auth_required_action_raise_priority_not_found() {
            let mock_server = MockServer::start().await;

            Mock::given(method("POST"))
                .and(path(
                    "/admin/realms/master/authentication/required-actions/nonexistent/raise-priority",
                ))
                .respond_with(ResponseTemplate::new(404))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthRequiredActionRaisePriorityParams {
                realm: "master".to_string(),
                alias: "nonexistent".to_string(),
            };

            let result = auth_required_action_raise_priority(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::NotFound)));
        }

        #[tokio::test]
        async fn test_auth_required_action_raise_priority_unauthorized() {
            let mock_server = MockServer::start().await;

            Mock::given(method("POST"))
                .and(path(
                    "/admin/realms/master/authentication/required-actions/CONFIGURE_TOTP/raise-priority",
                ))
                .respond_with(ResponseTemplate::new(401))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthRequiredActionRaisePriorityParams {
                realm: "master".to_string(),
                alias: "CONFIGURE_TOTP".to_string(),
            };

            let result = auth_required_action_raise_priority(&client, "invalid", &params).await;
            assert!(matches!(result, Err(ApiError::Unauthorized)));
        }
    }

    mod auth_required_action_lower_priority_tests {
        use super::*;

        #[tokio::test]
        async fn test_auth_required_action_lower_priority_success() {
            let mock_server = MockServer::start().await;

            Mock::given(method("POST"))
                .and(path(
                    "/admin/realms/master/authentication/required-actions/CONFIGURE_TOTP/lower-priority",
                ))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .and(body_json(serde_json::Value::Null))
                .respond_with(ResponseTemplate::new(204))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthRequiredActionLowerPriorityParams {
                realm: "master".to_string(),
                alias: "CONFIGURE_TOTP".to_string(),
            };

            let result = auth_required_action_lower_priority(&client, TEST_TOKEN, &params).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_auth_required_action_lower_priority_not_found() {
            let mock_server = MockServer::start().await;

            Mock::given(method("POST"))
                .and(path(
                    "/admin/realms/master/authentication/required-actions/nonexistent/lower-priority",
                ))
                .respond_with(ResponseTemplate::new(404))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthRequiredActionLowerPriorityParams {
                realm: "master".to_string(),
                alias: "nonexistent".to_string(),
            };

            let result = auth_required_action_lower_priority(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::NotFound)));
        }

        #[tokio::test]
        async fn test_auth_required_action_lower_priority_unauthorized() {
            let mock_server = MockServer::start().await;

            Mock::given(method("POST"))
                .and(path(
                    "/admin/realms/master/authentication/required-actions/CONFIGURE_TOTP/lower-priority",
                ))
                .respond_with(ResponseTemplate::new(401))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthRequiredActionLowerPriorityParams {
                realm: "master".to_string(),
                alias: "CONFIGURE_TOTP".to_string(),
            };

            let result = auth_required_action_lower_priority(&client, "invalid", &params).await;
            assert!(matches!(result, Err(ApiError::Unauthorized)));
        }
    }

    mod auth_unregistered_required_actions_list_tests {
        use super::*;

        #[tokio::test]
        async fn test_auth_unregistered_required_actions_list_success() {
            let mock_server = MockServer::start().await;

            let expected_actions = vec![serde_json::json!({
                "providerId": "webauthn-register",
                "name": "Webauthn Register"
            })];

            Mock::given(method("GET"))
                .and(path(
                    "/admin/realms/master/authentication/unregistered-required-actions",
                ))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .respond_with(ResponseTemplate::new(200).set_body_json(&expected_actions))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthUnregisteredRequiredActionsListParams {
                realm: "master".to_string(),
            };

            let actions = auth_unregistered_required_actions_list(&client, TEST_TOKEN, &params)
                .await
                .unwrap();
            assert_eq!(actions.len(), 1);
        }

        #[tokio::test]
        async fn test_auth_unregistered_required_actions_list_empty() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path(
                    "/admin/realms/master/authentication/unregistered-required-actions",
                ))
                .respond_with(
                    ResponseTemplate::new(200)
                        .set_body_json(Vec::<HashMap<String, serde_json::Value>>::new()),
                )
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthUnregisteredRequiredActionsListParams {
                realm: "master".to_string(),
            };

            let actions = auth_unregistered_required_actions_list(&client, TEST_TOKEN, &params)
                .await
                .unwrap();
            assert!(actions.is_empty());
        }

        #[tokio::test]
        async fn test_auth_unregistered_required_actions_list_unauthorized() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path(
                    "/admin/realms/master/authentication/unregistered-required-actions",
                ))
                .respond_with(ResponseTemplate::new(401))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthUnregisteredRequiredActionsListParams {
                realm: "master".to_string(),
            };

            let result = auth_unregistered_required_actions_list(&client, "invalid", &params).await;
            assert!(matches!(result, Err(ApiError::Unauthorized)));
        }
    }

    mod auth_required_action_register_tests {
        use super::*;

        #[tokio::test]
        async fn test_auth_required_action_register_success() {
            let mock_server = MockServer::start().await;

            let register_request = RegisterRequiredActionRequest {
                provider_id: "webauthn-register".to_string(),
                name: "Webauthn Register".to_string(),
            };

            Mock::given(method("POST"))
                .and(path(
                    "/admin/realms/master/authentication/register-required-action",
                ))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .and(body_json(&register_request))
                .respond_with(ResponseTemplate::new(201))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthRequiredActionRegisterParams {
                realm: "master".to_string(),
                provider_id: "webauthn-register".to_string(),
                name: "Webauthn Register".to_string(),
            };

            let result = auth_required_action_register(&client, TEST_TOKEN, &params).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_auth_required_action_register_not_found() {
            let mock_server = MockServer::start().await;

            Mock::given(method("POST"))
                .and(path(
                    "/admin/realms/master/authentication/register-required-action",
                ))
                .respond_with(ResponseTemplate::new(404))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthRequiredActionRegisterParams {
                realm: "master".to_string(),
                provider_id: "nonexistent-provider".to_string(),
                name: "Nonexistent".to_string(),
            };

            let result = auth_required_action_register(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::NotFound)));
        }

        #[tokio::test]
        async fn test_auth_required_action_register_unauthorized() {
            let mock_server = MockServer::start().await;

            Mock::given(method("POST"))
                .and(path(
                    "/admin/realms/master/authentication/register-required-action",
                ))
                .respond_with(ResponseTemplate::new(401))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthRequiredActionRegisterParams {
                realm: "master".to_string(),
                provider_id: "webauthn-register".to_string(),
                name: "Webauthn Register".to_string(),
            };

            let result = auth_required_action_register(&client, "invalid", &params).await;
            assert!(matches!(result, Err(ApiError::Unauthorized)));
        }
    }
}
