//! Keycloak Authentication Executions API.
//!
//! Provides operations for managing authentication executions within flows.

use serde::{Deserialize, Serialize};

use crate::api::{ApiError, KeycloakClient};

use super::{AuthenticationExecutionRepresentation, AuthenticatorConfigRepresentation};

/// Parameters for getting an authentication execution by ID.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AuthExecutionGetParams {
    pub realm: String,
    pub execution_id: String,
}

impl AuthExecutionGetParams {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.realm.trim().is_empty() {
            return Err(ApiError::BadRequest("realm is required".to_string()));
        }
        if self.execution_id.trim().is_empty() {
            return Err(ApiError::BadRequest("execution_id is required".to_string()));
        }
        Ok(())
    }
}

/// Parameters for creating a new execution within a flow.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AuthExecutionCreateParams {
    pub realm: String,
    pub flow_alias: String,
    pub provider: String,
}

impl AuthExecutionCreateParams {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.realm.trim().is_empty() {
            return Err(ApiError::BadRequest("realm is required".to_string()));
        }
        if self.flow_alias.trim().is_empty() {
            return Err(ApiError::BadRequest("flow_alias is required".to_string()));
        }
        if self.provider.trim().is_empty() {
            return Err(ApiError::BadRequest("provider is required".to_string()));
        }
        Ok(())
    }
}

/// Parameters for deleting an authentication execution.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AuthExecutionDeleteParams {
    pub realm: String,
    pub execution_id: String,
}

impl AuthExecutionDeleteParams {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.realm.trim().is_empty() {
            return Err(ApiError::BadRequest("realm is required".to_string()));
        }
        if self.execution_id.trim().is_empty() {
            return Err(ApiError::BadRequest("execution_id is required".to_string()));
        }
        Ok(())
    }
}

/// Parameters for raising or lowering execution priority.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AuthExecutionPriorityParams {
    pub realm: String,
    pub execution_id: String,
}

impl AuthExecutionPriorityParams {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.realm.trim().is_empty() {
            return Err(ApiError::BadRequest("realm is required".to_string()));
        }
        if self.execution_id.trim().is_empty() {
            return Err(ApiError::BadRequest("execution_id is required".to_string()));
        }
        Ok(())
    }
}

/// Parameters for getting an authenticator config.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AuthExecutionConfigGetParams {
    pub realm: String,
    pub config_id: String,
}

impl AuthExecutionConfigGetParams {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.realm.trim().is_empty() {
            return Err(ApiError::BadRequest("realm is required".to_string()));
        }
        if self.config_id.trim().is_empty() {
            return Err(ApiError::BadRequest("config_id is required".to_string()));
        }
        Ok(())
    }
}

/// Parameters for updating an authenticator config.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AuthExecutionConfigUpdateParams {
    pub realm: String,
    pub config_id: String,
    pub config: AuthenticatorConfigRepresentation,
}

impl AuthExecutionConfigUpdateParams {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.realm.trim().is_empty() {
            return Err(ApiError::BadRequest("realm is required".to_string()));
        }
        if self.config_id.trim().is_empty() {
            return Err(ApiError::BadRequest("config_id is required".to_string()));
        }
        Ok(())
    }
}

/// Request body for creating an execution.
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ExecutionCreateRequest {
    pub provider: String,
}

/// Get a single authentication execution by ID.
///
/// GET /admin/realms/{realm}/authentication/executions/{executionId}
pub async fn auth_execution_get(
    client: &KeycloakClient,
    token: &str,
    params: &AuthExecutionGetParams,
) -> Result<AuthenticationExecutionRepresentation, ApiError> {
    params.validate()?;

    let path = format!(
        "/admin/realms/{}/authentication/executions/{}",
        urlencoding::encode(&params.realm),
        &params.execution_id
    );

    client.get(&path, token).await
}

/// Create a new execution within an authentication flow.
///
/// POST /admin/realms/{realm}/authentication/flows/{flowAlias}/executions/execution
pub async fn auth_execution_create(
    client: &KeycloakClient,
    token: &str,
    params: &AuthExecutionCreateParams,
) -> Result<(), ApiError> {
    params.validate()?;

    let path = format!(
        "/admin/realms/{}/authentication/flows/{}/executions/execution",
        urlencoding::encode(&params.realm),
        urlencoding::encode(&params.flow_alias)
    );

    let body = ExecutionCreateRequest {
        provider: params.provider.clone(),
    };

    client.post_no_response(&path, token, &body).await
}

