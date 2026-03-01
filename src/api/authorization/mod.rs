//! Keycloak Authorization Services API module.
//!
//! Provides operations for managing authorization resources, scopes, policies, and permissions.

pub mod permissions;
pub mod policies;
pub mod types;

pub use permissions::*;
pub use policies::*;
pub use types::*;

use crate::api::{ApiError, KeycloakClient};

/// Get the resource server settings for a client.
///
/// GET /admin/realms/{realm}/clients/{id}/authz/resource-server
pub async fn authz_resource_server_get(
    client: &KeycloakClient,
    token: &str,
    params: &ResourceServerGetParams,
) -> Result<ResourceServerRepresentation, ApiError> {
    let path = format!(
        "/admin/realms/{}/clients/{}/authz/resource-server",
        urlencoding::encode(&params.realm),
        urlencoding::encode(&params.id)
    );

    client.get(&path, token).await
}

/// Update the resource server settings for a client.
///
/// PUT /admin/realms/{realm}/clients/{id}/authz/resource-server
pub async fn authz_resource_server_update(
    client: &KeycloakClient,
    token: &str,
    params: &ResourceServerUpdateParams,
) -> Result<(), ApiError> {
    let path = format!(
        "/admin/realms/{}/clients/{}/authz/resource-server",
        urlencoding::encode(&params.realm),
        urlencoding::encode(&params.id)
    );

    client.put(&path, token, &params.resource_server).await
}

/// List resources for a client's resource server.
///
/// GET /admin/realms/{realm}/clients/{id}/authz/resource-server/resource
pub async fn authz_resources_list(
    client: &KeycloakClient,
    token: &str,
    params: &ResourceListParams,
) -> Result<Vec<ResourceRepresentation>, ApiError> {
    let mut path = format!(
        "/admin/realms/{}/clients/{}/authz/resource-server/resource",
        urlencoding::encode(&params.realm),
        urlencoding::encode(&params.id)
    );

    let mut query_parts = Vec::new();

    if let Some(ref name) = params.name {
        query_parts.push(format!("name={}", urlencoding::encode(name)));
    }

    if let Some(ref uri) = params.uri {
        query_parts.push(format!("uri={}", urlencoding::encode(uri)));
    }

    if let Some(ref owner) = params.owner {
        query_parts.push(format!("owner={}", urlencoding::encode(owner)));
    }

    if let Some(ref resource_type) = params.resource_type {
        query_parts.push(format!("type={}", urlencoding::encode(resource_type)));
    }

    if let Some(ref scope) = params.scope {
        query_parts.push(format!("scope={}", urlencoding::encode(scope)));
    }

    if let Some(first) = params.first {
        query_parts.push(format!("first={}", first));
    }

    if let Some(max) = params.max {
        query_parts.push(format!("max={}", max));
    }

    if let Some(deep) = params.deep {
        query_parts.push(format!("deep={}", deep));
    }

    if let Some(exact_name) = params.exact_name {
        query_parts.push(format!("exactName={}", exact_name));
    }

    if !query_parts.is_empty() {
        path = format!("{}?{}", path, query_parts.join("&"));
    }

    client.get(&path, token).await
}

/// Get a specific resource by ID.
///
/// GET /admin/realms/{realm}/clients/{id}/authz/resource-server/resource/{resource-id}
pub async fn authz_resource_get(
    client: &KeycloakClient,
    token: &str,
    params: &ResourceGetParams,
) -> Result<ResourceRepresentation, ApiError> {
    let path = format!(
        "/admin/realms/{}/clients/{}/authz/resource-server/resource/{}",
        urlencoding::encode(&params.realm),
        urlencoding::encode(&params.id),
        urlencoding::encode(&params.resource_id)
    );

    client.get(&path, token).await
}

/// Create a new resource.
///
/// POST /admin/realms/{realm}/clients/{id}/authz/resource-server/resource
pub async fn authz_resource_create(
    client: &KeycloakClient,
    token: &str,
    params: &ResourceCreateParams,
) -> Result<ResourceRepresentation, ApiError> {
    let path = format!(
        "/admin/realms/{}/clients/{}/authz/resource-server/resource",
        urlencoding::encode(&params.realm),
        urlencoding::encode(&params.id)
    );

    client.post(&path, token, &params.resource).await
}

