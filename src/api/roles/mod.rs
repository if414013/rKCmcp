//! Realm roles API module for Keycloak Admin REST API.

mod composites;
mod types;

pub use composites::*;
pub use types::*;

use crate::api::{ApiError, KeycloakClient};

pub async fn realm_role_list(
    client: &KeycloakClient,
    token: &str,
    params: &RoleListParams,
) -> Result<Vec<RoleRepresentation>, ApiError> {
    let mut path = format!("/admin/realms/{}/roles", params.realm);
    let mut query_parts = Vec::new();

    if let Some(ref search) = params.search {
        query_parts.push(format!("search={}", urlencoding::encode(search)));
    }
    if let Some(first) = params.first {
        query_parts.push(format!("first={}", first));
    }
    if let Some(max) = params.max {
        query_parts.push(format!("max={}", max));
    }
    if let Some(brief) = params.brief_representation {
        query_parts.push(format!("briefRepresentation={}", brief));
    }

    if !query_parts.is_empty() {
        path = format!("{}?{}", path, query_parts.join("&"));
    }

    client.get(&path, token).await
}

pub async fn realm_role_get(
    client: &KeycloakClient,
    token: &str,
    params: &RoleGetParams,
) -> Result<RoleRepresentation, ApiError> {
    let path = format!(
        "/admin/realms/{}/roles/{}",
        params.realm,
        urlencoding::encode(&params.role_name)
    );
    client.get(&path, token).await
}

pub async fn realm_role_create(
    client: &KeycloakClient,
    token: &str,
    params: &RoleCreateParams,
) -> Result<(), ApiError> {
    let path = format!("/admin/realms/{}/roles", params.realm);

    let role = RoleRepresentation {
        name: Some(params.name.clone()),
        description: params.description.clone(),
        attributes: params.attributes.clone(),
        ..Default::default()
    };

    client.post_no_response(&path, token, &role).await
}

pub async fn realm_role_update(
    client: &KeycloakClient,
    token: &str,
    params: &RoleUpdateParams,
) -> Result<(), ApiError> {
    let path = format!(
        "/admin/realms/{}/roles/{}",
        params.realm,
        urlencoding::encode(&params.role_name)
    );

    let role = RoleRepresentation {
        name: params.new_name.clone().or_else(|| Some(params.role_name.clone())),
        description: params.description.clone(),
        attributes: params.attributes.clone(),
        ..Default::default()
    };

    client.put(&path, token, &role).await
}

