//! Keycloak Authorization Permissions API.
//!
//! Provides CRUD operations for authorization permissions and policy evaluation.

use crate::api::{ApiError, KeycloakClient};

pub use super::types::{
    DecisionStrategy, EvaluationResult, Logic, PermissionCreateParams, PermissionDeleteParams,
    PermissionGetParams, PermissionListParams, PermissionRepresentation, PermissionUpdateParams,
    PolicyEvaluateParams, PolicyEvaluationRequest, PolicyEvaluationResponse,
};

/// List permissions for a client's authorization resource server.
///
/// GET /admin/realms/{realm}/clients/{id}/authz/resource-server/permission
pub async fn authz_permissions_list(
    client: &KeycloakClient,
    token: &str,
    params: &PermissionListParams,
) -> Result<Vec<PermissionRepresentation>, ApiError> {
    let mut path = format!(
        "/admin/realms/{}/clients/{}/authz/resource-server/permission",
        urlencoding::encode(&params.realm),
        urlencoding::encode(&params.client_id)
    );

    let mut query_parts = Vec::new();

    if let Some(ref name) = params.name {
        query_parts.push(format!("name={}", urlencoding::encode(name)));
    }

    if let Some(ref permission_type) = params.permission_type {
        query_parts.push(format!("type={}", urlencoding::encode(permission_type)));
    }

    if let Some(ref resource) = params.resource {
        query_parts.push(format!("resource={}", urlencoding::encode(resource)));
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

    if !query_parts.is_empty() {
        path = format!("{}?{}", path, query_parts.join("&"));
    }

    client.get(&path, token).await
}

/// Get a single permission by ID.
///
/// GET /admin/realms/{realm}/clients/{id}/authz/resource-server/permission/{permission-id}
pub async fn authz_permission_get(
    client: &KeycloakClient,
    token: &str,
    params: &PermissionGetParams,
) -> Result<PermissionRepresentation, ApiError> {
    let path = format!(
        "/admin/realms/{}/clients/{}/authz/resource-server/permission/{}",
        urlencoding::encode(&params.realm),
        urlencoding::encode(&params.client_id),
        urlencoding::encode(&params.permission_id)
    );

    client.get(&path, token).await
}

/// Create a new permission.
///
/// POST /admin/realms/{realm}/clients/{id}/authz/resource-server/permission/{type}
pub async fn authz_permission_create(
    client: &KeycloakClient,
    token: &str,
    params: &PermissionCreateParams,
) -> Result<(), ApiError> {
    let path = format!(
        "/admin/realms/{}/clients/{}/authz/resource-server/permission/{}",
        urlencoding::encode(&params.realm),
        urlencoding::encode(&params.client_id),
        urlencoding::encode(&params.permission_type)
    );

    client
        .post_no_response(&path, token, &params.permission)
        .await
}

/// Update an existing permission.
///
/// PUT /admin/realms/{realm}/clients/{id}/authz/resource-server/permission/{permission-id}
pub async fn authz_permission_update(
    client: &KeycloakClient,
    token: &str,
    params: &PermissionUpdateParams,
) -> Result<(), ApiError> {
    let path = format!(
        "/admin/realms/{}/clients/{}/authz/resource-server/permission/{}",
        urlencoding::encode(&params.realm),
        urlencoding::encode(&params.client_id),
        urlencoding::encode(&params.permission_id)
    );

    client.put(&path, token, &params.permission).await
}

/// Delete a permission.
///
/// DELETE /admin/realms/{realm}/clients/{id}/authz/resource-server/permission/{permission-id}
pub async fn authz_permission_delete(
    client: &KeycloakClient,
    token: &str,
    params: &PermissionDeleteParams,
) -> Result<(), ApiError> {
    let path = format!(
        "/admin/realms/{}/clients/{}/authz/resource-server/permission/{}",
        urlencoding::encode(&params.realm),
        urlencoding::encode(&params.client_id),
        urlencoding::encode(&params.permission_id)
    );

    client.delete(&path, token).await
}

/// Evaluate policies for a given context.
///
/// POST /admin/realms/{realm}/clients/{id}/authz/resource-server/policy/evaluate
pub async fn authz_evaluate(
    client: &KeycloakClient,
    token: &str,
    params: &PolicyEvaluateParams,
) -> Result<PolicyEvaluationResponse, ApiError> {
    let path = format!(
        "/admin/realms/{}/clients/{}/authz/resource-server/policy/evaluate",
        urlencoding::encode(&params.realm),
        urlencoding::encode(&params.client_id)
    );

    client.post(&path, token, &params.request).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::authorization::types::ResourceEvaluationRequest;
    use wiremock::matchers::{body_json, header, method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    const TEST_TOKEN: &str = "test-bearer-token";

    fn sample_resource_permission() -> PermissionRepresentation {
        PermissionRepresentation {
            id: Some("perm-123".to_string()),
            name: Some("View Resource Permission".to_string()),
            description: Some("Permission for viewing resources".to_string()),
            permission_type: Some("resource".to_string()),
            logic: Some(Logic::Positive),
            decision_strategy: Some(DecisionStrategy::Affirmative),
            resources: Some(vec!["resource-1".to_string()]),
            policies: Some(vec!["policy-1".to_string()]),
            ..Default::default()
        }
    }

    #[tokio::test]
    async fn test_authz_permissions_list_success() {
        let mock_server = MockServer::start().await;

        let expected_permissions = vec![sample_resource_permission()];

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/master/clients/client-123/authz/resource-server/permission",
            ))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_permissions))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = PermissionListParams {
            realm: "master".to_string(),
            client_id: "client-123".to_string(),
            name: None,
            permission_type: None,
            resource: None,
            scope: None,
            first: None,
            max: None,
        };

        let result = authz_permissions_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());

        let permissions = result.expect("Failed to get permissions");
        assert_eq!(permissions.len(), 1);
        assert_eq!(
            permissions[0].name,
            Some("View Resource Permission".to_string())
        );
    }

    #[tokio::test]
    async fn test_authz_permissions_list_with_filters() {
        let mock_server = MockServer::start().await;

        let expected_permissions = vec![sample_resource_permission()];

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/test-realm/clients/client-456/authz/resource-server/permission",
            ))
            .and(query_param("name", "view"))
            .and(query_param("type", "resource"))
            .and(query_param("first", "0"))
            .and(query_param("max", "10"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_permissions))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = PermissionListParams {
            realm: "test-realm".to_string(),
            client_id: "client-456".to_string(),
            name: Some("view".to_string()),
            permission_type: Some("resource".to_string()),
            resource: None,
            scope: None,
            first: Some(0),
            max: Some(10),
        };

        let result = authz_permissions_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_authz_permissions_list_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/master/clients/client-123/authz/resource-server/permission",
            ))
            .respond_with(ResponseTemplate::new(401))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = PermissionListParams {
            realm: "master".to_string(),
            client_id: "client-123".to_string(),
            name: None,
            permission_type: None,
            resource: None,
            scope: None,
            first: None,
            max: None,
        };

        let result = authz_permissions_list(&client, "invalid-token", &params).await;
        assert!(matches!(result, Err(ApiError::Unauthorized)));
    }

    #[tokio::test]
    async fn test_authz_permissions_list_forbidden() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/master/clients/client-123/authz/resource-server/permission",
            ))
            .respond_with(ResponseTemplate::new(403))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = PermissionListParams {
            realm: "master".to_string(),
            client_id: "client-123".to_string(),
            name: None,
            permission_type: None,
            resource: None,
            scope: None,
            first: None,
            max: None,
        };

        let result = authz_permissions_list(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::Forbidden)));
    }

    #[tokio::test]
    async fn test_authz_permission_get_success() {
        let mock_server = MockServer::start().await;

        let expected_permission = sample_resource_permission();

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/master/clients/client-123/authz/resource-server/permission/perm-123",
            ))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_permission))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = PermissionGetParams {
            realm: "master".to_string(),
            client_id: "client-123".to_string(),
            permission_id: "perm-123".to_string(),
        };

        let result = authz_permission_get(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());

        let permission = result.expect("Failed to get permission");
        assert_eq!(permission.id, Some("perm-123".to_string()));
        assert_eq!(
            permission.name,
            Some("View Resource Permission".to_string())
        );
    }

    #[tokio::test]
    async fn test_authz_permission_get_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/master/clients/client-123/authz/resource-server/permission/nonexistent",
            ))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = PermissionGetParams {
            realm: "master".to_string(),
            client_id: "client-123".to_string(),
            permission_id: "nonexistent".to_string(),
        };

        let result = authz_permission_get(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_authz_permission_create_success() {
        let mock_server = MockServer::start().await;

        let new_permission = PermissionRepresentation {
            name: Some("New Resource Permission".to_string()),
            permission_type: Some("resource".to_string()),
            logic: Some(Logic::Positive),
            resources: Some(vec!["resource-1".to_string()]),
            ..Default::default()
        };

        Mock::given(method("POST"))
            .and(path(
                "/admin/realms/master/clients/client-123/authz/resource-server/permission/resource",
            ))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .and(body_json(&new_permission))
            .respond_with(ResponseTemplate::new(201))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = PermissionCreateParams {
            realm: "master".to_string(),
            client_id: "client-123".to_string(),
            permission_type: "resource".to_string(),
            permission: new_permission,
        };

        let result = authz_permission_create(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_authz_permission_create_scope_type() {
        let mock_server = MockServer::start().await;

        let new_permission = PermissionRepresentation {
            name: Some("New Scope Permission".to_string()),
            permission_type: Some("scope".to_string()),
            scopes: Some(vec!["view".to_string(), "edit".to_string()]),
            ..Default::default()
        };

        Mock::given(method("POST"))
            .and(path(
                "/admin/realms/master/clients/client-123/authz/resource-server/permission/scope",
            ))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .and(body_json(&new_permission))
            .respond_with(ResponseTemplate::new(201))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = PermissionCreateParams {
            realm: "master".to_string(),
            client_id: "client-123".to_string(),
            permission_type: "scope".to_string(),
            permission: new_permission,
        };

        let result = authz_permission_create(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_authz_permission_create_conflict() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path(
                "/admin/realms/master/clients/client-123/authz/resource-server/permission/resource",
            ))
            .respond_with(ResponseTemplate::new(409).set_body_json(serde_json::json!({
                "errorMessage": "Permission already exists"
            })))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = PermissionCreateParams {
            realm: "master".to_string(),
            client_id: "client-123".to_string(),
            permission_type: "resource".to_string(),
            permission: PermissionRepresentation {
                name: Some("Existing Permission".to_string()),
                ..Default::default()
            },
        };

        let result = authz_permission_create(&client, TEST_TOKEN, &params).await;
        match result {
            Err(ApiError::Conflict(msg)) => {
                assert!(msg.contains("Permission already exists"));
            }
            _ => panic!("Expected Conflict error"),
        }
    }

    #[tokio::test]
    async fn test_authz_permission_update_success() {
        let mock_server = MockServer::start().await;

        let updated_permission = PermissionRepresentation {
            id: Some("perm-123".to_string()),
            name: Some("Updated Permission".to_string()),
            permission_type: Some("resource".to_string()),
            logic: Some(Logic::Negative),
            ..Default::default()
        };

        Mock::given(method("PUT"))
            .and(path(
                "/admin/realms/master/clients/client-123/authz/resource-server/permission/perm-123",
            ))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .and(body_json(&updated_permission))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = PermissionUpdateParams {
            realm: "master".to_string(),
            client_id: "client-123".to_string(),
            permission_id: "perm-123".to_string(),
            permission: updated_permission,
        };

        let result = authz_permission_update(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_authz_permission_update_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path(
                "/admin/realms/master/clients/client-123/authz/resource-server/permission/nonexistent",
            ))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = PermissionUpdateParams {
            realm: "master".to_string(),
            client_id: "client-123".to_string(),
            permission_id: "nonexistent".to_string(),
            permission: PermissionRepresentation::default(),
        };

        let result = authz_permission_update(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_authz_permission_delete_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path(
                "/admin/realms/master/clients/client-123/authz/resource-server/permission/perm-123",
            ))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = PermissionDeleteParams {
            realm: "master".to_string(),
            client_id: "client-123".to_string(),
            permission_id: "perm-123".to_string(),
        };

        let result = authz_permission_delete(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_authz_permission_delete_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path(
                "/admin/realms/master/clients/client-123/authz/resource-server/permission/nonexistent",
            ))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = PermissionDeleteParams {
            realm: "master".to_string(),
            client_id: "client-123".to_string(),
            permission_id: "nonexistent".to_string(),
        };

        let result = authz_permission_delete(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_authz_evaluate_success() {
        let mock_server = MockServer::start().await;

        let eval_request = PolicyEvaluationRequest {
            user_id: Some("user-123".to_string()),
            resources: Some(vec![ResourceEvaluationRequest {
                name: Some("document".to_string()),
                scopes: Some(vec!["view".to_string()]),
                ..Default::default()
            }]),
            ..Default::default()
        };

        let eval_response = PolicyEvaluationResponse {
            status: Some("PERMIT".to_string()),
            results: Some(vec![]),
            ..Default::default()
        };

        Mock::given(method("POST"))
            .and(path(
                "/admin/realms/master/clients/client-123/authz/resource-server/policy/evaluate",
            ))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .and(body_json(&eval_request))
            .respond_with(ResponseTemplate::new(200).set_body_json(&eval_response))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = PolicyEvaluateParams {
            realm: "master".to_string(),
            client_id: "client-123".to_string(),
            request: eval_request,
        };

        let result = authz_evaluate(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());

        let response = result.expect("Failed to evaluate");
        assert_eq!(response.status, Some("PERMIT".to_string()));
    }

    #[tokio::test]
    async fn test_authz_evaluate_deny() {
        let mock_server = MockServer::start().await;

        let eval_request = PolicyEvaluationRequest {
            user_id: Some("user-456".to_string()),
            resources: Some(vec![ResourceEvaluationRequest {
                name: Some("admin-resource".to_string()),
                scopes: Some(vec!["delete".to_string()]),
                ..Default::default()
            }]),
            ..Default::default()
        };

        let eval_response = PolicyEvaluationResponse {
            status: Some("DENY".to_string()),
            results: Some(vec![]),
            ..Default::default()
        };

        Mock::given(method("POST"))
            .and(path(
                "/admin/realms/master/clients/client-123/authz/resource-server/policy/evaluate",
            ))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .and(body_json(&eval_request))
            .respond_with(ResponseTemplate::new(200).set_body_json(&eval_response))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = PolicyEvaluateParams {
            realm: "master".to_string(),
            client_id: "client-123".to_string(),
            request: eval_request,
        };

        let result = authz_evaluate(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());

        let response = result.expect("Failed to evaluate");
        assert_eq!(response.status, Some("DENY".to_string()));
    }

    #[tokio::test]
    async fn test_authz_evaluate_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path(
                "/admin/realms/master/clients/client-123/authz/resource-server/policy/evaluate",
            ))
            .respond_with(ResponseTemplate::new(401))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = PolicyEvaluateParams {
            realm: "master".to_string(),
            client_id: "client-123".to_string(),
            request: PolicyEvaluationRequest::default(),
        };

        let result = authz_evaluate(&client, "invalid-token", &params).await;
        assert!(matches!(result, Err(ApiError::Unauthorized)));
    }

    #[tokio::test]
    async fn test_authz_evaluate_forbidden() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path(
                "/admin/realms/master/clients/client-123/authz/resource-server/policy/evaluate",
            ))
            .respond_with(ResponseTemplate::new(403))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = PolicyEvaluateParams {
            realm: "master".to_string(),
            client_id: "client-123".to_string(),
            request: PolicyEvaluationRequest::default(),
        };

        let result = authz_evaluate(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::Forbidden)));
    }

    #[tokio::test]
    async fn test_authz_permission_with_special_characters() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/my-realm%2Ftest/clients/client%2F123/authz/resource-server/permission/perm%2B1",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(sample_resource_permission()))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = PermissionGetParams {
            realm: "my-realm/test".to_string(),
            client_id: "client/123".to_string(),
            permission_id: "perm+1".to_string(),
        };

        let result = authz_permission_get(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }
}
