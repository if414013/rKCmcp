//! Users API module for Keycloak Admin REST API.

pub mod credentials;
pub mod groups;
pub mod roles;
pub mod sessions;
pub mod types;

pub use types::{CredentialRepresentation, FederatedIdentityRepresentation, UserRepresentation};

use serde::Deserialize;

use crate::api::{ApiError, KeycloakClient};

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct UserListParams {
    pub realm: String,
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub first: Option<i32>,
    #[serde(default)]
    pub max: Option<i32>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default, rename = "firstName")]
    pub first_name: Option<String>,
    #[serde(default, rename = "lastName")]
    pub last_name: Option<String>,
    #[serde(default)]
    pub enabled: Option<bool>,
    #[serde(default)]
    pub exact: Option<bool>,
}

impl UserListParams {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.realm.trim().is_empty() {
            return Err(ApiError::BadRequest("realm is required".to_string()));
        }
        if let Some(first) = self.first {
            if first < 0 {
                return Err(ApiError::BadRequest("first must be non-negative".to_string()));
            }
        }
        if let Some(max) = self.max {
            if max < 0 {
                return Err(ApiError::BadRequest("max must be non-negative".to_string()));
            }
        }
        Ok(())
    }

    pub fn build_query_string(&self) -> String {
        let mut params = Vec::new();

        if let Some(ref search) = self.search {
            params.push(format!("search={}", urlencoding::encode(search)));
        }
        if let Some(ref email) = self.email {
            params.push(format!("email={}", urlencoding::encode(email)));
        }
        if let Some(ref username) = self.username {
            params.push(format!("username={}", urlencoding::encode(username)));
        }
        if let Some(ref first_name) = self.first_name {
            params.push(format!("firstName={}", urlencoding::encode(first_name)));
        }
        if let Some(ref last_name) = self.last_name {
            params.push(format!("lastName={}", urlencoding::encode(last_name)));
        }
        if let Some(enabled) = self.enabled {
            params.push(format!("enabled={}", enabled));
        }
        if let Some(exact) = self.exact {
            params.push(format!("exact={}", exact));
        }
        if let Some(first) = self.first {
            params.push(format!("first={}", first));
        }
        if let Some(max) = self.max {
            params.push(format!("max={}", max));
        }

        if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        }
    }
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct UserGetParams {
    pub realm: String,
    pub user_id: String,
}