pub async fn realm_role_delete(
    client: &KeycloakClient,
    token: &str,
    params: &RoleDeleteParams,
) -> Result<(), ApiError> {
    let path = format!(
        "/admin/realms/{}/roles/{}",
        params.realm,
        urlencoding::encode(&params.role_name)
    );
    client.delete(&path, token).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{header, method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    const TEST_TOKEN: &str = "test-bearer-token";

    #[tokio::test]
    async fn test_realm_role_list_no_params() {
        let mock_server = MockServer::start().await;

        let roles = vec![
            RoleRepresentation {
                id: Some("1".to_string()),
                name: Some("admin".to_string()),
                description: Some("Admin role".to_string()),
                ..Default::default()
            },
            RoleRepresentation {
                id: Some("2".to_string()),
                name: Some("user".to_string()),
                description: Some("User role".to_string()),
                ..Default::default()
            },
        ];

        Mock::given(method("GET"))
            .and(path("/admin/realms/master/roles"))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&roles))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("client creation should succeed");
        let params = RoleListParams {
            realm: "master".to_string(),
            search: None,
            first: None,
            max: None,
            brief_representation: None,
        };

        let result = realm_role_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
        let returned_roles = result.expect("should return roles");
        assert_eq!(returned_roles.len(), 2);
        assert_eq!(returned_roles[0].name, Some("admin".to_string()));
    }

    #[tokio::test]
    async fn test_realm_role_list_with_search() {
        let mock_server = MockServer::start().await;

        let roles = vec![RoleRepresentation {
            id: Some("1".to_string()),
            name: Some("admin".to_string()),
            ..Default::default()
        }];

        Mock::given(method("GET"))
            .and(path("/admin/realms/test-realm/roles"))
            .and(query_param("search", "admin"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&roles))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("client creation should succeed");
        let params = RoleListParams {
            realm: "test-realm".to_string(),
            search: Some("admin".to_string()),
            first: None,
            max: None,
            brief_representation: None,
        };

        let result = realm_role_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_realm_role_list_with_pagination() {
        let mock_server = MockServer::start().await;

        let roles: Vec<RoleRepresentation> = vec![];

        Mock::given(method("GET"))
            .and(path("/admin/realms/master/roles"))
            .and(query_param("first", "10"))
            .and(query_param("max", "20"))
            .and(query_param("briefRepresentation", "true"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&roles))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("client creation should succeed");
        let params = RoleListParams {
            realm: "master".to_string(),
            search: None,
            first: Some(10),
            max: Some(20),
            brief_representation: Some(true),
        };

        let result = realm_role_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_realm_role_get_success() {
        let mock_server = MockServer::start().await;

        let role = RoleRepresentation {
            id: Some("123".to_string()),
            name: Some("admin".to_string()),
            description: Some("Administrator role".to_string()),
            composite: Some(false),
            client_role: Some(false),
            container_id: Some("master".to_string()),
            attributes: None,
        };

        Mock::given(method("GET"))
            .and(path("/admin/realms/master/roles/admin"))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&role))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("client creation should succeed");
        let params = RoleGetParams {
            realm: "master".to_string(),
            role_name: "admin".to_string(),
        };

        let result = realm_role_get(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
        let returned_role = result.expect("should return role");
        assert_eq!(returned_role.name, Some("admin".to_string()));
        assert_eq!(returned_role.id, Some("123".to_string()));
    }

    #[tokio::test]
    async fn test_realm_role_get_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/admin/realms/master/roles/nonexistent"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("client creation should succeed");
        let params = RoleGetParams {
            realm: "master".to_string(),
            role_name: "nonexistent".to_string(),
        };

        let result = realm_role_get(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_realm_role_get_with_special_characters() {
        let mock_server = MockServer::start().await;

        let role = RoleRepresentation {
            id: Some("456".to_string()),
            name: Some("my role".to_string()),
            ..Default::default()
        };

        Mock::given(method("GET"))
            .and(path("/admin/realms/master/roles/my%20role"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&role))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("client creation should succeed");
        let params = RoleGetParams {
            realm: "master".to_string(),
            role_name: "my role".to_string(),
        };

        let result = realm_role_get(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_realm_role_create_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/admin/realms/master/roles"))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .and(header("content-type", "application/json"))
            .respond_with(ResponseTemplate::new(201))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("client creation should succeed");
        let params = RoleCreateParams {
            realm: "master".to_string(),
            name: "new-role".to_string(),
            description: Some("A new test role".to_string()),
            attributes: None,
        };

        let result = realm_role_create(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_realm_role_create_conflict() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/admin/realms/master/roles"))
            .respond_with(ResponseTemplate::new(409).set_body_json(serde_json::json!({
                "errorMessage": "Role with name admin already exists"
            })))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("client creation should succeed");
        let params = RoleCreateParams {
            realm: "master".to_string(),
            name: "admin".to_string(),
            description: None,
            attributes: None,
        };

        let result = realm_role_create(&client, TEST_TOKEN, &params).await;
        match result {
            Err(ApiError::Conflict(msg)) => {
                assert!(msg.contains("already exists"));
            }
            _ => panic!("Expected Conflict error"),
        }
    }

    #[tokio::test]
    async fn test_realm_role_update_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path("/admin/realms/master/roles/old-name"))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("client creation should succeed");
        let params = RoleUpdateParams {
            realm: "master".to_string(),
            role_name: "old-name".to_string(),
            new_name: Some("new-name".to_string()),
            description: Some("Updated description".to_string()),
            attributes: None,
        };

        let result = realm_role_update(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_realm_role_update_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path("/admin/realms/master/roles/nonexistent"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("client creation should succeed");
        let params = RoleUpdateParams {
            realm: "master".to_string(),
            role_name: "nonexistent".to_string(),
            new_name: None,
            description: Some("New description".to_string()),
            attributes: None,
        };

        let result = realm_role_update(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_realm_role_delete_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/admin/realms/master/roles/test-role"))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("client creation should succeed");
        let params = RoleDeleteParams {
            realm: "master".to_string(),
            role_name: "test-role".to_string(),
        };

        let result = realm_role_delete(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_realm_role_delete_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/admin/realms/master/roles/nonexistent"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("client creation should succeed");
        let params = RoleDeleteParams {
            realm: "master".to_string(),
            role_name: "nonexistent".to_string(),
        };

        let result = realm_role_delete(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_realm_role_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/admin/realms/master/roles"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("client creation should succeed");
        let params = RoleListParams {
            realm: "master".to_string(),
            search: None,
            first: None,
            max: None,
            brief_representation: None,
        };

        let result = realm_role_list(&client, "invalid-token", &params).await;
        assert!(matches!(result, Err(ApiError::Unauthorized)));
    }

    #[tokio::test]
    async fn test_realm_role_forbidden() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/admin/realms/master/roles"))
            .respond_with(ResponseTemplate::new(403))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("client creation should succeed");
        let params = RoleListParams {
            realm: "master".to_string(),
            search: None,
            first: None,
            max: None,
            brief_representation: None,
        };

        let result = realm_role_list(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::Forbidden)));
    }
}