/// Delete an authentication execution.
///
/// DELETE /admin/realms/{realm}/authentication/executions/{executionId}
pub async fn auth_execution_delete(
    client: &KeycloakClient,
    token: &str,
    params: &AuthExecutionDeleteParams,
) -> Result<(), ApiError> {
    params.validate()?;

    let path = format!(
        "/admin/realms/{}/authentication/executions/{}",
        urlencoding::encode(&params.realm),
        &params.execution_id
    );

    client.delete(&path, token).await
}

/// Raise the priority of an authentication execution.
///
/// POST /admin/realms/{realm}/authentication/executions/{executionId}/raise-priority
///
/// Moves the execution up in the flow's execution order.
pub async fn auth_execution_raise_priority(
    client: &KeycloakClient,
    token: &str,
    params: &AuthExecutionPriorityParams,
) -> Result<(), ApiError> {
    params.validate()?;

    let path = format!(
        "/admin/realms/{}/authentication/executions/{}/raise-priority",
        urlencoding::encode(&params.realm),
        &params.execution_id
    );

    client
        .post_no_response(&path, token, &serde_json::Value::Null)
        .await
}

/// Lower the priority of an authentication execution.
///
/// POST /admin/realms/{realm}/authentication/executions/{executionId}/lower-priority
///
/// Moves the execution down in the flow's execution order.
pub async fn auth_execution_lower_priority(
    client: &KeycloakClient,
    token: &str,
    params: &AuthExecutionPriorityParams,
) -> Result<(), ApiError> {
    params.validate()?;

    let path = format!(
        "/admin/realms/{}/authentication/executions/{}/lower-priority",
        urlencoding::encode(&params.realm),
        &params.execution_id
    );

    client
        .post_no_response(&path, token, &serde_json::Value::Null)
        .await
}

/// Get an authenticator config by ID.
///
/// GET /admin/realms/{realm}/authentication/config/{id}
pub async fn auth_execution_config_get(
    client: &KeycloakClient,
    token: &str,
    params: &AuthExecutionConfigGetParams,
) -> Result<AuthenticatorConfigRepresentation, ApiError> {
    params.validate()?;

    let path = format!(
        "/admin/realms/{}/authentication/config/{}",
        urlencoding::encode(&params.realm),
        &params.config_id
    );

    client.get(&path, token).await
}

