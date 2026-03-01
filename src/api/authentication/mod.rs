//! Keycloak Authentication Management API module.
//!
//! Provides operations for managing authentication flows in Keycloak realms.

pub mod executions;
pub mod required_actions;
pub mod types;

pub use executions::*;
pub use required_actions::*;
pub use types::*;

use serde::Deserialize;

use crate::api::{ApiError, KeycloakClient};

/// Parameters for listing authentication flows.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AuthFlowsListParams {
    pub realm: String,
}

impl AuthFlowsListParams {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.realm.trim().is_empty() {
            return Err(ApiError::BadRequest("realm is required".to_string()));
        }
        Ok(())
    }
}

/// Parameters for getting a single authentication flow by ID.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AuthFlowGetParams {
    pub realm: String,
    pub flow_id: String,
}

impl AuthFlowGetParams {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.realm.trim().is_empty() {
            return Err(ApiError::BadRequest("realm is required".to_string()));
        }
        if self.flow_id.trim().is_empty() {
            return Err(ApiError::BadRequest("flow_id is required".to_string()));
        }
        Ok(())
    }
}

/// Parameters for creating a new authentication flow.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AuthFlowCreateParams {
    pub realm: String,
    pub flow: AuthenticationFlowRepresentation,
}

impl AuthFlowCreateParams {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.realm.trim().is_empty() {
            return Err(ApiError::BadRequest("realm is required".to_string()));
        }
        if self.flow.alias.as_ref().is_none_or(|a| a.trim().is_empty()) {
            return Err(ApiError::BadRequest("flow alias is required".to_string()));
        }
        Ok(())
    }
}

/// Parameters for copying an authentication flow.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AuthFlowCopyParams {
    pub realm: String,
    pub flow_alias: String,
    pub new_name: String,
}

impl AuthFlowCopyParams {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.realm.trim().is_empty() {
            return Err(ApiError::BadRequest("realm is required".to_string()));
        }
        if self.flow_alias.trim().is_empty() {
            return Err(ApiError::BadRequest("flow_alias is required".to_string()));
        }
        if self.new_name.trim().is_empty() {
            return Err(ApiError::BadRequest("new_name is required".to_string()));
        }
        Ok(())
    }
}

/// Parameters for deleting an authentication flow.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AuthFlowDeleteParams {
    pub realm: String,
    pub flow_id: String,
}

impl AuthFlowDeleteParams {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.realm.trim().is_empty() {
            return Err(ApiError::BadRequest("realm is required".to_string()));
        }
        if self.flow_id.trim().is_empty() {
            return Err(ApiError::BadRequest("flow_id is required".to_string()));
        }
        Ok(())
    }
}

/// Parameters for listing executions within an authentication flow.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AuthFlowExecutionsListParams {
    pub realm: String,
    pub flow_alias: String,
}

impl AuthFlowExecutionsListParams {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.realm.trim().is_empty() {
            return Err(ApiError::BadRequest("realm is required".to_string()));
        }
        if self.flow_alias.trim().is_empty() {
            return Err(ApiError::BadRequest("flow_alias is required".to_string()));
        }
        Ok(())
    }
}

/// List all authentication flows in a realm.
///
/// GET /admin/realms/{realm}/authentication/flows
pub async fn auth_flows_list(
    client: &KeycloakClient,
    token: &str,
    params: &AuthFlowsListParams,
) -> Result<Vec<AuthenticationFlowRepresentation>, ApiError> {
    params.validate()?;

    let path = format!(
        "/admin/realms/{}/authentication/flows",
        urlencoding::encode(&params.realm)
    );

    client.get(&path, token).await
}

/// Get a single authentication flow by ID.
///
/// GET /admin/realms/{realm}/authentication/flows/{id}
pub async fn auth_flow_get(
    client: &KeycloakClient,
    token: &str,
    params: &AuthFlowGetParams,
) -> Result<AuthenticationFlowRepresentation, ApiError> {
    params.validate()?;

    let path = format!(
        "/admin/realms/{}/authentication/flows/{}",
        urlencoding::encode(&params.realm),
        urlencoding::encode(&params.flow_id)
    );

    client.get(&path, token).await
}

/// Create a new authentication flow.
///
/// POST /admin/realms/{realm}/authentication/flows
pub async fn auth_flow_create(
    client: &KeycloakClient,
    token: &str,
    params: &AuthFlowCreateParams,
) -> Result<(), ApiError> {
    params.validate()?;

    let path = format!(
        "/admin/realms/{}/authentication/flows",
        urlencoding::encode(&params.realm)
    );

    client.post_no_response(&path, token, &params.flow).await
}