impl UserGetParams {
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

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct UserCreateParams {
    pub realm: String,
    pub user: UserRepresentation,
}

impl UserCreateParams {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.realm.trim().is_empty() {
            return Err(ApiError::BadRequest("realm is required".to_string()));
        }
        if self.user.username.as_ref().is_none_or(|u| u.trim().is_empty()) {
            return Err(ApiError::BadRequest("username is required".to_string()));
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct UserUpdateParams {
    pub realm: String,
    pub user_id: String,
    pub user: UserRepresentation,
}

impl UserUpdateParams {
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

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct UserDeleteParams {
    pub realm: String,
    pub user_id: String,
}

impl UserDeleteParams {
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

pub async fn user_list(
    client: &KeycloakClient,
    token: &str,
    params: &UserListParams,
) -> Result<Vec<UserRepresentation>, ApiError> {
    params.validate()?;

    let query = params.build_query_string();
    let path = format!("/admin/realms/{}/users{}", params.realm, query);

    client.get(&path, token).await
}

pub async fn user_get(
    client: &KeycloakClient,
    token: &str,
    params: &UserGetParams,
) -> Result<UserRepresentation, ApiError> {
    params.validate()?;

    let path = format!("/admin/realms/{}/users/{}", params.realm, params.user_id);
    client.get(&path, token).await
}

pub async fn user_create(
    client: &KeycloakClient,
    token: &str,
    params: &UserCreateParams,
) -> Result<(), ApiError> {
    params.validate()?;

    let path = format!("/admin/realms/{}/users", params.realm);
    client.post_no_response(&path, token, &params.user).await
}

pub async fn user_update(
    client: &KeycloakClient,
    token: &str,
    params: &UserUpdateParams,
) -> Result<(), ApiError> {
    params.validate()?;

    let path = format!("/admin/realms/{}/users/{}", params.realm, params.user_id);
    client.put(&path, token, &params.user).await
}

pub async fn user_delete(
    client: &KeycloakClient,
    token: &str,
    params: &UserDeleteParams,
) -> Result<(), ApiError> {
    params.validate()?;

    let path = format!("/admin/realms/{}/users/{}", params.realm, params.user_id);
    client.delete(&path, token).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{header, method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    const TEST_TOKEN: &str = "test-bearer-token";

    mod user_list_params_tests {
        use super::*;

        #[test]
        fn test_validate_success() {
            let params = UserListParams {
                realm: "master".to_string(),
                search: None,
                first: Some(0),
                max: Some(10),
                email: None,
                username: None,
                first_name: None,
                last_name: None,
                enabled: None,
                exact: None,
            };
            assert!(params.validate().is_ok());
        }

        #[test]
        fn test_validate_empty_realm() {
            let params = UserListParams {
                realm: "".to_string(),
                search: None,
                first: None,
                max: None,
                email: None,
                username: None,
                first_name: None,
                last_name: None,
                enabled: None,
                exact: None,
            };
            let result = params.validate();
            assert!(matches!(result, Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_whitespace_realm() {
            let params = UserListParams {
                realm: "   ".to_string(),
                search: None,
                first: None,
                max: None,
                email: None,
                username: None,
                first_name: None,
                last_name: None,
                enabled: None,
                exact: None,
            };
            let result = params.validate();
            assert!(matches!(result, Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_negative_first() {
            let params = UserListParams {
                realm: "master".to_string(),
                search: None,
                first: Some(-1),
                max: None,
                email: None,
                username: None,
                first_name: None,
                last_name: None,
                enabled: None,
                exact: None,
            };
            let result = params.validate();
            assert!(matches!(result, Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_negative_max() {
            let params = UserListParams {
                realm: "master".to_string(),
                search: None,
                first: None,
                max: Some(-1),
                email: None,
                username: None,
                first_name: None,
                last_name: None,
                enabled: None,
                exact: None,
            };
            let result = params.validate();
            assert!(matches!(result, Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_build_query_string_empty() {
            let params = UserListParams {
                realm: "master".to_string(),
                search: None,
                first: None,
                max: None,
                email: None,
                username: None,
                first_name: None,
                last_name: None,
                enabled: None,
                exact: None,
            };
            assert_eq!(params.build_query_string(), "");
        }

        #[test]
        fn test_build_query_string_with_pagination() {
            let params = UserListParams {
                realm: "master".to_string(),
                search: None,
                first: Some(10),
                max: Some(20),
                email: None,
                username: None,
                first_name: None,
                last_name: None,
                enabled: None,
                exact: None,
            };
            let query = params.build_query_string();
            assert!(query.contains("first=10"));
            assert!(query.contains("max=20"));
        }

        #[test]
        fn test_build_query_string_with_search() {
            let params = UserListParams {
                realm: "master".to_string(),
                search: Some("john".to_string()),
                first: None,
                max: None,
                email: None,
                username: None,
                first_name: None,
                last_name: None,
                enabled: None,
                exact: None,
            };
            let query = params.build_query_string();
            assert!(query.contains("search=john"));
        }

        #[test]
        fn test_build_query_string_with_filters() {
            let params = UserListParams {
                realm: "master".to_string(),
                search: None,
                first: None,
                max: None,
                email: Some("test@example.com".to_string()),
                username: Some("testuser".to_string()),
                first_name: Some("John".to_string()),
                last_name: Some("Doe".to_string()),
                enabled: Some(true),
                exact: Some(true),
            };
            let query = params.build_query_string();
            assert!(query.contains("email=test%40example.com"));
            assert!(query.contains("username=testuser"));
            assert!(query.contains("firstName=John"));
            assert!(query.contains("lastName=Doe"));
            assert!(query.contains("enabled=true"));
            assert!(query.contains("exact=true"));
        }

        #[test]
        fn test_build_query_string_encodes_special_chars() {
            let params = UserListParams {
                realm: "master".to_string(),
                search: Some("john doe".to_string()),
                first: None,
                max: None,
                email: None,
                username: None,
                first_name: None,
                last_name: None,
                enabled: None,
                exact: None,
            };
            let query = params.build_query_string();
            assert!(query.contains("search=john%20doe"));
        }
    }

    mod user_get_params_tests {
        use super::*;

        #[test]
        fn test_validate_success() {
            let params = UserGetParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
            };
            assert!(params.validate().is_ok());
        }

        #[test]
        fn test_validate_empty_realm() {
            let params = UserGetParams {
                realm: "".to_string(),
                user_id: "123".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_user_id() {
            let params = UserGetParams {
                realm: "master".to_string(),
                user_id: "".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }
    }

    mod user_create_params_tests {
        use super::*;

        #[test]
        fn test_validate_success() {
            let params = UserCreateParams {
                realm: "master".to_string(),
                user: UserRepresentation {
                    username: Some("newuser".to_string()),
                    ..Default::default()
                },
            };
            assert!(params.validate().is_ok());
        }

        #[test]
        fn test_validate_empty_realm() {
            let params = UserCreateParams {
                realm: "".to_string(),
                user: UserRepresentation {
                    username: Some("newuser".to_string()),
                    ..Default::default()
                },
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_missing_username() {
            let params = UserCreateParams {
                realm: "master".to_string(),
                user: UserRepresentation::default(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_username() {
            let params = UserCreateParams {
                realm: "master".to_string(),
                user: UserRepresentation {
                    username: Some("".to_string()),
                    ..Default::default()
                },
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }
    }

    mod user_update_params_tests {
        use super::*;

        #[test]
        fn test_validate_success() {
            let params = UserUpdateParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                user: UserRepresentation {
                    email: Some("updated@example.com".to_string()),
                    ..Default::default()
                },
            };
            assert!(params.validate().is_ok());
        }

        #[test]
        fn test_validate_empty_realm() {
            let params = UserUpdateParams {
                realm: "".to_string(),
                user_id: "123".to_string(),
                user: UserRepresentation::default(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_user_id() {
            let params = UserUpdateParams {
                realm: "master".to_string(),
                user_id: "".to_string(),
                user: UserRepresentation::default(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }
    }

    mod user_delete_params_tests {
        use super::*;

        #[test]
        fn test_validate_success() {
            let params = UserDeleteParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
            };
            assert!(params.validate().is_ok());
        }

        #[test]
        fn test_validate_empty_realm() {
            let params = UserDeleteParams {
                realm: "".to_string(),
                user_id: "123".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_user_id() {
            let params = UserDeleteParams {
                realm: "master".to_string(),
                user_id: "".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }
    }

    mod user_list_tests {
        use super::*;

        #[tokio::test]
        async fn test_user_list_success() {
            let mock_server = MockServer::start().await;

            let expected_users = vec![
                UserRepresentation {
                    id: Some("1".to_string()),
                    username: Some("user1".to_string()),
                    ..Default::default()
                },
                UserRepresentation {
                    id: Some("2".to_string()),
                    username: Some("user2".to_string()),
                    ..Default::default()
                },
            ];

            Mock::given(method("GET"))
                .and(path("/admin/realms/master/users"))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .respond_with(ResponseTemplate::new(200).set_body_json(&expected_users))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = UserListParams {
                realm: "master".to_string(),
                search: None,
                first: None,
                max: None,
                email: None,
                username: None,
                first_name: None,
                last_name: None,
                enabled: None,
                exact: None,
            };

            let users = user_list(&client, TEST_TOKEN, &params).await.unwrap();
            assert_eq!(users.len(), 2);
            assert_eq!(users[0].username.as_deref(), Some("user1"));
        }

        #[tokio::test]
        async fn test_user_list_with_search() {
            let mock_server = MockServer::start().await;

            let expected_users = vec![UserRepresentation {
                id: Some("1".to_string()),
                username: Some("john".to_string()),
                ..Default::default()
            }];

            Mock::given(method("GET"))
                .and(path("/admin/realms/master/users"))
                .and(query_param("search", "john"))
                .respond_with(ResponseTemplate::new(200).set_body_json(&expected_users))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = UserListParams {
                realm: "master".to_string(),
                search: Some("john".to_string()),
                first: None,
                max: None,
                email: None,
                username: None,
                first_name: None,
                last_name: None,
                enabled: None,
                exact: None,
            };

            let users = user_list(&client, TEST_TOKEN, &params).await.unwrap();
            assert_eq!(users.len(), 1);
        }

        #[tokio::test]
        async fn test_user_list_with_pagination() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path("/admin/realms/master/users"))
                .and(query_param("first", "10"))
                .and(query_param("max", "20"))
                .respond_with(ResponseTemplate::new(200).set_body_json::<Vec<UserRepresentation>>(vec![]))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = UserListParams {
                realm: "master".to_string(),
                search: None,
                first: Some(10),
                max: Some(20),
                email: None,
                username: None,
                first_name: None,
                last_name: None,
                enabled: None,
                exact: None,
            };

            let result = user_list(&client, TEST_TOKEN, &params).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_user_list_validation_fails() {
            let mock_server = MockServer::start().await;
            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = UserListParams {
                realm: "".to_string(),
                search: None,
                first: None,
                max: None,
                email: None,
                username: None,
                first_name: None,
                last_name: None,
                enabled: None,
                exact: None,
            };

            let result = user_list(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::BadRequest(_))));
        }
    }

    mod user_get_tests {
        use super::*;

        #[tokio::test]
        async fn test_user_get_success() {
            let mock_server = MockServer::start().await;

            let expected_user = UserRepresentation {
                id: Some("123".to_string()),
                username: Some("testuser".to_string()),
                email: Some("test@example.com".to_string()),
                ..Default::default()
            };

            Mock::given(method("GET"))
                .and(path("/admin/realms/master/users/123"))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .respond_with(ResponseTemplate::new(200).set_body_json(&expected_user))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = UserGetParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
            };

            let user = user_get(&client, TEST_TOKEN, &params).await.unwrap();
            assert_eq!(user.id.as_deref(), Some("123"));
            assert_eq!(user.username.as_deref(), Some("testuser"));
        }

        #[tokio::test]
        async fn test_user_get_not_found() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path("/admin/realms/master/users/999"))
                .respond_with(ResponseTemplate::new(404))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = UserGetParams {
                realm: "master".to_string(),
                user_id: "999".to_string(),
            };

            let result = user_get(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::NotFound)));
        }
    }

    mod user_create_tests {
        use super::*;

        #[tokio::test]
        async fn test_user_create_success() {
            let mock_server = MockServer::start().await;

            Mock::given(method("POST"))
                .and(path("/admin/realms/master/users"))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .respond_with(ResponseTemplate::new(201))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = UserCreateParams {
                realm: "master".to_string(),
                user: UserRepresentation {
                    username: Some("newuser".to_string()),
                    email: Some("new@example.com".to_string()),
                    enabled: Some(true),
                    ..Default::default()
                },
            };

            let result = user_create(&client, TEST_TOKEN, &params).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_user_create_conflict() {
            let mock_server = MockServer::start().await;

            Mock::given(method("POST"))
                .and(path("/admin/realms/master/users"))
                .respond_with(ResponseTemplate::new(409).set_body_json(serde_json::json!({
                    "errorMessage": "User exists with same username"
                })))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = UserCreateParams {
                realm: "master".to_string(),
                user: UserRepresentation {
                    username: Some("existinguser".to_string()),
                    ..Default::default()
                },
            };

            let result = user_create(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::Conflict(_))));
        }
    }

    mod user_update_tests {
        use super::*;

        #[tokio::test]
        async fn test_user_update_success() {
            let mock_server = MockServer::start().await;

            Mock::given(method("PUT"))
                .and(path("/admin/realms/master/users/123"))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .respond_with(ResponseTemplate::new(204))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = UserUpdateParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                user: UserRepresentation {
                    email: Some("updated@example.com".to_string()),
                    ..Default::default()
                },
            };

            let result = user_update(&client, TEST_TOKEN, &params).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_user_update_not_found() {
            let mock_server = MockServer::start().await;

            Mock::given(method("PUT"))
                .and(path("/admin/realms/master/users/999"))
                .respond_with(ResponseTemplate::new(404))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = UserUpdateParams {
                realm: "master".to_string(),
                user_id: "999".to_string(),
                user: UserRepresentation::default(),
            };

            let result = user_update(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::NotFound)));
        }
    }

    mod user_delete_tests {
        use super::*;

        #[tokio::test]
        async fn test_user_delete_success() {
            let mock_server = MockServer::start().await;

            Mock::given(method("DELETE"))
                .and(path("/admin/realms/master/users/123"))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .respond_with(ResponseTemplate::new(204))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = UserDeleteParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
            };

            let result = user_delete(&client, TEST_TOKEN, &params).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_user_delete_not_found() {
            let mock_server = MockServer::start().await;

            Mock::given(method("DELETE"))
                .and(path("/admin/realms/master/users/999"))
                .respond_with(ResponseTemplate::new(404))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).unwrap();
            let params = UserDeleteParams {
                realm: "master".to_string(),
                user_id: "999".to_string(),
            };

            let result = user_delete(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::NotFound)));
        }
    }
}