/// Update an existing resource.
///
/// PUT /admin/realms/{realm}/clients/{id}/authz/resource-server/resource/{resource-id}
pub async fn authz_resource_update(
    client: &KeycloakClient,
    token: &str,
    params: &ResourceUpdateParams,
) -> Result<(), ApiError> {
    let path = format!(
        "/admin/realms/{}/clients/{}/authz/resource-server/resource/{}",
        urlencoding::encode(&params.realm),
        urlencoding::encode(&params.id),
        urlencoding::encode(&params.resource_id)
    );

    client.put(&path, token, &params.resource).await
}

/// Delete a resource.
///
/// DELETE /admin/realms/{realm}/clients/{id}/authz/resource-server/resource/{resource-id}
pub async fn authz_resource_delete(
    client: &KeycloakClient,
    token: &str,
    params: &ResourceDeleteParams,
) -> Result<(), ApiError> {
    let path = format!(
        "/admin/realms/{}/clients/{}/authz/resource-server/resource/{}",
        urlencoding::encode(&params.realm),
        urlencoding::encode(&params.id),
        urlencoding::encode(&params.resource_id)
    );

    client.delete(&path, token).await
}

/// List scopes for a client's resource server.
///
/// GET /admin/realms/{realm}/clients/{id}/authz/resource-server/scope
pub async fn authz_scopes_list(
    client: &KeycloakClient,
    token: &str,
    params: &ScopeListParams,
) -> Result<Vec<ScopeRepresentation>, ApiError> {
    let mut path = format!(
        "/admin/realms/{}/clients/{}/authz/resource-server/scope",
        urlencoding::encode(&params.realm),
        urlencoding::encode(&params.id)
    );

    let mut query_parts = Vec::new();

    if let Some(ref name) = params.name {
        query_parts.push(format!("name={}", urlencoding::encode(name)));
    }

    if let Some(first) = params.first {
        query_parts.push(format!("first={}", first));
    }

    if let Some(max) = params.max {
        query_parts.push(format!("max={}", max));
    }

    if !query_parts.is_empty() {
        path = format!("{}?{}", path, query_parts.join("&"));
    }

    client.get(&path, token).await
}

/// Create a new scope.
///
/// POST /admin/realms/{realm}/clients/{id}/authz/resource-server/scope
pub async fn authz_scope_create(
    client: &KeycloakClient,
    token: &str,
    params: &ScopeCreateParams,
) -> Result<ScopeRepresentation, ApiError> {
    let path = format!(
        "/admin/realms/{}/clients/{}/authz/resource-server/scope",
        urlencoding::encode(&params.realm),
        urlencoding::encode(&params.id)
    );

    client.post(&path, token, &params.scope).await
}