/// Copy an existing authentication flow.
///
/// POST /admin/realms/{realm}/authentication/flows/{flowAlias}/copy
pub async fn auth_flow_copy(
    client: &KeycloakClient,
    token: &str,
    params: &AuthFlowCopyParams,
) -> Result<(), ApiError> {
    params.validate()?;

    let path = format!(
        "/admin/realms/{}/authentication/flows/{}/copy",
        urlencoding::encode(&params.realm),
        urlencoding::encode(&params.flow_alias)
    );

    let body = FlowCopyRequest {
        new_name: params.new_name.clone(),
    };

    client.post_no_response(&path, token, &body).await
}

/// Delete an authentication flow.
///
/// DELETE /admin/realms/{realm}/authentication/flows/{id}
///
/// Note: Built-in flows cannot be deleted; Keycloak will return a 400 error.
pub async fn auth_flow_delete(
    client: &KeycloakClient,
    token: &str,
    params: &AuthFlowDeleteParams,
) -> Result<(), ApiError> {
    params.validate()?;

    let path = format!(
        "/admin/realms/{}/authentication/flows/{}",
        urlencoding::encode(&params.realm),
        urlencoding::encode(&params.flow_id)
    );

    client.delete(&path, token).await
}

/// List all executions within an authentication flow.
///
/// GET /admin/realms/{realm}/authentication/flows/{flowAlias}/executions
pub async fn auth_flow_executions_list(
    client: &KeycloakClient,
    token: &str,
    params: &AuthFlowExecutionsListParams,
) -> Result<Vec<AuthenticationExecutionInfoRepresentation>, ApiError> {
    params.validate()?;

    let path = format!(
        "/admin/realms/{}/authentication/flows/{}/executions",
        urlencoding::encode(&params.realm),
        urlencoding::encode(&params.flow_alias)
    );

    client.get(&path, token).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{body_json, header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    const TEST_TOKEN: &str = "test-bearer-token";

    fn sample_flow() -> AuthenticationFlowRepresentation {
        AuthenticationFlowRepresentation {
            id: Some("flow-123".to_string()),
            alias: Some("browser".to_string()),
            description: Some("Browser based authentication".to_string()),
            provider_id: Some("basic-flow".to_string()),
            top_level: Some(true),
            built_in: Some(true),
            authentication_executions: None,
        }
    }

    fn sample_execution() -> AuthenticationExecutionInfoRepresentation {
        AuthenticationExecutionInfoRepresentation {
            id: Some("exec-123".to_string()),
            requirement: Some("REQUIRED".to_string()),
            display_name: Some("Username Password Form".to_string()),
            alias: None,
            description: Some("Validates username and password".to_string()),
            requirement_choices: Some(vec![
                "REQUIRED".to_string(),
                "ALTERNATIVE".to_string(),
                "DISABLED".to_string(),
            ]),
            configurable: Some(false),
            provider_id: Some("auth-username-password-form".to_string()),
            level: Some(0),
            index: Some(0),
            authentication_flow: Some(false),
            flow_id: None,
        }
    }

    mod validation_tests {
        use super::*;

        #[test]
        fn test_auth_flows_list_params_valid() {
            let params = AuthFlowsListParams {
                realm: "master".to_string(),
            };
            assert!(params.validate().is_ok());
        }

        #[test]
        fn test_auth_flows_list_params_empty_realm() {
            let params = AuthFlowsListParams {
                realm: "".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_auth_flows_list_params_whitespace_realm() {
            let params = AuthFlowsListParams {
                realm: "   ".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_auth_flow_get_params_valid() {
            let params = AuthFlowGetParams {
                realm: "master".to_string(),
                flow_id: "flow-123".to_string(),
            };
            assert!(params.validate().is_ok());
        }

        #[test]
        fn test_auth_flow_get_params_empty_realm() {
            let params = AuthFlowGetParams {
                realm: "".to_string(),
                flow_id: "flow-123".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_auth_flow_get_params_empty_flow_id() {
            let params = AuthFlowGetParams {
                realm: "master".to_string(),
                flow_id: "".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_auth_flow_create_params_valid() {
            let params = AuthFlowCreateParams {
                realm: "master".to_string(),
                flow: AuthenticationFlowRepresentation {
                    alias: Some("my-flow".to_string()),
                    ..Default::default()
                },
            };
            assert!(params.validate().is_ok());
        }

        #[test]
        fn test_auth_flow_create_params_empty_realm() {
            let params = AuthFlowCreateParams {
                realm: "".to_string(),
                flow: AuthenticationFlowRepresentation {
                    alias: Some("my-flow".to_string()),
                    ..Default::default()
                },
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_auth_flow_create_params_missing_alias() {
            let params = AuthFlowCreateParams {
                realm: "master".to_string(),
                flow: AuthenticationFlowRepresentation::default(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_auth_flow_create_params_empty_alias() {
            let params = AuthFlowCreateParams {
                realm: "master".to_string(),
                flow: AuthenticationFlowRepresentation {
                    alias: Some("".to_string()),
                    ..Default::default()
                },
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_auth_flow_copy_params_valid() {
            let params = AuthFlowCopyParams {
                realm: "master".to_string(),
                flow_alias: "browser".to_string(),
                new_name: "my-browser".to_string(),
            };
            assert!(params.validate().is_ok());
        }

        #[test]
        fn test_auth_flow_copy_params_empty_realm() {
            let params = AuthFlowCopyParams {
                realm: "".to_string(),
                flow_alias: "browser".to_string(),
                new_name: "my-browser".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_auth_flow_copy_params_empty_flow_alias() {
            let params = AuthFlowCopyParams {
                realm: "master".to_string(),
                flow_alias: "".to_string(),
                new_name: "my-browser".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_auth_flow_copy_params_empty_new_name() {
            let params = AuthFlowCopyParams {
                realm: "master".to_string(),
                flow_alias: "browser".to_string(),
                new_name: "".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_auth_flow_delete_params_valid() {
            let params = AuthFlowDeleteParams {
                realm: "master".to_string(),
                flow_id: "flow-123".to_string(),
            };
            assert!(params.validate().is_ok());
        }

        #[test]
        fn test_auth_flow_delete_params_empty_realm() {
            let params = AuthFlowDeleteParams {
                realm: "".to_string(),
                flow_id: "flow-123".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_auth_flow_delete_params_empty_flow_id() {
            let params = AuthFlowDeleteParams {
                realm: "master".to_string(),
                flow_id: "".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_auth_flow_executions_list_params_valid() {
            let params = AuthFlowExecutionsListParams {
                realm: "master".to_string(),
                flow_alias: "browser".to_string(),
            };
            assert!(params.validate().is_ok());
        }

        #[test]
        fn test_auth_flow_executions_list_params_empty_realm() {
            let params = AuthFlowExecutionsListParams {
                realm: "".to_string(),
                flow_alias: "browser".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_auth_flow_executions_list_params_empty_flow_alias() {
            let params = AuthFlowExecutionsListParams {
                realm: "master".to_string(),
                flow_alias: "".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }
    }

    mod auth_flows_list_tests {
        use super::*;

        #[tokio::test]
        async fn test_auth_flows_list_success() {
            let mock_server = MockServer::start().await;

            let expected_flows = vec![sample_flow()];

            Mock::given(method("GET"))
                .and(path("/admin/realms/master/authentication/flows"))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .respond_with(ResponseTemplate::new(200).set_body_json(&expected_flows))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthFlowsListParams {
                realm: "master".to_string(),
            };

            let flows = auth_flows_list(&client, TEST_TOKEN, &params).await.unwrap();
            assert_eq!(flows.len(), 1);
            assert_eq!(flows[0].alias, Some("browser".to_string()));
        }

        #[tokio::test]
        async fn test_auth_flows_list_empty() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path("/admin/realms/master/authentication/flows"))
                .respond_with(
                    ResponseTemplate::new(200)
                        .set_body_json(Vec::<AuthenticationFlowRepresentation>::new()),
                )
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthFlowsListParams {
                realm: "master".to_string(),
            };

            let flows = auth_flows_list(&client, TEST_TOKEN, &params).await.unwrap();
            assert!(flows.is_empty());
        }

        #[tokio::test]
        async fn test_auth_flows_list_unauthorized() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path("/admin/realms/master/authentication/flows"))
                .respond_with(ResponseTemplate::new(401))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthFlowsListParams {
                realm: "master".to_string(),
            };

            let result = auth_flows_list(&client, "invalid", &params).await;
            assert!(matches!(result, Err(ApiError::Unauthorized)));
        }

        #[tokio::test]
        async fn test_auth_flows_list_forbidden() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path("/admin/realms/master/authentication/flows"))
                .respond_with(ResponseTemplate::new(403))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthFlowsListParams {
                realm: "master".to_string(),
            };

            let result = auth_flows_list(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::Forbidden)));
        }

        #[tokio::test]
        async fn test_auth_flows_list_special_characters_realm() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path("/admin/realms/my%20realm/authentication/flows"))
                .respond_with(
                    ResponseTemplate::new(200)
                        .set_body_json(Vec::<AuthenticationFlowRepresentation>::new()),
                )
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthFlowsListParams {
                realm: "my realm".to_string(),
            };

            let result = auth_flows_list(&client, TEST_TOKEN, &params).await;
            assert!(result.is_ok());
        }
    }

    mod auth_flow_get_tests {
        use super::*;

        #[tokio::test]
        async fn test_auth_flow_get_success() {
            let mock_server = MockServer::start().await;

            let expected_flow = sample_flow();

            Mock::given(method("GET"))
                .and(path("/admin/realms/master/authentication/flows/flow-123"))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .respond_with(ResponseTemplate::new(200).set_body_json(&expected_flow))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthFlowGetParams {
                realm: "master".to_string(),
                flow_id: "flow-123".to_string(),
            };

            let flow = auth_flow_get(&client, TEST_TOKEN, &params).await.unwrap();
            assert_eq!(flow.id, Some("flow-123".to_string()));
            assert_eq!(flow.alias, Some("browser".to_string()));
        }

        #[tokio::test]
        async fn test_auth_flow_get_not_found() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path(
                    "/admin/realms/master/authentication/flows/nonexistent",
                ))
                .respond_with(ResponseTemplate::new(404))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthFlowGetParams {
                realm: "master".to_string(),
                flow_id: "nonexistent".to_string(),
            };

            let result = auth_flow_get(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::NotFound)));
        }

        #[tokio::test]
        async fn test_auth_flow_get_special_characters() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path(
                    "/admin/realms/master/authentication/flows/flow%2Fwith%2Fslash",
                ))
                .respond_with(ResponseTemplate::new(200).set_body_json(sample_flow()))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthFlowGetParams {
                realm: "master".to_string(),
                flow_id: "flow/with/slash".to_string(),
            };

            let result = auth_flow_get(&client, TEST_TOKEN, &params).await;
            assert!(result.is_ok());
        }
    }

    mod auth_flow_create_tests {
        use super::*;

        #[tokio::test]
        async fn test_auth_flow_create_success() {
            let mock_server = MockServer::start().await;

            let new_flow = AuthenticationFlowRepresentation {
                alias: Some("my-custom-flow".to_string()),
                description: Some("Custom authentication flow".to_string()),
                provider_id: Some("basic-flow".to_string()),
                top_level: Some(true),
                built_in: Some(false),
                ..Default::default()
            };

            Mock::given(method("POST"))
                .and(path("/admin/realms/master/authentication/flows"))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .and(body_json(&new_flow))
                .respond_with(ResponseTemplate::new(201))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthFlowCreateParams {
                realm: "master".to_string(),
                flow: new_flow,
            };

            let result = auth_flow_create(&client, TEST_TOKEN, &params).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_auth_flow_create_conflict() {
            let mock_server = MockServer::start().await;

            Mock::given(method("POST"))
                .and(path("/admin/realms/master/authentication/flows"))
                .respond_with(ResponseTemplate::new(409).set_body_json(serde_json::json!({
                    "errorMessage": "Flow with alias browser already exists"
                })))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthFlowCreateParams {
                realm: "master".to_string(),
                flow: AuthenticationFlowRepresentation {
                    alias: Some("browser".to_string()),
                    ..Default::default()
                },
            };

            let result = auth_flow_create(&client, TEST_TOKEN, &params).await;
            match result {
                Err(ApiError::Conflict(msg)) => {
                    assert!(msg.contains("already exists"));
                }
                _ => panic!("Expected Conflict error"),
            }
        }
    }

    mod auth_flow_copy_tests {
        use super::*;

        #[tokio::test]
        async fn test_auth_flow_copy_success() {
            let mock_server = MockServer::start().await;

            let copy_request = FlowCopyRequest {
                new_name: "my-browser-copy".to_string(),
            };

            Mock::given(method("POST"))
                .and(path(
                    "/admin/realms/master/authentication/flows/browser/copy",
                ))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .and(body_json(&copy_request))
                .respond_with(ResponseTemplate::new(201))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthFlowCopyParams {
                realm: "master".to_string(),
                flow_alias: "browser".to_string(),
                new_name: "my-browser-copy".to_string(),
            };

            let result = auth_flow_copy(&client, TEST_TOKEN, &params).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_auth_flow_copy_not_found() {
            let mock_server = MockServer::start().await;

            Mock::given(method("POST"))
                .and(path(
                    "/admin/realms/master/authentication/flows/nonexistent/copy",
                ))
                .respond_with(ResponseTemplate::new(404))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthFlowCopyParams {
                realm: "master".to_string(),
                flow_alias: "nonexistent".to_string(),
                new_name: "copy-of-nonexistent".to_string(),
            };

            let result = auth_flow_copy(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::NotFound)));
        }

        #[tokio::test]
        async fn test_auth_flow_copy_special_characters_alias() {
            let mock_server = MockServer::start().await;

            Mock::given(method("POST"))
                .and(path(
                    "/admin/realms/master/authentication/flows/my%20flow/copy",
                ))
                .respond_with(ResponseTemplate::new(201))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthFlowCopyParams {
                realm: "master".to_string(),
                flow_alias: "my flow".to_string(),
                new_name: "my-flow-copy".to_string(),
            };

            let result = auth_flow_copy(&client, TEST_TOKEN, &params).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_auth_flow_copy_conflict() {
            let mock_server = MockServer::start().await;

            Mock::given(method("POST"))
                .and(path(
                    "/admin/realms/master/authentication/flows/browser/copy",
                ))
                .respond_with(ResponseTemplate::new(409).set_body_json(serde_json::json!({
                    "errorMessage": "Flow with alias 'existing' already exists"
                })))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthFlowCopyParams {
                realm: "master".to_string(),
                flow_alias: "browser".to_string(),
                new_name: "existing".to_string(),
            };

            let result = auth_flow_copy(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::Conflict(_))));
        }
    }

    mod auth_flow_delete_tests {
        use super::*;

        #[tokio::test]
        async fn test_auth_flow_delete_success() {
            let mock_server = MockServer::start().await;

            Mock::given(method("DELETE"))
                .and(path("/admin/realms/master/authentication/flows/flow-123"))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .respond_with(ResponseTemplate::new(204))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthFlowDeleteParams {
                realm: "master".to_string(),
                flow_id: "flow-123".to_string(),
            };

            let result = auth_flow_delete(&client, TEST_TOKEN, &params).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_auth_flow_delete_not_found() {
            let mock_server = MockServer::start().await;

            Mock::given(method("DELETE"))
                .and(path(
                    "/admin/realms/master/authentication/flows/nonexistent",
                ))
                .respond_with(ResponseTemplate::new(404))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthFlowDeleteParams {
                realm: "master".to_string(),
                flow_id: "nonexistent".to_string(),
            };

            let result = auth_flow_delete(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::NotFound)));
        }

        #[tokio::test]
        async fn test_auth_flow_delete_builtin_bad_request() {
            let mock_server = MockServer::start().await;

            Mock::given(method("DELETE"))
                .and(path(
                    "/admin/realms/master/authentication/flows/browser-builtin-id",
                ))
                .respond_with(ResponseTemplate::new(400).set_body_json(serde_json::json!({
                    "errorMessage": "Cannot delete built-in flow"
                })))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthFlowDeleteParams {
                realm: "master".to_string(),
                flow_id: "browser-builtin-id".to_string(),
            };

            let result = auth_flow_delete(&client, TEST_TOKEN, &params).await;
            match result {
                Err(ApiError::BadRequest(msg)) => {
                    assert!(msg.contains("Cannot delete built-in flow"));
                }
                _ => panic!("Expected BadRequest error for built-in flow deletion"),
            }
        }

        #[tokio::test]
        async fn test_auth_flow_delete_forbidden() {
            let mock_server = MockServer::start().await;

            Mock::given(method("DELETE"))
                .and(path("/admin/realms/master/authentication/flows/flow-123"))
                .respond_with(ResponseTemplate::new(403))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthFlowDeleteParams {
                realm: "master".to_string(),
                flow_id: "flow-123".to_string(),
            };

            let result = auth_flow_delete(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::Forbidden)));
        }
    }

    mod auth_flow_executions_list_tests {
        use super::*;

        #[tokio::test]
        async fn test_auth_flow_executions_list_success() {
            let mock_server = MockServer::start().await;

            let expected_executions = vec![sample_execution()];

            Mock::given(method("GET"))
                .and(path(
                    "/admin/realms/master/authentication/flows/browser/executions",
                ))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .respond_with(ResponseTemplate::new(200).set_body_json(&expected_executions))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthFlowExecutionsListParams {
                realm: "master".to_string(),
                flow_alias: "browser".to_string(),
            };

            let executions = auth_flow_executions_list(&client, TEST_TOKEN, &params)
                .await
                .unwrap();
            assert_eq!(executions.len(), 1);
            assert_eq!(executions[0].id, Some("exec-123".to_string()));
            assert_eq!(executions[0].requirement, Some("REQUIRED".to_string()));
        }

        #[tokio::test]
        async fn test_auth_flow_executions_list_empty() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path(
                    "/admin/realms/master/authentication/flows/empty-flow/executions",
                ))
                .respond_with(
                    ResponseTemplate::new(200)
                        .set_body_json(Vec::<AuthenticationExecutionInfoRepresentation>::new()),
                )
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthFlowExecutionsListParams {
                realm: "master".to_string(),
                flow_alias: "empty-flow".to_string(),
            };

            let executions = auth_flow_executions_list(&client, TEST_TOKEN, &params)
                .await
                .unwrap();
            assert!(executions.is_empty());
        }

        #[tokio::test]
        async fn test_auth_flow_executions_list_not_found() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path(
                    "/admin/realms/master/authentication/flows/nonexistent/executions",
                ))
                .respond_with(ResponseTemplate::new(404))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthFlowExecutionsListParams {
                realm: "master".to_string(),
                flow_alias: "nonexistent".to_string(),
            };

            let result = auth_flow_executions_list(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::NotFound)));
        }

        #[tokio::test]
        async fn test_auth_flow_executions_list_special_characters_alias() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path(
                    "/admin/realms/master/authentication/flows/my%20browser%20flow/executions",
                ))
                .respond_with(
                    ResponseTemplate::new(200)
                        .set_body_json(Vec::<AuthenticationExecutionInfoRepresentation>::new()),
                )
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthFlowExecutionsListParams {
                realm: "master".to_string(),
                flow_alias: "my browser flow".to_string(),
            };

            let result = auth_flow_executions_list(&client, TEST_TOKEN, &params).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_auth_flow_executions_list_multiple() {
            let mock_server = MockServer::start().await;

            let executions = vec![
                AuthenticationExecutionInfoRepresentation {
                    id: Some("exec-1".to_string()),
                    requirement: Some("ALTERNATIVE".to_string()),
                    display_name: Some("Cookie".to_string()),
                    level: Some(0),
                    index: Some(0),
                    ..Default::default()
                },
                AuthenticationExecutionInfoRepresentation {
                    id: Some("exec-2".to_string()),
                    requirement: Some("ALTERNATIVE".to_string()),
                    display_name: Some("Identity Provider Redirector".to_string()),
                    level: Some(0),
                    index: Some(1),
                    ..Default::default()
                },
                AuthenticationExecutionInfoRepresentation {
                    id: Some("exec-3".to_string()),
                    requirement: Some("ALTERNATIVE".to_string()),
                    display_name: Some("Forms".to_string()),
                    level: Some(0),
                    index: Some(2),
                    authentication_flow: Some(true),
                    flow_id: Some("forms-subflow-id".to_string()),
                    ..Default::default()
                },
            ];

            Mock::given(method("GET"))
                .and(path(
                    "/admin/realms/master/authentication/flows/browser/executions",
                ))
                .respond_with(ResponseTemplate::new(200).set_body_json(&executions))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = AuthFlowExecutionsListParams {
                realm: "master".to_string(),
                flow_alias: "browser".to_string(),
            };

            let result = auth_flow_executions_list(&client, TEST_TOKEN, &params)
                .await
                .unwrap();
            assert_eq!(result.len(), 3);

            assert_eq!(result[0].index, Some(0));
            assert_eq!(result[1].index, Some(1));
            assert_eq!(result[2].index, Some(2));

            assert_eq!(result[2].authentication_flow, Some(true));
            assert_eq!(result[2].flow_id, Some("forms-subflow-id".to_string()));
        }
    }
}
