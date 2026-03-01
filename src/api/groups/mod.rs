//! Groups API module for Keycloak Admin REST API.
//!
//! Provides CRUD operations for managing groups in Keycloak realms.

pub mod role_mappings;
pub mod types;

pub use types::GroupRepresentation;

use schemars::JsonSchema;
use serde::Deserialize;

use crate::api::users::types::UserRepresentation;
use crate::api::{ApiError, KeycloakClient};

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GroupListParams {
    pub realm: String,
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub first: Option<i32>,
    #[serde(default)]
    pub max: Option<i32>,
    #[serde(default)]
    pub brief_representation: Option<bool>,
    #[serde(default)]
    pub exact: Option<bool>,
    #[serde(default)]
    pub q: Option<String>,
}

impl GroupListParams {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.realm.trim().is_empty() {
            return Err(ApiError::BadRequest("realm is required".to_string()));
        }
        if let Some(first) = self.first {
            if first < 0 {
                return Err(ApiError::BadRequest(
                    "first must be non-negative".to_string(),
                ));
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
        if let Some(first) = self.first {
            params.push(format!("first={}", first));
        }
        if let Some(max) = self.max {
            params.push(format!("max={}", max));
        }
        if let Some(brief) = self.brief_representation {
            params.push(format!("briefRepresentation={}", brief));
        }
        if let Some(exact) = self.exact {
            params.push(format!("exact={}", exact));
        }
        if let Some(ref q) = self.q {
            params.push(format!("q={}", urlencoding::encode(q)));
        }

        if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        }
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GroupGetParams {
    pub realm: String,
    pub group_id: String,
}

impl GroupGetParams {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.realm.trim().is_empty() {
            return Err(ApiError::BadRequest("realm is required".to_string()));
        }
        if self.group_id.trim().is_empty() {
            return Err(ApiError::BadRequest("group_id is required".to_string()));
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GroupCreateParams {
    pub realm: String,
    pub group: GroupRepresentation,
}

impl GroupCreateParams {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.realm.trim().is_empty() {
            return Err(ApiError::BadRequest("realm is required".to_string()));
        }
        if self.group.name.as_ref().is_none_or(|n| n.trim().is_empty()) {
            return Err(ApiError::BadRequest("group name is required".to_string()));
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GroupUpdateParams {
    pub realm: String,
    pub group_id: String,
    pub group: GroupRepresentation,
}

impl GroupUpdateParams {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.realm.trim().is_empty() {
            return Err(ApiError::BadRequest("realm is required".to_string()));
        }
        if self.group_id.trim().is_empty() {
            return Err(ApiError::BadRequest("group_id is required".to_string()));
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GroupDeleteParams {
    pub realm: String,
    pub group_id: String,
}

impl GroupDeleteParams {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.realm.trim().is_empty() {
            return Err(ApiError::BadRequest("realm is required".to_string()));
        }
        if self.group_id.trim().is_empty() {
            return Err(ApiError::BadRequest("group_id is required".to_string()));
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GroupCountParams {
    pub realm: String,
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub top: Option<bool>,
}

impl GroupCountParams {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.realm.trim().is_empty() {
            return Err(ApiError::BadRequest("realm is required".to_string()));
        }
        Ok(())
    }

    pub fn build_query_string(&self) -> String {
        let mut params = Vec::new();

        if let Some(ref search) = self.search {
            params.push(format!("search={}", urlencoding::encode(search)));
        }
        if let Some(top) = self.top {
            params.push(format!("top={}", top));
        }

        if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        }
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GroupMembersListParams {
    pub realm: String,
    pub group_id: String,
    #[serde(default)]
    pub first: Option<i32>,
    #[serde(default)]
    pub max: Option<i32>,
    #[serde(default)]
    pub brief_representation: Option<bool>,
}

impl GroupMembersListParams {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.realm.trim().is_empty() {
            return Err(ApiError::BadRequest("realm is required".to_string()));
        }
        if self.group_id.trim().is_empty() {
            return Err(ApiError::BadRequest("group_id is required".to_string()));
        }
        if let Some(first) = self.first {
            if first < 0 {
                return Err(ApiError::BadRequest(
                    "first must be non-negative".to_string(),
                ));
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

        if let Some(first) = self.first {
            params.push(format!("first={}", first));
        }
        if let Some(max) = self.max {
            params.push(format!("max={}", max));
        }
        if let Some(brief) = self.brief_representation {
            params.push(format!("briefRepresentation={}", brief));
        }

        if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        }
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GroupChildrenListParams {
    pub realm: String,
    pub group_id: String,
}

impl GroupChildrenListParams {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.realm.trim().is_empty() {
            return Err(ApiError::BadRequest("realm is required".to_string()));
        }
        if self.group_id.trim().is_empty() {
            return Err(ApiError::BadRequest("group_id is required".to_string()));
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GroupChildCreateParams {
    pub realm: String,
    pub group_id: String,
    pub group: GroupRepresentation,
}

impl GroupChildCreateParams {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.realm.trim().is_empty() {
            return Err(ApiError::BadRequest("realm is required".to_string()));
        }
        if self.group_id.trim().is_empty() {
            return Err(ApiError::BadRequest("group_id is required".to_string()));
        }
        if self.group.name.as_ref().is_none_or(|n| n.trim().is_empty()) {
            return Err(ApiError::BadRequest("group name is required".to_string()));
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GroupSetParentParams {
    pub realm: String,
    pub child_id: String,
    pub parent_id: String,
}

impl GroupSetParentParams {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.realm.trim().is_empty() {
            return Err(ApiError::BadRequest("realm is required".to_string()));
        }
        if self.child_id.trim().is_empty() {
            return Err(ApiError::BadRequest("child_id is required".to_string()));
        }
        if self.parent_id.trim().is_empty() {
            return Err(ApiError::BadRequest("parent_id is required".to_string()));
        }
        Ok(())
    }
}

/// GET /admin/realms/{realm}/groups
pub async fn group_list(
    client: &KeycloakClient,
    token: &str,
    params: &GroupListParams,
) -> Result<Vec<GroupRepresentation>, ApiError> {
    params.validate()?;

    let query = params.build_query_string();
    let path = format!("/admin/realms/{}/groups{}", params.realm, query);

    client.get(&path, token).await
}

/// GET /admin/realms/{realm}/groups/{id}
pub async fn group_get(
    client: &KeycloakClient,
    token: &str,
    params: &GroupGetParams,
) -> Result<GroupRepresentation, ApiError> {
    params.validate()?;

    let path = format!(
        "/admin/realms/{}/groups/{}",
        params.realm,
        urlencoding::encode(&params.group_id)
    );

    client.get(&path, token).await
}

/// POST /admin/realms/{realm}/groups
pub async fn group_create(
    client: &KeycloakClient,
    token: &str,
    params: &GroupCreateParams,
) -> Result<(), ApiError> {
    params.validate()?;

    let path = format!("/admin/realms/{}/groups", params.realm);
    client.post_no_response(&path, token, &params.group).await
}

/// PUT /admin/realms/{realm}/groups/{id}
pub async fn group_update(
    client: &KeycloakClient,
    token: &str,
    params: &GroupUpdateParams,
) -> Result<(), ApiError> {
    params.validate()?;

    let path = format!(
        "/admin/realms/{}/groups/{}",
        params.realm,
        urlencoding::encode(&params.group_id)
    );

    client.put(&path, token, &params.group).await
}

/// DELETE /admin/realms/{realm}/groups/{id}
pub async fn group_delete(
    client: &KeycloakClient,
    token: &str,
    params: &GroupDeleteParams,
) -> Result<(), ApiError> {
    params.validate()?;

    let path = format!(
        "/admin/realms/{}/groups/{}",
        params.realm,
        urlencoding::encode(&params.group_id)
    );

    client.delete(&path, token).await
}

/// GET /admin/realms/{realm}/groups/count
pub async fn group_count(
    client: &KeycloakClient,
    token: &str,
    params: &GroupCountParams,
) -> Result<i64, ApiError> {
    params.validate()?;

    let query = params.build_query_string();
    let path = format!("/admin/realms/{}/groups/count{}", params.realm, query);

    #[derive(Deserialize)]
    struct CountResponse {
        count: i64,
    }

    let response: CountResponse = client.get(&path, token).await?;
    Ok(response.count)
}

/// GET /admin/realms/{realm}/groups/{id}/members
pub async fn group_members_list(
    client: &KeycloakClient,
    token: &str,
    params: &GroupMembersListParams,
) -> Result<Vec<UserRepresentation>, ApiError> {
    params.validate()?;

    let query = params.build_query_string();
    let path = format!(
        "/admin/realms/{}/groups/{}/members{}",
        params.realm,
        urlencoding::encode(&params.group_id),
        query
    );

    client.get(&path, token).await
}

/// GET /admin/realms/{realm}/groups/{id}/children
pub async fn group_children_list(
    client: &KeycloakClient,
    token: &str,
    params: &GroupChildrenListParams,
) -> Result<Vec<GroupRepresentation>, ApiError> {
    params.validate()?;

    let path = format!(
        "/admin/realms/{}/groups/{}/children",
        params.realm,
        urlencoding::encode(&params.group_id)
    );

    client.get(&path, token).await
}

/// POST /admin/realms/{realm}/groups/{id}/children
pub async fn group_child_create(
    client: &KeycloakClient,
    token: &str,
    params: &GroupChildCreateParams,
) -> Result<(), ApiError> {
    params.validate()?;

    let path = format!(
        "/admin/realms/{}/groups/{}/children",
        params.realm,
        urlencoding::encode(&params.group_id)
    );

    client.post_no_response(&path, token, &params.group).await
}

/// POST /admin/realms/{realm}/groups/{child-id}/parent
pub async fn group_set_parent(
    client: &KeycloakClient,
    token: &str,
    params: &GroupSetParentParams,
) -> Result<(), ApiError> {
    params.validate()?;

    let path = format!(
        "/admin/realms/{}/groups/{}/parent",
        params.realm,
        urlencoding::encode(&params.child_id)
    );

    #[derive(serde::Serialize)]
    struct ParentIdPayload {
        id: String,
    }

    let payload = ParentIdPayload {
        id: params.parent_id.clone(),
    };

    client.post_no_response(&path, token, &payload).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{body_json, header, method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    const TEST_TOKEN: &str = "test-bearer-token";

    fn sample_group() -> GroupRepresentation {
        GroupRepresentation {
            id: Some("group-123".to_string()),
            name: Some("Developers".to_string()),
            path: Some("/Developers".to_string()),
            ..Default::default()
        }
    }

    mod group_list_params_tests {
        use super::*;

        #[test]
        fn test_validate_success() {
            let params = GroupListParams {
                realm: "master".to_string(),
                search: None,
                first: Some(0),
                max: Some(10),
                brief_representation: None,
                exact: None,
                q: None,
            };
            assert!(params.validate().is_ok());
        }

        #[test]
        fn test_validate_empty_realm() {
            let params = GroupListParams {
                realm: "".to_string(),
                search: None,
                first: None,
                max: None,
                brief_representation: None,
                exact: None,
                q: None,
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_whitespace_realm() {
            let params = GroupListParams {
                realm: "   ".to_string(),
                search: None,
                first: None,
                max: None,
                brief_representation: None,
                exact: None,
                q: None,
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_negative_first() {
            let params = GroupListParams {
                realm: "master".to_string(),
                search: None,
                first: Some(-1),
                max: None,
                brief_representation: None,
                exact: None,
                q: None,
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_negative_max() {
            let params = GroupListParams {
                realm: "master".to_string(),
                search: None,
                first: None,
                max: Some(-1),
                brief_representation: None,
                exact: None,
                q: None,
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_build_query_string_empty() {
            let params = GroupListParams {
                realm: "master".to_string(),
                search: None,
                first: None,
                max: None,
                brief_representation: None,
                exact: None,
                q: None,
            };
            assert_eq!(params.build_query_string(), "");
        }

        #[test]
        fn test_build_query_string_with_pagination() {
            let params = GroupListParams {
                realm: "master".to_string(),
                search: None,
                first: Some(10),
                max: Some(20),
                brief_representation: None,
                exact: None,
                q: None,
            };
            let query = params.build_query_string();
            assert!(query.contains("first=10"));
            assert!(query.contains("max=20"));
        }

        #[test]
        fn test_build_query_string_with_search() {
            let params = GroupListParams {
                realm: "master".to_string(),
                search: Some("dev".to_string()),
                first: None,
                max: None,
                brief_representation: None,
                exact: None,
                q: None,
            };
            let query = params.build_query_string();
            assert!(query.contains("search=dev"));
        }

        #[test]
        fn test_build_query_string_with_special_chars() {
            let params = GroupListParams {
                realm: "master".to_string(),
                search: Some("my group".to_string()),
                first: None,
                max: None,
                brief_representation: None,
                exact: None,
                q: None,
            };
            let query = params.build_query_string();
            assert!(query.contains("search=my%20group"));
        }

        #[test]
        fn test_build_query_string_with_all_params() {
            let params = GroupListParams {
                realm: "master".to_string(),
                search: Some("test".to_string()),
                first: Some(5),
                max: Some(15),
                brief_representation: Some(true),
                exact: Some(false),
                q: Some("attr:value".to_string()),
            };
            let query = params.build_query_string();
            assert!(query.contains("search=test"));
            assert!(query.contains("first=5"));
            assert!(query.contains("max=15"));
            assert!(query.contains("briefRepresentation=true"));
            assert!(query.contains("exact=false"));
            assert!(query.contains("q=attr%3Avalue"));
        }
    }

    mod group_get_params_tests {
        use super::*;

        #[test]
        fn test_validate_success() {
            let params = GroupGetParams {
                realm: "master".to_string(),
                group_id: "123".to_string(),
            };
            assert!(params.validate().is_ok());
        }

        #[test]
        fn test_validate_empty_realm() {
            let params = GroupGetParams {
                realm: "".to_string(),
                group_id: "123".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_group_id() {
            let params = GroupGetParams {
                realm: "master".to_string(),
                group_id: "".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }
    }

    mod group_create_params_tests {
        use super::*;

        #[test]
        fn test_validate_success() {
            let params = GroupCreateParams {
                realm: "master".to_string(),
                group: GroupRepresentation {
                    name: Some("NewGroup".to_string()),
                    ..Default::default()
                },
            };
            assert!(params.validate().is_ok());
        }

        #[test]
        fn test_validate_empty_realm() {
            let params = GroupCreateParams {
                realm: "".to_string(),
                group: GroupRepresentation {
                    name: Some("NewGroup".to_string()),
                    ..Default::default()
                },
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_missing_name() {
            let params = GroupCreateParams {
                realm: "master".to_string(),
                group: GroupRepresentation::default(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_name() {
            let params = GroupCreateParams {
                realm: "master".to_string(),
                group: GroupRepresentation {
                    name: Some("".to_string()),
                    ..Default::default()
                },
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }
    }

    mod group_update_params_tests {
        use super::*;

        #[test]
        fn test_validate_success() {
            let params = GroupUpdateParams {
                realm: "master".to_string(),
                group_id: "123".to_string(),
                group: GroupRepresentation {
                    name: Some("Updated".to_string()),
                    ..Default::default()
                },
            };
            assert!(params.validate().is_ok());
        }

        #[test]
        fn test_validate_empty_realm() {
            let params = GroupUpdateParams {
                realm: "".to_string(),
                group_id: "123".to_string(),
                group: GroupRepresentation::default(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_group_id() {
            let params = GroupUpdateParams {
                realm: "master".to_string(),
                group_id: "".to_string(),
                group: GroupRepresentation::default(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }
    }

    mod group_delete_params_tests {
        use super::*;

        #[test]
        fn test_validate_success() {
            let params = GroupDeleteParams {
                realm: "master".to_string(),
                group_id: "123".to_string(),
            };
            assert!(params.validate().is_ok());
        }

        #[test]
        fn test_validate_empty_realm() {
            let params = GroupDeleteParams {
                realm: "".to_string(),
                group_id: "123".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_group_id() {
            let params = GroupDeleteParams {
                realm: "master".to_string(),
                group_id: "".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }
    }

    mod group_count_params_tests {
        use super::*;

        #[test]
        fn test_validate_success() {
            let params = GroupCountParams {
                realm: "master".to_string(),
                search: None,
                top: None,
            };
            assert!(params.validate().is_ok());
        }

        #[test]
        fn test_validate_empty_realm() {
            let params = GroupCountParams {
                realm: "".to_string(),
                search: None,
                top: None,
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_build_query_string_empty() {
            let params = GroupCountParams {
                realm: "master".to_string(),
                search: None,
                top: None,
            };
            assert_eq!(params.build_query_string(), "");
        }

        #[test]
        fn test_build_query_string_with_search() {
            let params = GroupCountParams {
                realm: "master".to_string(),
                search: Some("dev".to_string()),
                top: None,
            };
            let query = params.build_query_string();
            assert!(query.contains("search=dev"));
        }

        #[test]
        fn test_build_query_string_with_top() {
            let params = GroupCountParams {
                realm: "master".to_string(),
                search: None,
                top: Some(true),
            };
            let query = params.build_query_string();
            assert!(query.contains("top=true"));
        }
    }

    mod group_list_tests {
        use super::*;

        #[tokio::test]
        async fn test_group_list_success() {
            let mock_server = MockServer::start().await;

            let expected_groups = vec![sample_group()];

            Mock::given(method("GET"))
                .and(path("/admin/realms/master/groups"))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .respond_with(ResponseTemplate::new(200).set_body_json(&expected_groups))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupListParams {
                realm: "master".to_string(),
                search: None,
                first: None,
                max: None,
                brief_representation: None,
                exact: None,
                q: None,
            };

            let groups = group_list(&client, TEST_TOKEN, &params)
                .await
                .expect("Failed to list groups");
            assert_eq!(groups.len(), 1);
            assert_eq!(groups[0].name.as_deref(), Some("Developers"));
        }

        #[tokio::test]
        async fn test_group_list_empty() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path("/admin/realms/master/groups"))
                .respond_with(
                    ResponseTemplate::new(200).set_body_json::<Vec<GroupRepresentation>>(vec![]),
                )
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupListParams {
                realm: "master".to_string(),
                search: None,
                first: None,
                max: None,
                brief_representation: None,
                exact: None,
                q: None,
            };

            let groups = group_list(&client, TEST_TOKEN, &params)
                .await
                .expect("Failed to list groups");
            assert!(groups.is_empty());
        }

        #[tokio::test]
        async fn test_group_list_with_search() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path("/admin/realms/master/groups"))
                .and(query_param("search", "dev"))
                .respond_with(ResponseTemplate::new(200).set_body_json(vec![sample_group()]))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupListParams {
                realm: "master".to_string(),
                search: Some("dev".to_string()),
                first: None,
                max: None,
                brief_representation: None,
                exact: None,
                q: None,
            };

            let result = group_list(&client, TEST_TOKEN, &params).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_group_list_with_pagination() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path("/admin/realms/master/groups"))
                .and(query_param("first", "10"))
                .and(query_param("max", "20"))
                .respond_with(
                    ResponseTemplate::new(200).set_body_json::<Vec<GroupRepresentation>>(vec![]),
                )
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupListParams {
                realm: "master".to_string(),
                search: None,
                first: Some(10),
                max: Some(20),
                brief_representation: None,
                exact: None,
                q: None,
            };

            let result = group_list(&client, TEST_TOKEN, &params).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_group_list_unauthorized() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path("/admin/realms/master/groups"))
                .respond_with(ResponseTemplate::new(401))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupListParams {
                realm: "master".to_string(),
                search: None,
                first: None,
                max: None,
                brief_representation: None,
                exact: None,
                q: None,
            };

            let result = group_list(&client, "invalid-token", &params).await;
            assert!(matches!(result, Err(ApiError::Unauthorized)));
        }

        #[tokio::test]
        async fn test_group_list_forbidden() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path("/admin/realms/master/groups"))
                .respond_with(ResponseTemplate::new(403))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupListParams {
                realm: "master".to_string(),
                search: None,
                first: None,
                max: None,
                brief_representation: None,
                exact: None,
                q: None,
            };

            let result = group_list(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::Forbidden)));
        }

        #[tokio::test]
        async fn test_group_list_validation_fails() {
            let mock_server = MockServer::start().await;
            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupListParams {
                realm: "".to_string(),
                search: None,
                first: None,
                max: None,
                brief_representation: None,
                exact: None,
                q: None,
            };

            let result = group_list(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::BadRequest(_))));
        }
    }

    mod group_get_tests {
        use super::*;

        #[tokio::test]
        async fn test_group_get_success() {
            let mock_server = MockServer::start().await;

            let expected_group = sample_group();

            Mock::given(method("GET"))
                .and(path("/admin/realms/master/groups/group-123"))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .respond_with(ResponseTemplate::new(200).set_body_json(&expected_group))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupGetParams {
                realm: "master".to_string(),
                group_id: "group-123".to_string(),
            };

            let group = group_get(&client, TEST_TOKEN, &params)
                .await
                .expect("Failed to get group");
            assert_eq!(group.id.as_deref(), Some("group-123"));
            assert_eq!(group.name.as_deref(), Some("Developers"));
        }

        #[tokio::test]
        async fn test_group_get_not_found() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path("/admin/realms/master/groups/nonexistent"))
                .respond_with(ResponseTemplate::new(404))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupGetParams {
                realm: "master".to_string(),
                group_id: "nonexistent".to_string(),
            };

            let result = group_get(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::NotFound)));
        }

        #[tokio::test]
        async fn test_group_get_with_special_chars() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path("/admin/realms/master/groups/group%2Fwith%2Bspecial"))
                .respond_with(ResponseTemplate::new(200).set_body_json(sample_group()))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupGetParams {
                realm: "master".to_string(),
                group_id: "group/with+special".to_string(),
            };

            let result = group_get(&client, TEST_TOKEN, &params).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_group_get_validation_fails() {
            let mock_server = MockServer::start().await;
            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupGetParams {
                realm: "master".to_string(),
                group_id: "".to_string(),
            };

            let result = group_get(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::BadRequest(_))));
        }
    }

    mod group_create_tests {
        use super::*;

        #[tokio::test]
        async fn test_group_create_success() {
            let mock_server = MockServer::start().await;

            let new_group = GroupRepresentation {
                name: Some("NewGroup".to_string()),
                ..Default::default()
            };

            Mock::given(method("POST"))
                .and(path("/admin/realms/master/groups"))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .and(body_json(&new_group))
                .respond_with(ResponseTemplate::new(201))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupCreateParams {
                realm: "master".to_string(),
                group: new_group,
            };

            let result = group_create(&client, TEST_TOKEN, &params).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_group_create_conflict() {
            let mock_server = MockServer::start().await;

            Mock::given(method("POST"))
                .and(path("/admin/realms/master/groups"))
                .respond_with(ResponseTemplate::new(409).set_body_json(serde_json::json!({
                    "errorMessage": "Group exists with same name"
                })))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupCreateParams {
                realm: "master".to_string(),
                group: GroupRepresentation {
                    name: Some("ExistingGroup".to_string()),
                    ..Default::default()
                },
            };

            let result = group_create(&client, TEST_TOKEN, &params).await;
            match result {
                Err(ApiError::Conflict(msg)) => {
                    assert!(msg.contains("Group exists"));
                }
                _ => panic!("Expected Conflict error"),
            }
        }

        #[tokio::test]
        async fn test_group_create_validation_fails() {
            let mock_server = MockServer::start().await;
            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupCreateParams {
                realm: "master".to_string(),
                group: GroupRepresentation::default(),
            };

            let result = group_create(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::BadRequest(_))));
        }
    }

    mod group_update_tests {
        use super::*;

        #[tokio::test]
        async fn test_group_update_success() {
            let mock_server = MockServer::start().await;

            let updated_group = GroupRepresentation {
                id: Some("group-123".to_string()),
                name: Some("UpdatedGroup".to_string()),
                ..Default::default()
            };

            Mock::given(method("PUT"))
                .and(path("/admin/realms/master/groups/group-123"))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .and(body_json(&updated_group))
                .respond_with(ResponseTemplate::new(204))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupUpdateParams {
                realm: "master".to_string(),
                group_id: "group-123".to_string(),
                group: updated_group,
            };

            let result = group_update(&client, TEST_TOKEN, &params).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_group_update_not_found() {
            let mock_server = MockServer::start().await;

            Mock::given(method("PUT"))
                .and(path("/admin/realms/master/groups/nonexistent"))
                .respond_with(ResponseTemplate::new(404))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupUpdateParams {
                realm: "master".to_string(),
                group_id: "nonexistent".to_string(),
                group: GroupRepresentation::default(),
            };

            let result = group_update(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::NotFound)));
        }

        #[tokio::test]
        async fn test_group_update_validation_fails() {
            let mock_server = MockServer::start().await;
            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupUpdateParams {
                realm: "".to_string(),
                group_id: "123".to_string(),
                group: GroupRepresentation::default(),
            };

            let result = group_update(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::BadRequest(_))));
        }
    }

    mod group_delete_tests {
        use super::*;

        #[tokio::test]
        async fn test_group_delete_success() {
            let mock_server = MockServer::start().await;

            Mock::given(method("DELETE"))
                .and(path("/admin/realms/master/groups/group-123"))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .respond_with(ResponseTemplate::new(204))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupDeleteParams {
                realm: "master".to_string(),
                group_id: "group-123".to_string(),
            };

            let result = group_delete(&client, TEST_TOKEN, &params).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_group_delete_not_found() {
            let mock_server = MockServer::start().await;

            Mock::given(method("DELETE"))
                .and(path("/admin/realms/master/groups/nonexistent"))
                .respond_with(ResponseTemplate::new(404))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupDeleteParams {
                realm: "master".to_string(),
                group_id: "nonexistent".to_string(),
            };

            let result = group_delete(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::NotFound)));
        }

        #[tokio::test]
        async fn test_group_delete_validation_fails() {
            let mock_server = MockServer::start().await;
            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupDeleteParams {
                realm: "master".to_string(),
                group_id: "".to_string(),
            };

            let result = group_delete(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::BadRequest(_))));
        }
    }

    mod group_count_tests {
        use super::*;

        #[tokio::test]
        async fn test_group_count_success() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path("/admin/realms/master/groups/count"))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .respond_with(
                    ResponseTemplate::new(200).set_body_json(serde_json::json!({"count": 42})),
                )
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupCountParams {
                realm: "master".to_string(),
                search: None,
                top: None,
            };

            let count = group_count(&client, TEST_TOKEN, &params)
                .await
                .expect("Failed to count groups");
            assert_eq!(count, 42);
        }

        #[tokio::test]
        async fn test_group_count_zero() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path("/admin/realms/master/groups/count"))
                .respond_with(
                    ResponseTemplate::new(200).set_body_json(serde_json::json!({"count": 0})),
                )
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupCountParams {
                realm: "master".to_string(),
                search: None,
                top: None,
            };

            let count = group_count(&client, TEST_TOKEN, &params)
                .await
                .expect("Failed to count groups");
            assert_eq!(count, 0);
        }

        #[tokio::test]
        async fn test_group_count_with_search() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path("/admin/realms/master/groups/count"))
                .and(query_param("search", "dev"))
                .respond_with(
                    ResponseTemplate::new(200).set_body_json(serde_json::json!({"count": 5})),
                )
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupCountParams {
                realm: "master".to_string(),
                search: Some("dev".to_string()),
                top: None,
            };

            let count = group_count(&client, TEST_TOKEN, &params)
                .await
                .expect("Failed to count groups");
            assert_eq!(count, 5);
        }

        #[tokio::test]
        async fn test_group_count_with_top() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path("/admin/realms/master/groups/count"))
                .and(query_param("top", "true"))
                .respond_with(
                    ResponseTemplate::new(200).set_body_json(serde_json::json!({"count": 10})),
                )
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupCountParams {
                realm: "master".to_string(),
                search: None,
                top: Some(true),
            };

            let count = group_count(&client, TEST_TOKEN, &params)
                .await
                .expect("Failed to count groups");
            assert_eq!(count, 10);
        }

        #[tokio::test]
        async fn test_group_count_validation_fails() {
            let mock_server = MockServer::start().await;
            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupCountParams {
                realm: "".to_string(),
                search: None,
                top: None,
            };

            let result = group_count(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::BadRequest(_))));
        }
    }

    mod group_members_list_params_tests {
        use super::*;

        #[test]
        fn test_validate_success() {
            let params = GroupMembersListParams {
                realm: "master".to_string(),
                group_id: "group-123".to_string(),
                first: Some(0),
                max: Some(10),
                brief_representation: None,
            };
            assert!(params.validate().is_ok());
        }

        #[test]
        fn test_validate_empty_realm() {
            let params = GroupMembersListParams {
                realm: "".to_string(),
                group_id: "group-123".to_string(),
                first: None,
                max: None,
                brief_representation: None,
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_group_id() {
            let params = GroupMembersListParams {
                realm: "master".to_string(),
                group_id: "".to_string(),
                first: None,
                max: None,
                brief_representation: None,
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_negative_first() {
            let params = GroupMembersListParams {
                realm: "master".to_string(),
                group_id: "group-123".to_string(),
                first: Some(-1),
                max: None,
                brief_representation: None,
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_negative_max() {
            let params = GroupMembersListParams {
                realm: "master".to_string(),
                group_id: "group-123".to_string(),
                first: None,
                max: Some(-1),
                brief_representation: None,
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_build_query_string_empty() {
            let params = GroupMembersListParams {
                realm: "master".to_string(),
                group_id: "group-123".to_string(),
                first: None,
                max: None,
                brief_representation: None,
            };
            assert_eq!(params.build_query_string(), "");
        }

        #[test]
        fn test_build_query_string_with_pagination() {
            let params = GroupMembersListParams {
                realm: "master".to_string(),
                group_id: "group-123".to_string(),
                first: Some(10),
                max: Some(20),
                brief_representation: None,
            };
            let query = params.build_query_string();
            assert!(query.contains("first=10"));
            assert!(query.contains("max=20"));
        }

        #[test]
        fn test_build_query_string_with_brief_representation() {
            let params = GroupMembersListParams {
                realm: "master".to_string(),
                group_id: "group-123".to_string(),
                first: None,
                max: None,
                brief_representation: Some(true),
            };
            let query = params.build_query_string();
            assert!(query.contains("briefRepresentation=true"));
        }
    }

    mod group_children_list_params_tests {
        use super::*;

        #[test]
        fn test_validate_success() {
            let params = GroupChildrenListParams {
                realm: "master".to_string(),
                group_id: "group-123".to_string(),
            };
            assert!(params.validate().is_ok());
        }

        #[test]
        fn test_validate_empty_realm() {
            let params = GroupChildrenListParams {
                realm: "".to_string(),
                group_id: "group-123".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_group_id() {
            let params = GroupChildrenListParams {
                realm: "master".to_string(),
                group_id: "".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }
    }

    mod group_child_create_params_tests {
        use super::*;

        #[test]
        fn test_validate_success() {
            let params = GroupChildCreateParams {
                realm: "master".to_string(),
                group_id: "parent-123".to_string(),
                group: GroupRepresentation {
                    name: Some("ChildGroup".to_string()),
                    ..Default::default()
                },
            };
            assert!(params.validate().is_ok());
        }

        #[test]
        fn test_validate_empty_realm() {
            let params = GroupChildCreateParams {
                realm: "".to_string(),
                group_id: "parent-123".to_string(),
                group: GroupRepresentation {
                    name: Some("ChildGroup".to_string()),
                    ..Default::default()
                },
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_group_id() {
            let params = GroupChildCreateParams {
                realm: "master".to_string(),
                group_id: "".to_string(),
                group: GroupRepresentation {
                    name: Some("ChildGroup".to_string()),
                    ..Default::default()
                },
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_missing_name() {
            let params = GroupChildCreateParams {
                realm: "master".to_string(),
                group_id: "parent-123".to_string(),
                group: GroupRepresentation::default(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_name() {
            let params = GroupChildCreateParams {
                realm: "master".to_string(),
                group_id: "parent-123".to_string(),
                group: GroupRepresentation {
                    name: Some("".to_string()),
                    ..Default::default()
                },
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }
    }

    mod group_set_parent_params_tests {
        use super::*;

        #[test]
        fn test_validate_success() {
            let params = GroupSetParentParams {
                realm: "master".to_string(),
                child_id: "child-123".to_string(),
                parent_id: "parent-123".to_string(),
            };
            assert!(params.validate().is_ok());
        }

        #[test]
        fn test_validate_empty_realm() {
            let params = GroupSetParentParams {
                realm: "".to_string(),
                child_id: "child-123".to_string(),
                parent_id: "parent-123".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_child_id() {
            let params = GroupSetParentParams {
                realm: "master".to_string(),
                child_id: "".to_string(),
                parent_id: "parent-123".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }

        #[test]
        fn test_validate_empty_parent_id() {
            let params = GroupSetParentParams {
                realm: "master".to_string(),
                child_id: "child-123".to_string(),
                parent_id: "".to_string(),
            };
            assert!(matches!(params.validate(), Err(ApiError::BadRequest(_))));
        }
    }

    mod group_members_list_tests {
        use super::*;
        use crate::api::users::types::UserRepresentation;

        fn sample_user() -> UserRepresentation {
            UserRepresentation {
                id: Some("user-123".to_string()),
                username: Some("testuser".to_string()),
                email: Some("test@example.com".to_string()),
                enabled: Some(true),
                ..Default::default()
            }
        }

        #[tokio::test]
        async fn test_group_members_list_success() {
            let mock_server = MockServer::start().await;

            let expected_members = vec![sample_user()];

            Mock::given(method("GET"))
                .and(path("/admin/realms/master/groups/group-123/members"))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .respond_with(ResponseTemplate::new(200).set_body_json(&expected_members))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupMembersListParams {
                realm: "master".to_string(),
                group_id: "group-123".to_string(),
                first: None,
                max: None,
                brief_representation: None,
            };

            let members = group_members_list(&client, TEST_TOKEN, &params)
                .await
                .expect("Failed to list members");
            assert_eq!(members.len(), 1);
            assert_eq!(members[0].username.as_deref(), Some("testuser"));
        }

        #[tokio::test]
        async fn test_group_members_list_empty() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path("/admin/realms/master/groups/group-123/members"))
                .respond_with(
                    ResponseTemplate::new(200).set_body_json::<Vec<UserRepresentation>>(vec![]),
                )
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupMembersListParams {
                realm: "master".to_string(),
                group_id: "group-123".to_string(),
                first: None,
                max: None,
                brief_representation: None,
            };

            let members = group_members_list(&client, TEST_TOKEN, &params)
                .await
                .expect("Failed to list members");
            assert!(members.is_empty());
        }

        #[tokio::test]
        async fn test_group_members_list_with_pagination() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path("/admin/realms/master/groups/group-123/members"))
                .and(query_param("first", "10"))
                .and(query_param("max", "20"))
                .respond_with(
                    ResponseTemplate::new(200).set_body_json::<Vec<UserRepresentation>>(vec![]),
                )
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupMembersListParams {
                realm: "master".to_string(),
                group_id: "group-123".to_string(),
                first: Some(10),
                max: Some(20),
                brief_representation: None,
            };

            let result = group_members_list(&client, TEST_TOKEN, &params).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_group_members_list_not_found() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path("/admin/realms/master/groups/nonexistent/members"))
                .respond_with(ResponseTemplate::new(404))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupMembersListParams {
                realm: "master".to_string(),
                group_id: "nonexistent".to_string(),
                first: None,
                max: None,
                brief_representation: None,
            };

            let result = group_members_list(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::NotFound)));
        }

        #[tokio::test]
        async fn test_group_members_list_unauthorized() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path("/admin/realms/master/groups/group-123/members"))
                .respond_with(ResponseTemplate::new(401))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupMembersListParams {
                realm: "master".to_string(),
                group_id: "group-123".to_string(),
                first: None,
                max: None,
                brief_representation: None,
            };

            let result = group_members_list(&client, "invalid-token", &params).await;
            assert!(matches!(result, Err(ApiError::Unauthorized)));
        }

        #[tokio::test]
        async fn test_group_members_list_with_special_chars() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path(
                    "/admin/realms/master/groups/group%2Fwith%2Bspecial/members",
                ))
                .respond_with(
                    ResponseTemplate::new(200).set_body_json::<Vec<UserRepresentation>>(vec![]),
                )
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupMembersListParams {
                realm: "master".to_string(),
                group_id: "group/with+special".to_string(),
                first: None,
                max: None,
                brief_representation: None,
            };

            let result = group_members_list(&client, TEST_TOKEN, &params).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_group_members_list_validation_fails() {
            let mock_server = MockServer::start().await;
            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupMembersListParams {
                realm: "master".to_string(),
                group_id: "".to_string(),
                first: None,
                max: None,
                brief_representation: None,
            };

            let result = group_members_list(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::BadRequest(_))));
        }
    }

    mod group_children_list_tests {
        use super::*;

        #[tokio::test]
        async fn test_group_children_list_success() {
            let mock_server = MockServer::start().await;

            let expected_children = vec![sample_group()];

            Mock::given(method("GET"))
                .and(path("/admin/realms/master/groups/group-123/children"))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .respond_with(ResponseTemplate::new(200).set_body_json(&expected_children))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupChildrenListParams {
                realm: "master".to_string(),
                group_id: "group-123".to_string(),
            };

            let children = group_children_list(&client, TEST_TOKEN, &params)
                .await
                .expect("Failed to list children");
            assert_eq!(children.len(), 1);
            assert_eq!(children[0].name.as_deref(), Some("Developers"));
        }

        #[tokio::test]
        async fn test_group_children_list_empty() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path("/admin/realms/master/groups/group-123/children"))
                .respond_with(
                    ResponseTemplate::new(200).set_body_json::<Vec<GroupRepresentation>>(vec![]),
                )
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupChildrenListParams {
                realm: "master".to_string(),
                group_id: "group-123".to_string(),
            };

            let children = group_children_list(&client, TEST_TOKEN, &params)
                .await
                .expect("Failed to list children");
            assert!(children.is_empty());
        }

        #[tokio::test]
        async fn test_group_children_list_not_found() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path("/admin/realms/master/groups/nonexistent/children"))
                .respond_with(ResponseTemplate::new(404))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupChildrenListParams {
                realm: "master".to_string(),
                group_id: "nonexistent".to_string(),
            };

            let result = group_children_list(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::NotFound)));
        }

        #[tokio::test]
        async fn test_group_children_list_unauthorized() {
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path("/admin/realms/master/groups/group-123/children"))
                .respond_with(ResponseTemplate::new(401))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupChildrenListParams {
                realm: "master".to_string(),
                group_id: "group-123".to_string(),
            };

            let result = group_children_list(&client, "invalid-token", &params).await;
            assert!(matches!(result, Err(ApiError::Unauthorized)));
        }

        #[tokio::test]
        async fn test_group_children_list_validation_fails() {
            let mock_server = MockServer::start().await;
            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupChildrenListParams {
                realm: "".to_string(),
                group_id: "group-123".to_string(),
            };

            let result = group_children_list(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::BadRequest(_))));
        }
    }

    mod group_child_create_tests {
        use super::*;

        #[tokio::test]
        async fn test_group_child_create_success() {
            let mock_server = MockServer::start().await;

            let new_group = GroupRepresentation {
                name: Some("ChildGroup".to_string()),
                ..Default::default()
            };

            Mock::given(method("POST"))
                .and(path("/admin/realms/master/groups/parent-123/children"))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .and(body_json(&new_group))
                .respond_with(ResponseTemplate::new(201))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupChildCreateParams {
                realm: "master".to_string(),
                group_id: "parent-123".to_string(),
                group: new_group,
            };

            let result = group_child_create(&client, TEST_TOKEN, &params).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_group_child_create_not_found() {
            let mock_server = MockServer::start().await;

            Mock::given(method("POST"))
                .and(path("/admin/realms/master/groups/nonexistent/children"))
                .respond_with(ResponseTemplate::new(404))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupChildCreateParams {
                realm: "master".to_string(),
                group_id: "nonexistent".to_string(),
                group: GroupRepresentation {
                    name: Some("ChildGroup".to_string()),
                    ..Default::default()
                },
            };

            let result = group_child_create(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::NotFound)));
        }

        #[tokio::test]
        async fn test_group_child_create_conflict() {
            let mock_server = MockServer::start().await;

            Mock::given(method("POST"))
                .and(path("/admin/realms/master/groups/parent-123/children"))
                .respond_with(ResponseTemplate::new(409).set_body_json(serde_json::json!({
                    "errorMessage": "Group exists with same name"
                })))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupChildCreateParams {
                realm: "master".to_string(),
                group_id: "parent-123".to_string(),
                group: GroupRepresentation {
                    name: Some("ExistingChild".to_string()),
                    ..Default::default()
                },
            };

            let result = group_child_create(&client, TEST_TOKEN, &params).await;
            match result {
                Err(ApiError::Conflict(msg)) => {
                    assert!(msg.contains("Group exists"));
                }
                _ => panic!("Expected Conflict error"),
            }
        }

        #[tokio::test]
        async fn test_group_child_create_validation_fails() {
            let mock_server = MockServer::start().await;
            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupChildCreateParams {
                realm: "master".to_string(),
                group_id: "parent-123".to_string(),
                group: GroupRepresentation::default(),
            };

            let result = group_child_create(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::BadRequest(_))));
        }
    }

    mod group_set_parent_tests {
        use super::*;

        #[tokio::test]
        async fn test_group_set_parent_success() {
            let mock_server = MockServer::start().await;

            Mock::given(method("POST"))
                .and(path("/admin/realms/master/groups/child-123/parent"))
                .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
                .respond_with(ResponseTemplate::new(204))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupSetParentParams {
                realm: "master".to_string(),
                child_id: "child-123".to_string(),
                parent_id: "parent-123".to_string(),
            };

            let result = group_set_parent(&client, TEST_TOKEN, &params).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_group_set_parent_not_found() {
            let mock_server = MockServer::start().await;

            Mock::given(method("POST"))
                .and(path("/admin/realms/master/groups/nonexistent/parent"))
                .respond_with(ResponseTemplate::new(404))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupSetParentParams {
                realm: "master".to_string(),
                child_id: "nonexistent".to_string(),
                parent_id: "parent-123".to_string(),
            };

            let result = group_set_parent(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::NotFound)));
        }

        #[tokio::test]
        async fn test_group_set_parent_unauthorized() {
            let mock_server = MockServer::start().await;

            Mock::given(method("POST"))
                .and(path("/admin/realms/master/groups/child-123/parent"))
                .respond_with(ResponseTemplate::new(401))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupSetParentParams {
                realm: "master".to_string(),
                child_id: "child-123".to_string(),
                parent_id: "parent-123".to_string(),
            };

            let result = group_set_parent(&client, "invalid-token", &params).await;
            assert!(matches!(result, Err(ApiError::Unauthorized)));
        }

        #[tokio::test]
        async fn test_group_set_parent_with_special_chars() {
            let mock_server = MockServer::start().await;

            Mock::given(method("POST"))
                .and(path(
                    "/admin/realms/master/groups/child%2Fwith%2Bspecial/parent",
                ))
                .respond_with(ResponseTemplate::new(204))
                .mount(&mock_server)
                .await;

            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupSetParentParams {
                realm: "master".to_string(),
                child_id: "child/with+special".to_string(),
                parent_id: "parent-123".to_string(),
            };

            let result = group_set_parent(&client, TEST_TOKEN, &params).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_group_set_parent_validation_fails() {
            let mock_server = MockServer::start().await;
            let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
            let params = GroupSetParentParams {
                realm: "".to_string(),
                child_id: "child-123".to_string(),
                parent_id: "parent-123".to_string(),
            };

            let result = group_set_parent(&client, TEST_TOKEN, &params).await;
            assert!(matches!(result, Err(ApiError::BadRequest(_))));
        }
    }
}