/// Update an authenticator config.
///
/// PUT /admin/realms/{realm}/authentication/config/{id}
pub async fn auth_execution_config_update(
    client: &KeycloakClient,
    token: &str,
    params: &AuthExecutionConfigUpdateParams,
) -> Result<(), ApiError> {
    params.validate()?;

    let path = format!(
        "/admin/realms/{}/authentication/config/{}",
        urlencoding::encode(&params.realm),
        &params.config_id
    );

    client.put(&path, token, &params.config).await
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use wiremock::matchers::{body_json, header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    const TEST_TOKEN: &str = "test-bearer-token";

    fn sample_execution() -> AuthenticationExecutionRepresentation {
        AuthenticationExecutionRepresentation {
            id: Some("exec-uuid-123".to_string()),
            authenticator: Some("auth-username-password-form".to_string()),
            authenticator_flow: Some(false),
            requirement: Some("REQUIRED".to_string()),
            priority: Some(10),
            parent_flow: Some("parent-flow-id".to_string()),
            authenticator_config: None,
            flow_id: None,
        }
    }

    fn sample_config() -> AuthenticatorConfigRepresentation {
        let mut config_map = HashMap::new();
        config_map.insert("defaultProvider".to_string(), "otp".to_string());

        AuthenticatorConfigRepresentation {
            id: Some("config-uuid-123".to_string()),
            alias: Some("otp-config".to_string()),
            config: Some(config_map),
        }
    }

    mod validation_tests {
        use super::*;

        #[test]
        fn test_auth_execution_get_params_valid() {
            let params = AuthExecutionGetParams {
                realm: "master".to_string(),
                execution_id: "exec-uuid-123".to_string(),
            };
            assert!(params.validate().is_ok());
        }

        #[test]
        fn test_auth_execution_get_params_empty_realm() {
            let params = AuthExecutionGetParams {
                realm: "".to_string(),
                execution_id: "exec-uuid-123".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_auth_execution_get_params_empty_execution_id() {
            let params = AuthExecutionGetParams {
                realm: "master".to_string(),
                execution_id: "".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_auth_execution_get_params_whitespace_realm() {
            let params = AuthExecutionGetParams {
                realm: "   ".to_string(),
                execution_id: "exec-uuid-123".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_auth_execution_create_params_valid() {
            let params = AuthExecutionCreateParams {
                realm: "master".to_string(),
                flow_alias: "browser".to_string(),
                provider: "auth-otp-form".to_string(),
            };
            assert!(params.validate().is_ok());
        }

        #[test]
        fn test_auth_execution_create_params_empty_realm() {
            let params = AuthExecutionCreateParams {
                realm: "".to_string(),
                flow_alias: "browser".to_string(),
                provider: "auth-otp-form".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_auth_execution_create_params_empty_flow_alias() {
            let params = AuthExecutionCreateParams {
                realm: "master".to_string(),
                flow_alias: "".to_string(),
                provider: "auth-otp-form".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_auth_execution_create_params_empty_provider() {
            let params = AuthExecutionCreateParams {
                realm: "master".to_string(),
                flow_alias: "browser".to_string(),
                provider: "".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_auth_execution_delete_params_valid() {
            let params = AuthExecutionDeleteParams {
                realm: "master".to_string(),
                execution_id: "exec-uuid-123".to_string(),
            };
            assert!(params.validate().is_ok());
        }

        #[test]
        fn test_auth_execution_delete_params_empty_realm() {
            let params = AuthExecutionDeleteParams {
                realm: "".to_string(),
                execution_id: "exec-uuid-123".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_auth_execution_delete_params_empty_execution_id() {
            let params = AuthExecutionDeleteParams {
                realm: "master".to_string(),
                execution_id: "".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_auth_execution_priority_params_valid() {
            let params = AuthExecutionPriorityParams {
                realm: "master".to_string(),
                execution_id: "exec-uuid-123".to_string(),
            };
            assert!(params.validate().is_ok());
        }

        #[test]
        fn test_auth_execution_priority_params_empty_realm() {
            let params = AuthExecutionPriorityParams {
                realm: "".to_string(),
                execution_id: "exec-uuid-123".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_auth_execution_priority_params_empty_execution_id() {
            let params = AuthExecutionPriorityParams {
                realm: "master".to_string(),
                execution_id: "".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_auth_execution_config_get_params_valid() {
            let params = AuthExecutionConfigGetParams {
                realm: "master".to_string(),
                config_id: "config-uuid-123".to_string(),
            };
            assert!(params.validate().is_ok());
        }

        #[test]
        fn test_auth_execution_config_get_params_empty_realm() {
            let params = AuthExecutionConfigGetParams {
                realm: "".to_string(),
                config_id: "config-uuid-123".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_auth_execution_config_get_params_empty_config_id() {
            let params = AuthExecutionConfigGetParams {
                realm: "master".to_string(),
                config_id: "".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_auth_execution_config_update_params_valid() {
            let params = AuthExecutionConfigUpdateParams {
                realm: "master".to_string(),
                config_id: "config-uuid-123".to_string(),
                config: sample_config(),
            };
            assert!(params.validate().is_ok());
        }

        #[test]
        fn test_auth_execution_config_update_params_empty_realm() {
            let params = AuthExecutionConfigUpdateParams {
                realm: "".to_string(),
                config_id: "config-uuid-123".to_string(),
                config: sample_config(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_auth_execution_config_update_params_empty_config_id() {
            let params = AuthExecutionConfigUpdateParams {
                realm: "master".to_string(),
                config_id: "".to_string(),
                config: sample_config(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }
    }

    mod auth_execution_get_tests {
        use super::*;

        #[tokio::test]
        async fn test_auth_execution_get_success() {
            let mock_server = MockServer::start().await;

            let expected = sample_execution();

            Mock::given(method("GET"))
                .and(path(
                    "/admin/realms/master/authentication/executions/exec-uuid-123",
                ))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .respond_with(ResponseTemplate::new(200).set_body_json(&expected))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthExecutionGetParams {
                realm: "master".to_string(),
                execution_id: "exec-uuid-123".to_string(),
            };

            let result = auth_execution_get(&client, TEST_TOKEN, &params)
                .await
                .unwrap();
            assert_eq!(result.id, Some("exec-uuid-123".to_string()));
            assert_eq!(
                result.authenticator,
                Some("auth-username-password-form".to_string())
            );
        }

        #[tokio::test]
        async fn test_auth_execution_get_not_found() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path(
                    "/admin/realms/master/authentication/executions/nonexistent",
                ))
                .respond_with(ResponseTemplate::new(404))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthExecutionGetParams {
                realm: "master".to_string(),
                execution_id: "nonexistent".to_string(),
            };

            let result = auth_execution_get(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::NotFound)));
        }

        #[tokio::test]
        async fn test_auth_execution_get_unauthorized() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path(
                    "/admin/realms/master/authentication/executions/exec-uuid-123",
                ))
                .respond_with(ResponseTemplate::new(401))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthExecutionGetParams {
                realm: "master".to_string(),
                execution_id: "exec-uuid-123".to_string(),
            };

            let result = auth_execution_get(&client, "invalid", &params).await;
            assert!(matches!(result, Err(ApiError::Unauthorized)));
        }

        #[tokio::test]
        async fn test_auth_execution_get_forbidden() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path(
                    "/admin/realms/master/authentication/executions/exec-uuid-123",
                ))
                .respond_with(ResponseTemplate::new(403))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthExecutionGetParams {
                realm: "master".to_string(),
                execution_id: "exec-uuid-123".to_string(),
            };

            let result = auth_execution_get(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::Forbidden)));
        }

        #[tokio::test]
        async fn test_auth_execution_get_special_characters_realm() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path(
                    "/admin/realms/my%20realm/authentication/executions/exec-uuid-123",
                ))
                .respond_with(ResponseTemplate::new(200).set_body_json(sample_execution()))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthExecutionGetParams {
                realm: "my realm".to_string(),
                execution_id: "exec-uuid-123".to_string(),
            };

            let result = auth_execution_get(&client, TEST_TOKEN, &params).await;
            assert!(result.is_ok());
        }
    }

    mod auth_execution_create_tests {
        use super::*;

        #[tokio::test]
        async fn test_auth_execution_create_success() {
            let mock_server = MockServer::start().await;

            let expected_body = ExecutionCreateRequest {
                provider: "auth-otp-form".to_string(),
            };

            Mock::given(method("POST"))
                .and(path(
                    "/admin/realms/master/authentication/flows/browser/executions/execution",
                ))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .and(body_json(&expected_body))
                .respond_with(ResponseTemplate::new(201))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthExecutionCreateParams {
                realm: "master".to_string(),
                flow_alias: "browser".to_string(),
                provider: "auth-otp-form".to_string(),
            };

            let result = auth_execution_create(&client, TEST_TOKEN, &params).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_auth_execution_create_not_found() {
            let mock_server = MockServer::start().await;

            Mock::given(method("POST"))
                .and(path(
                    "/admin/realms/master/authentication/flows/nonexistent/executions/execution",
                ))
                .respond_with(ResponseTemplate::new(404))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthExecutionCreateParams {
                realm: "master".to_string(),
                flow_alias: "nonexistent".to_string(),
                provider: "auth-otp-form".to_string(),
            };

            let result = auth_execution_create(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::NotFound)));
        }

        #[tokio::test]
        async fn test_auth_execution_create_unauthorized() {
            let mock_server = MockServer::start().await;

            Mock::given(method("POST"))
                .and(path(
                    "/admin/realms/master/authentication/flows/browser/executions/execution",
                ))
                .respond_with(ResponseTemplate::new(401))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthExecutionCreateParams {
                realm: "master".to_string(),
                flow_alias: "browser".to_string(),
                provider: "auth-otp-form".to_string(),
            };

            let result = auth_execution_create(&client, "invalid", &params).await;
            assert!(matches!(result, Err(ApiError::Unauthorized)));
        }

        #[tokio::test]
        async fn test_auth_execution_create_forbidden() {
            let mock_server = MockServer::start().await;

            Mock::given(method("POST"))
                .and(path(
                    "/admin/realms/master/authentication/flows/browser/executions/execution",
                ))
                .respond_with(ResponseTemplate::new(403))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthExecutionCreateParams {
                realm: "master".to_string(),
                flow_alias: "browser".to_string(),
                provider: "auth-otp-form".to_string(),
            };

            let result = auth_execution_create(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::Forbidden)));
        }

        #[tokio::test]
        async fn test_auth_execution_create_special_characters() {
            let mock_server = MockServer::start().await;

            Mock::given(method("POST"))
                .and(path(
                    "/admin/realms/my%20realm/authentication/flows/my%20flow/executions/execution",
                ))
                .respond_with(ResponseTemplate::new(201))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthExecutionCreateParams {
                realm: "my realm".to_string(),
                flow_alias: "my flow".to_string(),
                provider: "auth-otp-form".to_string(),
            };

            let result = auth_execution_create(&client, TEST_TOKEN, &params).await;
            assert!(result.is_ok());
        }
    }

    mod auth_execution_delete_tests {
        use super::*;

        #[tokio::test]
        async fn test_auth_execution_delete_success() {
            let mock_server = MockServer::start().await;

            Mock::given(method("DELETE"))
                .and(path(
                    "/admin/realms/master/authentication/executions/exec-uuid-123",
                ))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .respond_with(ResponseTemplate::new(204))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthExecutionDeleteParams {
                realm: "master".to_string(),
                execution_id: "exec-uuid-123".to_string(),
            };

            let result = auth_execution_delete(&client, TEST_TOKEN, &params).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_auth_execution_delete_not_found() {
            let mock_server = MockServer::start().await;

            Mock::given(method("DELETE"))
                .and(path(
                    "/admin/realms/master/authentication/executions/nonexistent",
                ))
                .respond_with(ResponseTemplate::new(404))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthExecutionDeleteParams {
                realm: "master".to_string(),
                execution_id: "nonexistent".to_string(),
            };

            let result = auth_execution_delete(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::NotFound)));
        }

        #[tokio::test]
        async fn test_auth_execution_delete_unauthorized() {
            let mock_server = MockServer::start().await;

            Mock::given(method("DELETE"))
                .and(path(
                    "/admin/realms/master/authentication/executions/exec-uuid-123",
                ))
                .respond_with(ResponseTemplate::new(401))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthExecutionDeleteParams {
                realm: "master".to_string(),
                execution_id: "exec-uuid-123".to_string(),
            };

            let result = auth_execution_delete(&client, "invalid", &params).await;
            assert!(matches!(result, Err(ApiError::Unauthorized)));
        }

        #[tokio::test]
        async fn test_auth_execution_delete_forbidden() {
            let mock_server = MockServer::start().await;

            Mock::given(method("DELETE"))
                .and(path(
                    "/admin/realms/master/authentication/executions/exec-uuid-123",
                ))
                .respond_with(ResponseTemplate::new(403))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthExecutionDeleteParams {
                realm: "master".to_string(),
                execution_id: "exec-uuid-123".to_string(),
            };

            let result = auth_execution_delete(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::Forbidden)));
        }
    }

    mod auth_execution_raise_priority_tests {
        use super::*;

        #[tokio::test]
        async fn test_auth_execution_raise_priority_success() {
            let mock_server = MockServer::start().await;

            Mock::given(method("POST"))
                .and(path(
                    "/admin/realms/master/authentication/executions/exec-uuid-123/raise-priority",
                ))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .respond_with(ResponseTemplate::new(204))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthExecutionPriorityParams {
                realm: "master".to_string(),
                execution_id: "exec-uuid-123".to_string(),
            };

            let result = auth_execution_raise_priority(&client, TEST_TOKEN, &params).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_auth_execution_raise_priority_not_found() {
            let mock_server = MockServer::start().await;

            Mock::given(method("POST"))
                .and(path(
                    "/admin/realms/master/authentication/executions/nonexistent/raise-priority",
                ))
                .respond_with(ResponseTemplate::new(404))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthExecutionPriorityParams {
                realm: "master".to_string(),
                execution_id: "nonexistent".to_string(),
            };

            let result = auth_execution_raise_priority(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::NotFound)));
        }

        #[tokio::test]
        async fn test_auth_execution_raise_priority_unauthorized() {
            let mock_server = MockServer::start().await;

            Mock::given(method("POST"))
                .and(path(
                    "/admin/realms/master/authentication/executions/exec-uuid-123/raise-priority",
                ))
                .respond_with(ResponseTemplate::new(401))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthExecutionPriorityParams {
                realm: "master".to_string(),
                execution_id: "exec-uuid-123".to_string(),
            };

            let result = auth_execution_raise_priority(&client, "invalid", &params).await;
            assert!(matches!(result, Err(ApiError::Unauthorized)));
        }

        #[tokio::test]
        async fn test_auth_execution_raise_priority_forbidden() {
            let mock_server = MockServer::start().await;

            Mock::given(method("POST"))
                .and(path(
                    "/admin/realms/master/authentication/executions/exec-uuid-123/raise-priority",
                ))
                .respond_with(ResponseTemplate::new(403))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthExecutionPriorityParams {
                realm: "master".to_string(),
                execution_id: "exec-uuid-123".to_string(),
            };

            let result = auth_execution_raise_priority(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::Forbidden)));
        }

        #[tokio::test]
        async fn test_auth_execution_raise_priority_special_characters() {
            let mock_server = MockServer::start().await;

            Mock::given(method("POST"))
                .and(path(
                    "/admin/realms/my%20realm/authentication/executions/exec-uuid-123/raise-priority",
                ))
                .respond_with(ResponseTemplate::new(204))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthExecutionPriorityParams {
                realm: "my realm".to_string(),
                execution_id: "exec-uuid-123".to_string(),
            };

            let result = auth_execution_raise_priority(&client, TEST_TOKEN, &params).await;
            assert!(result.is_ok());
        }
    }

    mod auth_execution_lower_priority_tests {
        use super::*;

        #[tokio::test]
        async fn test_auth_execution_lower_priority_success() {
            let mock_server = MockServer::start().await;

            Mock::given(method("POST"))
                .and(path(
                    "/admin/realms/master/authentication/executions/exec-uuid-123/lower-priority",
                ))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .respond_with(ResponseTemplate::new(204))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthExecutionPriorityParams {
                realm: "master".to_string(),
                execution_id: "exec-uuid-123".to_string(),
            };

            let result = auth_execution_lower_priority(&client, TEST_TOKEN, &params).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_auth_execution_lower_priority_not_found() {
            let mock_server = MockServer::start().await;

            Mock::given(method("POST"))
                .and(path(
                    "/admin/realms/master/authentication/executions/nonexistent/lower-priority",
                ))
                .respond_with(ResponseTemplate::new(404))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthExecutionPriorityParams {
                realm: "master".to_string(),
                execution_id: "nonexistent".to_string(),
            };

            let result = auth_execution_lower_priority(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::NotFound)));
        }

        #[tokio::test]
        async fn test_auth_execution_lower_priority_unauthorized() {
            let mock_server = MockServer::start().await;

            Mock::given(method("POST"))
                .and(path(
                    "/admin/realms/master/authentication/executions/exec-uuid-123/lower-priority",
                ))
                .respond_with(ResponseTemplate::new(401))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthExecutionPriorityParams {
                realm: "master".to_string(),
                execution_id: "exec-uuid-123".to_string(),
            };

            let result = auth_execution_lower_priority(&client, "invalid", &params).await;
            assert!(matches!(result, Err(ApiError::Unauthorized)));
        }

        #[tokio::test]
        async fn test_auth_execution_lower_priority_forbidden() {
            let mock_server = MockServer::start().await;

            Mock::given(method("POST"))
                .and(path(
                    "/admin/realms/master/authentication/executions/exec-uuid-123/lower-priority",
                ))
                .respond_with(ResponseTemplate::new(403))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthExecutionPriorityParams {
                realm: "master".to_string(),
                execution_id: "exec-uuid-123".to_string(),
            };

            let result = auth_execution_lower_priority(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::Forbidden)));
        }
    }

    mod auth_execution_config_get_tests {
        use super::*;

        #[tokio::test]
        async fn test_auth_execution_config_get_success() {
            let mock_server = MockServer::start().await;

            let expected = sample_config();

            Mock::given(method("GET"))
                .and(path(
                    "/admin/realms/master/authentication/config/config-uuid-123",
                ))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .respond_with(ResponseTemplate::new(200).set_body_json(&expected))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthExecutionConfigGetParams {
                realm: "master".to_string(),
                config_id: "config-uuid-123".to_string(),
            };

            let result = auth_execution_config_get(&client, TEST_TOKEN, &params)
                .await
                .unwrap();
            assert_eq!(result.id, Some("config-uuid-123".to_string()));
            assert_eq!(result.alias, Some("otp-config".to_string()));
            assert!(result.config.is_some());
        }

        #[tokio::test]
        async fn test_auth_execution_config_get_not_found() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path(
                    "/admin/realms/master/authentication/config/nonexistent",
                ))
                .respond_with(ResponseTemplate::new(404))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthExecutionConfigGetParams {
                realm: "master".to_string(),
                config_id: "nonexistent".to_string(),
            };

            let result = auth_execution_config_get(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::NotFound)));
        }

        #[tokio::test]
        async fn test_auth_execution_config_get_unauthorized() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path(
                    "/admin/realms/master/authentication/config/config-uuid-123",
                ))
                .respond_with(ResponseTemplate::new(401))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthExecutionConfigGetParams {
                realm: "master".to_string(),
                config_id: "config-uuid-123".to_string(),
            };

            let result = auth_execution_config_get(&client, "invalid", &params).await;
            assert!(matches!(result, Err(ApiError::Unauthorized)));
        }

        #[tokio::test]
        async fn test_auth_execution_config_get_forbidden() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path(
                    "/admin/realms/master/authentication/config/config-uuid-123",
                ))
                .respond_with(ResponseTemplate::new(403))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthExecutionConfigGetParams {
                realm: "master".to_string(),
                config_id: "config-uuid-123".to_string(),
            };

            let result = auth_execution_config_get(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::Forbidden)));
        }

        #[tokio::test]
        async fn test_auth_execution_config_get_special_characters() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path(
                    "/admin/realms/my%20realm/authentication/config/config-uuid-123",
                ))
                .respond_with(ResponseTemplate::new(200).set_body_json(sample_config()))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthExecutionConfigGetParams {
                realm: "my realm".to_string(),
                config_id: "config-uuid-123".to_string(),
            };

            let result = auth_execution_config_get(&client, TEST_TOKEN, &params).await;
            assert!(result.is_ok());
        }
    }

    mod auth_execution_config_update_tests {
        use super::*;

        #[tokio::test]
        async fn test_auth_execution_config_update_success() {
            let mock_server = MockServer::start().await;

            let config = sample_config();

            Mock::given(method("PUT"))
                .and(path(
                    "/admin/realms/master/authentication/config/config-uuid-123",
                ))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .and(body_json(&config))
                .respond_with(ResponseTemplate::new(204))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthExecutionConfigUpdateParams {
                realm: "master".to_string(),
                config_id: "config-uuid-123".to_string(),
                config,
            };

            let result = auth_execution_config_update(&client, TEST_TOKEN, &params).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_auth_execution_config_update_not_found() {
            let mock_server = MockServer::start().await;

            Mock::given(method("PUT"))
                .and(path(
                    "/admin/realms/master/authentication/config/nonexistent",
                ))
                .respond_with(ResponseTemplate::new(404))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthExecutionConfigUpdateParams {
                realm: "master".to_string(),
                config_id: "nonexistent".to_string(),
                config: sample_config(),
            };

            let result = auth_execution_config_update(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::NotFound)));
        }

        #[tokio::test]
        async fn test_auth_execution_config_update_unauthorized() {
            let mock_server = MockServer::start().await;

            Mock::given(method("PUT"))
                .and(path(
                    "/admin/realms/master/authentication/config/config-uuid-123",
                ))
                .respond_with(ResponseTemplate::new(401))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthExecutionConfigUpdateParams {
                realm: "master".to_string(),
                config_id: "config-uuid-123".to_string(),
                config: sample_config(),
            };

            let result = auth_execution_config_update(&client, "invalid", &params).await;
            assert!(matches!(result, Err(ApiError::Unauthorized)));
        }

        #[tokio::test]
        async fn test_auth_execution_config_update_forbidden() {
            let mock_server = MockServer::start().await;

            Mock::given(method("PUT"))
                .and(path(
                    "/admin/realms/master/authentication/config/config-uuid-123",
                ))
                .respond_with(ResponseTemplate::new(403))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthExecutionConfigUpdateParams {
                realm: "master".to_string(),
                config_id: "config-uuid-123".to_string(),
                config: sample_config(),
            };

            let result = auth_execution_config_update(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::Forbidden)));
        }

        #[tokio::test]
        async fn test_auth_execution_config_update_special_characters() {
            let mock_server = MockServer::start().await;

            Mock::given(method("PUT"))
                .and(path(
                    "/admin/realms/my%20realm/authentication/config/config-uuid-123",
                ))
                .respond_with(ResponseTemplate::new(204))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthExecutionConfigUpdateParams {
                realm: "my realm".to_string(),
                config_id: "config-uuid-123".to_string(),
                config: sample_config(),
            };

            let result = auth_execution_config_update(&client, TEST_TOKEN, &params).await;
            assert!(result.is_ok());
        }
    }
}
