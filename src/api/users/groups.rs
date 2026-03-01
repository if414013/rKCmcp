//! User group membership management tools for Keycloak Admin REST API.
//!
//! This module provides tools for managing user group memberships including:
//! - Listing groups a user belongs to
//! - Adding a user to a group
//! - Removing a user from a group
//! - Counting groups a user belongs to

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::api::{ApiError, KeycloakClient};

/// Represents a group in Keycloak.
///
/// This is a simplified representation focused on group membership operations.
#[derive(Debug, Clone, Serialize, Deserialize, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GroupRepresentation {
    /// Unique identifier for the group (UUID)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Group name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Group path (hierarchical path like "/parent/child")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    /// Subgroups (child groups)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_groups: Option<Vec<GroupRepresentation>>,
}

/// Parameters for listing groups a user belongs to.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct UserGroupsListParams {
    /// The realm name
    pub realm: String,
    /// The user ID
    pub user_id: String,
    /// Optional brief representation (less detail)
    #[serde(default)]
    pub brief_representation: Option<bool>,
    /// Optional first result (pagination)
    #[serde(default)]
    pub first: Option<i32>,
    /// Optional max results (pagination)
    #[serde(default)]
    pub max: Option<i32>,
}

impl UserGroupsListParams {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.realm.trim().is_empty() {
            return Err(ApiError::BadRequest("realm is required".to_string()));
        }
        if self.user_id.trim().is_empty() {
            return Err(ApiError::BadRequest("user_id is required".to_string()));
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

        if let Some(brief) = self.brief_representation {
            params.push(format!("briefRepresentation={}", brief));
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

/// Parameters for adding a user to a group.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct UserGroupAddParams {
    /// The realm name
    pub realm: String,
    /// The user ID
    pub user_id: String,
    /// The group ID to add the user to
    pub group_id: String,
}

impl UserGroupAddParams {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.realm.trim().is_empty() {
            return Err(ApiError::BadRequest("realm is required".to_string()));
        }
        if self.user_id.trim().is_empty() {
            return Err(ApiError::BadRequest("user_id is required".to_string()));
        }
        if self.group_id.trim().is_empty() {
            return Err(ApiError::BadRequest("group_id is required".to_string()));
        }
        Ok(())
    }
}

/// Parameters for removing a user from a group.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct UserGroupRemoveParams {
    /// The realm name
    pub realm: String,
    /// The user ID
    pub user_id: String,
    /// The group ID to remove the user from
    pub group_id: String,
}

impl UserGroupRemoveParams {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.realm.trim().is_empty() {
            return Err(ApiError::BadRequest("realm is required".to_string()));
        }
        if self.user_id.trim().is_empty() {
            return Err(ApiError::BadRequest("user_id is required".to_string()));
        }
        if self.group_id.trim().is_empty() {
            return Err(ApiError::BadRequest("group_id is required".to_string()));
        }
        Ok(())
    }
}

/// Parameters for counting groups a user belongs to.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct UserGroupCountParams {
    /// The realm name
    pub realm: String,
    /// The user ID
    pub user_id: String,
}