/// Delete a scope.
///
/// DELETE /admin/realms/{realm}/clients/{id}/authz/resource-server/scope/{scope-id}
pub async fn authz_scope_delete(
    client: &KeycloakClient,
    token: &str,
    params: &ScopeDeleteParams,
) -> Result<(), ApiError> {
    let path = format!(
        "/admin/realms/{}/clients/{}/authz/resource-server/scope/{}",
        urlencoding::encode(&params.realm),
        urlencoding::encode(&params.id),
        urlencoding::encode(&params.scope_id)
    );

    client.delete(&path, token).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{body_json, header, method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    const TEST_TOKEN: &str = "test-bearer-token";

    fn sample_resource_server() -> ResourceServerRepresentation {
        ResourceServerRepresentation {
            id: Some("client-123".to_string()),
            client_id: Some("my-authz-client".to_string()),
            name: Some("My Authz Client".to_string()),
            allow_remote_resource_management: Some(true),
            policy_enforcement_mode: Some("ENFORCING".to_string()),
            decision_strategy: Some("UNANIMOUS".to_string()),
        }
    }

    fn sample_resource() -> ResourceRepresentation {
        ResourceRepresentation {
            id: Some("resource-123".to_string()),
            name: Some("My Resource".to_string()),
            display_name: Some("My Display Name".to_string()),
            uris: Some(vec!["/api/resources/*".to_string()]),
            resource_type: Some("urn:my-app:resources:default".to_string()),
            icon_uri: None,
            owner: None,
            owner_managed_access: Some(false),
            scopes: None,
            attributes: None,
        }
    }

    fn sample_scope() -> ScopeRepresentation {
        ScopeRepresentation {
            id: Some("scope-123".to_string()),
            name: Some("read".to_string()),
            display_name: Some("Read Access".to_string()),
            icon_uri: None,
        }
    }

    #[tokio::test]
    async fn test_authz_resource_server_get_success() {
        let mock_server = MockServer::start().await;
        let expected_rs = sample_resource_server();

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/master/clients/client-123/authz/resource-server",
            ))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_rs))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ResourceServerGetParams {
            realm: "master".to_string(),
            id: "client-123".to_string(),
        };

        let result = authz_resource_server_get(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());

        let rs = result.expect("Failed to get resource server");
        assert_eq!(rs.id, Some("client-123".to_string()));
        assert_eq!(rs.client_id, Some("my-authz-client".to_string()));
    }

    #[tokio::test]
    async fn test_authz_resource_server_get_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/master/clients/nonexistent/authz/resource-server",
            ))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ResourceServerGetParams {
            realm: "master".to_string(),
            id: "nonexistent".to_string(),
        };

        let result = authz_resource_server_get(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_authz_resource_server_get_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/master/clients/client-123/authz/resource-server",
            ))
            .respond_with(ResponseTemplate::new(401))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ResourceServerGetParams {
            realm: "master".to_string(),
            id: "client-123".to_string(),
        };

        let result = authz_resource_server_get(&client, "invalid-token", &params).await;
        assert!(matches!(result, Err(ApiError::Unauthorized)));
    }

    #[tokio::test]
    async fn test_authz_resource_server_get_forbidden() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/master/clients/client-123/authz/resource-server",
            ))
            .respond_with(ResponseTemplate::new(403))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ResourceServerGetParams {
            realm: "master".to_string(),
            id: "client-123".to_string(),
        };

        let result = authz_resource_server_get(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::Forbidden)));
    }

    #[tokio::test]
    async fn test_authz_resource_server_update_success() {
        let mock_server = MockServer::start().await;
        let rs = sample_resource_server();

        Mock::given(method("PUT"))
            .and(path(
                "/admin/realms/master/clients/client-123/authz/resource-server",
            ))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .and(body_json(&rs))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ResourceServerUpdateParams {
            realm: "master".to_string(),
            id: "client-123".to_string(),
            resource_server: rs,
        };

        let result = authz_resource_server_update(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_authz_resource_server_update_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path(
                "/admin/realms/master/clients/nonexistent/authz/resource-server",
            ))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ResourceServerUpdateParams {
            realm: "master".to_string(),
            id: "nonexistent".to_string(),
            resource_server: ResourceServerRepresentation::default(),
        };

        let result = authz_resource_server_update(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_authz_resources_list_success() {
        let mock_server = MockServer::start().await;
        let expected_resources = vec![sample_resource()];

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/master/clients/client-123/authz/resource-server/resource",
            ))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_resources))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ResourceListParams {
            realm: "master".to_string(),
            id: "client-123".to_string(),
            name: None,
            uri: None,
            owner: None,
            resource_type: None,
            scope: None,
            first: None,
            max: None,
            deep: None,
            exact_name: None,
        };

        let result = authz_resources_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());

        let resources = result.expect("Failed to get resources");
        assert_eq!(resources.len(), 1);
        assert_eq!(resources[0].name, Some("My Resource".to_string()));
    }

    #[tokio::test]
    async fn test_authz_resources_list_with_filters() {
        let mock_server = MockServer::start().await;
        let expected_resources = vec![sample_resource()];

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/test-realm/clients/client-123/authz/resource-server/resource",
            ))
            .and(query_param("name", "my-resource"))
            .and(query_param("first", "0"))
            .and(query_param("max", "10"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_resources))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ResourceListParams {
            realm: "test-realm".to_string(),
            id: "client-123".to_string(),
            name: Some("my-resource".to_string()),
            uri: None,
            owner: None,
            resource_type: None,
            scope: None,
            first: Some(0),
            max: Some(10),
            deep: None,
            exact_name: None,
        };

        let result = authz_resources_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_authz_resources_list_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/master/clients/client-123/authz/resource-server/resource",
            ))
            .respond_with(ResponseTemplate::new(401))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ResourceListParams {
            realm: "master".to_string(),
            id: "client-123".to_string(),
            name: None,
            uri: None,
            owner: None,
            resource_type: None,
            scope: None,
            first: None,
            max: None,
            deep: None,
            exact_name: None,
        };

        let result = authz_resources_list(&client, "invalid-token", &params).await;
        assert!(matches!(result, Err(ApiError::Unauthorized)));
    }

    #[tokio::test]
    async fn test_authz_resource_get_success() {
        let mock_server = MockServer::start().await;
        let expected_resource = sample_resource();

        Mock::given(method("GET"))
            .and(path("/admin/realms/master/clients/client-123/authz/resource-server/resource/resource-123"))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_resource))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ResourceGetParams {
            realm: "master".to_string(),
            id: "client-123".to_string(),
            resource_id: "resource-123".to_string(),
        };

        let result = authz_resource_get(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());

        let resource = result.expect("Failed to get resource");
        assert_eq!(resource.id, Some("resource-123".to_string()));
    }

    #[tokio::test]
    async fn test_authz_resource_get_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/admin/realms/master/clients/client-123/authz/resource-server/resource/nonexistent"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ResourceGetParams {
            realm: "master".to_string(),
            id: "client-123".to_string(),
            resource_id: "nonexistent".to_string(),
        };

        let result = authz_resource_get(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_authz_resource_create_success() {
        let mock_server = MockServer::start().await;
        let new_resource = ResourceRepresentation {
            name: Some("New Resource".to_string()),
            uris: Some(vec!["/api/new/*".to_string()]),
            ..Default::default()
        };

        let created_resource = ResourceRepresentation {
            id: Some("new-resource-id".to_string()),
            name: Some("New Resource".to_string()),
            uris: Some(vec!["/api/new/*".to_string()]),
            ..Default::default()
        };

        Mock::given(method("POST"))
            .and(path(
                "/admin/realms/master/clients/client-123/authz/resource-server/resource",
            ))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .and(body_json(&new_resource))
            .respond_with(ResponseTemplate::new(201).set_body_json(&created_resource))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ResourceCreateParams {
            realm: "master".to_string(),
            id: "client-123".to_string(),
            resource: new_resource,
        };

        let result = authz_resource_create(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());

        let resource = result.expect("Failed to create resource");
        assert_eq!(resource.id, Some("new-resource-id".to_string()));
    }

    #[tokio::test]
    async fn test_authz_resource_create_forbidden() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path(
                "/admin/realms/master/clients/client-123/authz/resource-server/resource",
            ))
            .respond_with(ResponseTemplate::new(403))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ResourceCreateParams {
            realm: "master".to_string(),
            id: "client-123".to_string(),
            resource: ResourceRepresentation::default(),
        };

        let result = authz_resource_create(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::Forbidden)));
    }

    #[tokio::test]
    async fn test_authz_resource_update_success() {
        let mock_server = MockServer::start().await;
        let updated_resource = sample_resource();

        Mock::given(method("PUT"))
            .and(path("/admin/realms/master/clients/client-123/authz/resource-server/resource/resource-123"))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .and(body_json(&updated_resource))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ResourceUpdateParams {
            realm: "master".to_string(),
            id: "client-123".to_string(),
            resource_id: "resource-123".to_string(),
            resource: updated_resource,
        };

        let result = authz_resource_update(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_authz_resource_update_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path("/admin/realms/master/clients/client-123/authz/resource-server/resource/nonexistent"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ResourceUpdateParams {
            realm: "master".to_string(),
            id: "client-123".to_string(),
            resource_id: "nonexistent".to_string(),
            resource: ResourceRepresentation::default(),
        };

        let result = authz_resource_update(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_authz_resource_delete_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/admin/realms/master/clients/client-123/authz/resource-server/resource/resource-123"))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ResourceDeleteParams {
            realm: "master".to_string(),
            id: "client-123".to_string(),
            resource_id: "resource-123".to_string(),
        };

        let result = authz_resource_delete(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_authz_resource_delete_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/admin/realms/master/clients/client-123/authz/resource-server/resource/nonexistent"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ResourceDeleteParams {
            realm: "master".to_string(),
            id: "client-123".to_string(),
            resource_id: "nonexistent".to_string(),
        };

        let result = authz_resource_delete(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_authz_scopes_list_success() {
        let mock_server = MockServer::start().await;
        let expected_scopes = vec![sample_scope()];

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/master/clients/client-123/authz/resource-server/scope",
            ))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_scopes))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ScopeListParams {
            realm: "master".to_string(),
            id: "client-123".to_string(),
            name: None,
            first: None,
            max: None,
        };

        let result = authz_scopes_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());

        let scopes = result.expect("Failed to get scopes");
        assert_eq!(scopes.len(), 1);
        assert_eq!(scopes[0].name, Some("read".to_string()));
    }

    #[tokio::test]
    async fn test_authz_scopes_list_with_filters() {
        let mock_server = MockServer::start().await;
        let expected_scopes = vec![sample_scope()];

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/test-realm/clients/client-123/authz/resource-server/scope",
            ))
            .and(query_param("name", "read"))
            .and(query_param("first", "0"))
            .and(query_param("max", "50"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_scopes))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ScopeListParams {
            realm: "test-realm".to_string(),
            id: "client-123".to_string(),
            name: Some("read".to_string()),
            first: Some(0),
            max: Some(50),
        };

        let result = authz_scopes_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_authz_scopes_list_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/master/clients/client-123/authz/resource-server/scope",
            ))
            .respond_with(ResponseTemplate::new(401))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ScopeListParams {
            realm: "master".to_string(),
            id: "client-123".to_string(),
            name: None,
            first: None,
            max: None,
        };

        let result = authz_scopes_list(&client, "invalid-token", &params).await;
        assert!(matches!(result, Err(ApiError::Unauthorized)));
    }

    #[tokio::test]
    async fn test_authz_scope_create_success() {
        let mock_server = MockServer::start().await;
        let new_scope = ScopeRepresentation {
            name: Some("write".to_string()),
            display_name: Some("Write Access".to_string()),
            ..Default::default()
        };

        let created_scope = ScopeRepresentation {
            id: Some("new-scope-id".to_string()),
            name: Some("write".to_string()),
            display_name: Some("Write Access".to_string()),
            icon_uri: None,
        };

        Mock::given(method("POST"))
            .and(path(
                "/admin/realms/master/clients/client-123/authz/resource-server/scope",
            ))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .and(body_json(&new_scope))
            .respond_with(ResponseTemplate::new(201).set_body_json(&created_scope))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ScopeCreateParams {
            realm: "master".to_string(),
            id: "client-123".to_string(),
            scope: new_scope,
        };

        let result = authz_scope_create(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());

        let scope = result.expect("Failed to create scope");
        assert_eq!(scope.id, Some("new-scope-id".to_string()));
    }

    #[tokio::test]
    async fn test_authz_scope_create_forbidden() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path(
                "/admin/realms/master/clients/client-123/authz/resource-server/scope",
            ))
            .respond_with(ResponseTemplate::new(403))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ScopeCreateParams {
            realm: "master".to_string(),
            id: "client-123".to_string(),
            scope: ScopeRepresentation::default(),
        };

        let result = authz_scope_create(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::Forbidden)));
    }

    #[tokio::test]
    async fn test_authz_scope_delete_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path(
                "/admin/realms/master/clients/client-123/authz/resource-server/scope/scope-123",
            ))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ScopeDeleteParams {
            realm: "master".to_string(),
            id: "client-123".to_string(),
            scope_id: "scope-123".to_string(),
        };

        let result = authz_scope_delete(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_authz_scope_delete_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path(
                "/admin/realms/master/clients/client-123/authz/resource-server/scope/nonexistent",
            ))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ScopeDeleteParams {
            realm: "master".to_string(),
            id: "client-123".to_string(),
            scope_id: "nonexistent".to_string(),
        };

        let result = authz_scope_delete(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_authz_resource_with_special_characters() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/admin/realms/my%20realm/clients/client%2Fspecial/authz/resource-server/resource/resource%2B123"))
            .respond_with(ResponseTemplate::new(200).set_body_json(sample_resource()))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ResourceGetParams {
            realm: "my realm".to_string(),
            id: "client/special".to_string(),
            resource_id: "resource+123".to_string(),
        };

        let result = authz_resource_get(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }
}