impl UserGroupCountParams {
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

/// List groups that a user belongs to.
///
/// Returns the list of groups the user is a member of.
///
/// # Endpoint
/// `GET /admin/realms/{realm}/users/{id}/groups`
pub async fn user_groups_list(
    client: &KeycloakClient,
    token: &str,
    params: &UserGroupsListParams,
) -> Result<Vec<GroupRepresentation>, ApiError> {
    params.validate()?;

    let query = params.build_query_string();
    let path = format!(
        "/admin/realms/{}/users/{}/groups{}",
        params.realm, params.user_id, query
    );

    client.get(&path, token).await
}

/// Add a user to a group.
///
/// Makes the user a member of the specified group.
///
/// # Endpoint
/// `PUT /admin/realms/{realm}/users/{id}/groups/{groupId}`
pub async fn user_group_add(
    client: &KeycloakClient,
    token: &str,
    params: &UserGroupAddParams,
) -> Result<(), ApiError> {
    params.validate()?;

    let path = format!(
        "/admin/realms/{}/users/{}/groups/{}",
        params.realm, params.user_id, params.group_id
    );
    // Keycloak expects an empty body for this PUT request
    client.put(&path, token, &serde_json::Value::Null).await
}

/// Remove a user from a group.
///
/// Removes the user's membership from the specified group.
///
/// # Endpoint
/// `DELETE /admin/realms/{realm}/users/{id}/groups/{groupId}`
pub async fn user_group_remove(
    client: &KeycloakClient,
    token: &str,
    params: &UserGroupRemoveParams,
) -> Result<(), ApiError> {
    params.validate()?;

    let path = format!(
        "/admin/realms/{}/users/{}/groups/{}",
        params.realm, params.user_id, params.group_id
    );
    client.delete(&path, token).await
}

/// Count the number of groups a user belongs to.
///
/// Returns a count of groups the user is a member of.
///
/// # Endpoint
/// `GET /admin/realms/{realm}/users/{id}/groups/count`
pub async fn user_group_count(
    client: &KeycloakClient,
    token: &str,
    params: &UserGroupCountParams,
) -> Result<i64, ApiError> {
    params.validate()?;

    let path = format!(
        "/admin/realms/{}/users/{}/groups/count",
        params.realm, params.user_id
    );

    // Keycloak returns a JSON object like {"count": 5}
    #[derive(Deserialize)]
    struct CountResponse {
        count: i64,
    }

    let response: CountResponse = client.get(&path, token).await?;
    Ok(response.count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{header, method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    const TEST_TOKEN: &str = "test-bearer-token";

    mod user_groups_list_params_tests {
        use super::*;

        #[test]
        fn test_validate_success() {
            let params = UserGroupsListParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                brief_representation: None,
                first: None,
                max: None,
            };
            assert!(params.validate().is_ok());
        }

        #[test]
        fn test_validate_empty_realm() {
            let params = UserGroupsListParams {
                realm: "".to_string(),
                user_id: "123".to_string(),
                brief_representation: None,
                first: None,
                max: None,
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_whitespace_realm() {
            let params = UserGroupsListParams {
                realm: "   ".to_string(),
                user_id: "123".to_string(),
                brief_representation: None,
                first: None,
                max: None,
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_user_id() {
            let params = UserGroupsListParams {
                realm: "master".to_string(),
                user_id: "".to_string(),
                brief_representation: None,
                first: None,
                max: None,
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_negative_first() {
            let params = UserGroupsListParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                brief_representation: None,
                first: Some(-1),
                max: None,
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_negative_max() {
            let params = UserGroupsListParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                brief_representation: None,
                first: None,
                max: Some(-1),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_build_query_string_empty() {
            let params = UserGroupsListParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                brief_representation: None,
                first: None,
                max: None,
            };
            assert_eq!(params.build_query_string(), "");
        }

        #[test]
        fn test_build_query_string_with_pagination() {
            let params = UserGroupsListParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                brief_representation: None,
                first: Some(10),
                max: Some(20),
            };
            let query = params.build_query_string();
            assert!(query.contains("first=10"));
            assert!(query.contains("max=20"));
        }

        #[test]
        fn test_build_query_string_with_brief() {
            let params = UserGroupsListParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                brief_representation: Some(true),
                first: None,
                max: None,
            };
            let query = params.build_query_string();
            assert!(query.contains("briefRepresentation=true"));
        }

        #[test]
        fn test_build_query_string_with_all_params() {
            let params = UserGroupsListParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                brief_representation: Some(false),
                first: Some(5),
                max: Some(15),
            };
            let query = params.build_query_string();
            assert!(query.contains("briefRepresentation=false"));
            assert!(query.contains("first=5"));
            assert!(query.contains("max=15"));
        }
    }

    mod user_group_add_params_tests {
        use super::*;

        #[test]
        fn test_validate_success() {
            let params = UserGroupAddParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                group_id: "456".to_string(),
            };
            assert!(params.validate().is_ok());
        }

        #[test]
        fn test_validate_empty_realm() {
            let params = UserGroupAddParams {
                realm: "".to_string(),
                user_id: "123".to_string(),
                group_id: "456".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_user_id() {
            let params = UserGroupAddParams {
                realm: "master".to_string(),
                user_id: "".to_string(),
                group_id: "456".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_group_id() {
            let params = UserGroupAddParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                group_id: "".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }
    }

    mod user_group_remove_params_tests {
        use super::*;

        #[test]
        fn test_validate_success() {
            let params = UserGroupRemoveParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                group_id: "456".to_string(),
            };
            assert!(params.validate().is_ok());
        }

        #[test]
        fn test_validate_empty_realm() {
            let params = UserGroupRemoveParams {
                realm: "".to_string(),
                user_id: "123".to_string(),
                group_id: "456".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_user_id() {
            let params = UserGroupRemoveParams {
                realm: "master".to_string(),
                user_id: "".to_string(),
                group_id: "456".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_group_id() {
            let params = UserGroupRemoveParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                group_id: "".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }
    }

    mod user_group_count_params_tests {
        use super::*;

        #[test]
        fn test_validate_success() {
            let params = UserGroupCountParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
            };
            assert!(params.validate().is_ok());
        }

        #[test]
        fn test_validate_empty_realm() {
            let params = UserGroupCountParams {
                realm: "".to_string(),
                user_id: "123".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_user_id() {
            let params = UserGroupCountParams {
                realm: "master".to_string(),
                user_id: "".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }
    }

    mod user_groups_list_tests {
        use super::*;

        #[tokio::test]
        async fn test_groups_list_success() {
            let mock_server = MockServer::start().await;

            let expected_groups = vec![
                GroupRepresentation {
                    id: Some("group-1".to_string()),
                    name: Some("Developers".to_string()),
                    path: Some("/Developers".to_string()),
                    sub_groups: None,
                },
                GroupRepresentation {
                    id: Some("group-2".to_string()),
                    name: Some("Admins".to_string()),
                    path: Some("/Admins".to_string()),
                    sub_groups: None,
                },
            ];

            Mock::given(method("GET"))
                .and(path("/admin/realms/master/users/123/groups"))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .respond_with(ResponseTemplate::new(200).set_body_json(&expected_groups))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserGroupsListParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                brief_representation: None,
                first: None,
                max: None,
            };

            let groups = user_groups_list(&client, TEST_TOKEN, &params)
                .await
                .expect("Failed to list groups");
            assert_eq!(groups.len(), 2);
            assert_eq!(groups[0].name.as_deref(), Some("Developers"));
            assert_eq!(groups[1].name.as_deref(), Some("Admins"));
        }

        #[tokio::test]
        async fn test_groups_list_empty() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path("/admin/realms/master/users/123/groups"))
                .respond_with(
                    ResponseTemplate::new(200).set_body_json::<Vec<GroupRepresentation>>(vec![]),
                )
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserGroupsListParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                brief_representation: None,
                first: None,
                max: None,
            };

            let groups = user_groups_list(&client, TEST_TOKEN, &params)
                .await
                .expect("Failed to list groups");
            assert!(groups.is_empty());
        }

        #[tokio::test]
        async fn test_groups_list_with_pagination() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path("/admin/realms/master/users/123/groups"))
                .and(query_param("first", "10"))
                .and(query_param("max", "20"))
                .respond_with(
                    ResponseTemplate::new(200).set_body_json::<Vec<GroupRepresentation>>(vec![]),
                )
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserGroupsListParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                brief_representation: None,
                first: Some(10),
                max: Some(20),
            };

            let result = user_groups_list(&client, TEST_TOKEN, &params).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_groups_list_with_brief() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path("/admin/realms/master/users/123/groups"))
                .and(query_param("briefRepresentation", "true"))
                .respond_with(
                    ResponseTemplate::new(200).set_body_json::<Vec<GroupRepresentation>>(vec![]),
                )
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserGroupsListParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                brief_representation: Some(true),
                first: None,
                max: None,
            };

            let result = user_groups_list(&client, TEST_TOKEN, &params).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_groups_list_user_not_found() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path("/admin/realms/master/users/999/groups"))
                .respond_with(ResponseTemplate::new(404))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserGroupsListParams {
                realm: "master".to_string(),
                user_id: "999".to_string(),
                brief_representation: None,
                first: None,
                max: None,
            };

            let result = user_groups_list(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::NotFound)));
        }

        #[tokio::test]
        async fn test_groups_list_validation_fails() {
            let mock_server = MockServer::start().await;
            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserGroupsListParams {
                realm: "".to_string(),
                user_id: "123".to_string(),
                brief_representation: None,
                first: None,
                max: None,
            };

            let result = user_groups_list(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::BadRequest(_))));
        }
    }

    mod user_group_add_tests {
        use super::*;

        #[tokio::test]
        async fn test_group_add_success() {
            let mock_server = MockServer::start().await;

            Mock::given(method("PUT"))
                .and(path("/admin/realms/master/users/123/groups/456"))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .respond_with(ResponseTemplate::new(204))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserGroupAddParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                group_id: "456".to_string(),
            };

            let result = user_group_add(&client, TEST_TOKEN, &params).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_group_add_user_not_found() {
            let mock_server = MockServer::start().await;

            Mock::given(method("PUT"))
                .and(path("/admin/realms/master/users/999/groups/456"))
                .respond_with(ResponseTemplate::new(404))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserGroupAddParams {
                realm: "master".to_string(),
                user_id: "999".to_string(),
                group_id: "456".to_string(),
            };

            let result = user_group_add(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::NotFound)));
        }

        #[tokio::test]
        async fn test_group_add_group_not_found() {
            let mock_server = MockServer::start().await;

            Mock::given(method("PUT"))
                .and(path("/admin/realms/master/users/123/groups/999"))
                .respond_with(ResponseTemplate::new(404))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserGroupAddParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                group_id: "999".to_string(),
            };

            let result = user_group_add(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::NotFound)));
        }

        #[tokio::test]
        async fn test_group_add_validation_fails() {
            let mock_server = MockServer::start().await;
            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserGroupAddParams {
                realm: "".to_string(),
                user_id: "123".to_string(),
                group_id: "456".to_string(),
            };

            let result = user_group_add(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::BadRequest(_))));
        }
    }

    mod user_group_remove_tests {
        use super::*;

        #[tokio::test]
        async fn test_group_remove_success() {
            let mock_server = MockServer::start().await;

            Mock::given(method("DELETE"))
                .and(path("/admin/realms/master/users/123/groups/456"))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .respond_with(ResponseTemplate::new(204))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserGroupRemoveParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
                group_id: "456".to_string(),
            };

            let result = user_group_remove(&client, TEST_TOKEN, &params).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_group_remove_user_not_found() {
            let mock_server = MockServer::start().await;

            Mock::given(method("DELETE"))
                .and(path("/admin/realms/master/users/999/groups/456"))
                .respond_with(ResponseTemplate::new(404))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserGroupRemoveParams {
                realm: "master".to_string(),
                user_id: "999".to_string(),
                group_id: "456".to_string(),
            };

            let result = user_group_remove(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::NotFound)));
        }

        #[tokio::test]
        async fn test_group_remove_validation_fails() {
            let mock_server = MockServer::start().await;
            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserGroupRemoveParams {
                realm: "master".to_string(),
                user_id: "".to_string(),
                group_id: "456".to_string(),
            };

            let result = user_group_remove(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::BadRequest(_))));
        }
    }

    mod user_group_count_tests {
        use super::*;

        #[tokio::test]
        async fn test_group_count_success() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path("/admin/realms/master/users/123/groups/count"))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .respond_with(
                    ResponseTemplate::new(200).set_body_json(&serde_json::json!({"count": 5})),
                )
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserGroupCountParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
            };

            let count = user_group_count(&client, TEST_TOKEN, &params)
                .await
                .expect("Failed to count groups");
            assert_eq!(count, 5);
        }

        #[tokio::test]
        async fn test_group_count_zero() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path("/admin/realms/master/users/123/groups/count"))
                .respond_with(
                    ResponseTemplate::new(200).set_body_json(&serde_json::json!({"count": 0})),
                )
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserGroupCountParams {
                realm: "master".to_string(),
                user_id: "123".to_string(),
            };

            let count = user_group_count(&client, TEST_TOKEN, &params)
                .await
                .expect("Failed to count groups");
            assert_eq!(count, 0);
        }

        #[tokio::test]
        async fn test_group_count_user_not_found() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path("/admin/realms/master/users/999/groups/count"))
                .respond_with(ResponseTemplate::new(404))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserGroupCountParams {
                realm: "master".to_string(),
                user_id: "999".to_string(),
            };

            let result = user_group_count(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::NotFound)));
        }

        #[tokio::test]
        async fn test_group_count_validation_fails() {
            let mock_server = MockServer::start().await;
            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = UserGroupCountParams {
                realm: "".to_string(),
                user_id: "123".to_string(),
            };

            let result = user_group_count(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::BadRequest(_))));
        }
    }
}
