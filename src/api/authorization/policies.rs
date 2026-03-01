//! Keycloak Authorization Policies API.
//!
//! Provides CRUD operations for authorization policies.

use crate::api::{ApiError, KeycloakClient};

pub use super::types::{
    DecisionStrategy, Logic, PolicyCreateParams, PolicyDeleteParams, PolicyGetParams,
    PolicyListParams, PolicyRepresentation, PolicyUpdateParams,
};

/// List policies for a client's authorization resource server.
///
/// GET /admin/realms/{realm}/clients/{id}/authz/resource-server/policy
pub async fn authz_policies_list(
    client: &KeycloakClient,
    token: &str,
    params: &PolicyListParams,
) -> Result<Vec<PolicyRepresentation>, ApiError> {
    let mut path = format!(
        "/admin/realms/{}/clients/{}/authz/resource-server/policy",
        urlencoding::encode(&params.realm),
        urlencoding::encode(&params.client_id)
    );

    let mut query_parts = Vec::new();

    if let Some(ref name) = params.name {
        query_parts.push(format!("name={}", urlencoding::encode(name)));
    }

    if let Some(ref policy_type) = params.policy_type {
        query_parts.push(format!("type={}", urlencoding::encode(policy_type)));
    }

    if let Some(ref resource) = params.resource {
        query_parts.push(format!("resource={}", urlencoding::encode(resource)));
    }

    if let Some(ref scope) = params.scope {
        query_parts.push(format!("scope={}", urlencoding::encode(scope)));
    }

    if let Some(permission) = params.permission {
        query_parts.push(format!("permission={}", permission));
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

/// Get a single policy by ID.
///
/// GET /admin/realms/{realm}/clients/{id}/authz/resource-server/policy/{policy-id}
pub async fn authz_policy_get(
    client: &KeycloakClient,
    token: &str,
    params: &PolicyGetParams,
) -> Result<PolicyRepresentation, ApiError> {
    let path = format!(
        "/admin/realms/{}/clients/{}/authz/resource-server/policy/{}",
        urlencoding::encode(&params.realm),
        urlencoding::encode(&params.client_id),
        urlencoding::encode(&params.policy_id)
    );

    client.get(&path, token).await
}

/// Create a new policy.
///
/// POST /admin/realms/{realm}/clients/{id}/authz/resource-server/policy/{type}
pub async fn authz_policy_create(
    client: &KeycloakClient,
    token: &str,
    params: &PolicyCreateParams,
) -> Result<(), ApiError> {
    let path = format!(
        "/admin/realms/{}/clients/{}/authz/resource-server/policy/{}",
        urlencoding::encode(&params.realm),
        urlencoding::encode(&params.client_id),
        urlencoding::encode(&params.policy_type)
    );

    client.post_no_response(&path, token, &params.policy).await
}

/// Update an existing policy.
///
/// PUT /admin/realms/{realm}/clients/{id}/authz/resource-server/policy/{policy-id}
pub async fn authz_policy_update(
    client: &KeycloakClient,
    token: &str,
    params: &PolicyUpdateParams,
) -> Result<(), ApiError> {
    let path = format!(
        "/admin/realms/{}/clients/{}/authz/resource-server/policy/{}",
        urlencoding::encode(&params.realm),
        urlencoding::encode(&params.client_id),
        urlencoding::encode(&params.policy_id)
    );

    client.put(&path, token, &params.policy).await
}

/// Delete a policy.
///
/// DELETE /admin/realms/{realm}/clients/{id}/authz/resource-server/policy/{policy-id}
pub async fn authz_policy_delete(
    client: &KeycloakClient,
    token: &str,
    params: &PolicyDeleteParams,
) -> Result<(), ApiError> {
    let path = format!(
        "/admin/realms/{}/clients/{}/authz/resource-server/policy/{}",
        urlencoding::encode(&params.realm),
        urlencoding::encode(&params.client_id),
        urlencoding::encode(&params.policy_id)
    );

    client.delete(&path, token).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{body_json, header, method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    const TEST_TOKEN: &str = "test-bearer-token";

    fn sample_role_policy() -> PolicyRepresentation {
        PolicyRepresentation {
            id: Some("policy-123".to_string()),
            name: Some("Admin Role Policy".to_string()),
            description: Some("Policy for admin role access".to_string()),
            policy_type: Some("role".to_string()),
            logic: Some(Logic::Positive),
            decision_strategy: Some(DecisionStrategy::Unanimous),
            ..Default::default()
        }
    }

    #[tokio::test]
    async fn test_authz_policies_list_success() {
        let mock_server = MockServer::start().await;

        let expected_policies = vec![sample_role_policy()];

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/master/clients/client-123/authz/resource-server/policy",
            ))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_policies))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = PolicyListParams {
            realm: "master".to_string(),
            client_id: "client-123".to_string(),
            name: None,
            policy_type: None,
            resource: None,
            scope: None,
            permission: None,
            first: None,
            max: None,
        };

        let result = authz_policies_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());

        let policies = result.expect("Failed to get policies");
        assert_eq!(policies.len(), 1);
        assert_eq!(policies[0].name, Some("Admin Role Policy".to_string()));
    }

    #[tokio::test]
    async fn test_authz_policies_list_with_filters() {
        let mock_server = MockServer::start().await;

        let expected_policies = vec![sample_role_policy()];

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/test-realm/clients/client-456/authz/resource-server/policy",
            ))
            .and(query_param("name", "admin"))
            .and(query_param("type", "role"))
            .and(query_param("first", "0"))
            .and(query_param("max", "10"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_policies))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = PolicyListParams {
            realm: "test-realm".to_string(),
            client_id: "client-456".to_string(),
            name: Some("admin".to_string()),
            policy_type: Some("role".to_string()),
            resource: None,
            scope: None,
            permission: None,
            first: Some(0),
            max: Some(10),
        };

        let result = authz_policies_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_authz_policies_list_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/master/clients/client-123/authz/resource-server/policy",
            ))
            .respond_with(ResponseTemplate::new(401))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = PolicyListParams {
            realm: "master".to_string(),
            client_id: "client-123".to_string(),
            name: None,
            policy_type: None,
            resource: None,
            scope: None,
            permission: None,
            first: None,
            max: None,
        };

        let result = authz_policies_list(&client, "invalid-token", &params).await;
        assert!(matches!(result, Err(ApiError::Unauthorized)));
    }

    #[tokio::test]
    async fn test_authz_policies_list_forbidden() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/master/clients/client-123/authz/resource-server/policy",
            ))
            .respond_with(ResponseTemplate::new(403))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = PolicyListParams {
            realm: "master".to_string(),
            client_id: "client-123".to_string(),
            name: None,
            policy_type: None,
            resource: None,
            scope: None,
            permission: None,
            first: None,
            max: None,
        };

        let result = authz_policies_list(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::Forbidden)));
    }

    #[tokio::test]
    async fn test_authz_policy_get_success() {
        let mock_server = MockServer::start().await;

        let expected_policy = sample_role_policy();

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/master/clients/client-123/authz/resource-server/policy/policy-123",
            ))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_policy))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = PolicyGetParams {
            realm: "master".to_string(),
            client_id: "client-123".to_string(),
            policy_id: "policy-123".to_string(),
        };

        let result = authz_policy_get(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());

        let policy = result.expect("Failed to get policy");
        assert_eq!(policy.id, Some("policy-123".to_string()));
        assert_eq!(policy.name, Some("Admin Role Policy".to_string()));
    }

    #[tokio::test]
    async fn test_authz_policy_get_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/master/clients/client-123/authz/resource-server/policy/nonexistent",
            ))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = PolicyGetParams {
            realm: "master".to_string(),
            client_id: "client-123".to_string(),
            policy_id: "nonexistent".to_string(),
        };

        let result = authz_policy_get(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_authz_policy_create_success() {
        let mock_server = MockServer::start().await;

        let new_policy = PolicyRepresentation {
            name: Some("New Role Policy".to_string()),
            policy_type: Some("role".to_string()),
            logic: Some(Logic::Positive),
            ..Default::default()
        };

        Mock::given(method("POST"))
            .and(path(
                "/admin/realms/master/clients/client-123/authz/resource-server/policy/role",
            ))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .and(body_json(&new_policy))
            .respond_with(ResponseTemplate::new(201))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = PolicyCreateParams {
            realm: "master".to_string(),
            client_id: "client-123".to_string(),
            policy_type: "role".to_string(),
            policy: new_policy,
        };

        let result = authz_policy_create(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_authz_policy_create_conflict() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path(
                "/admin/realms/master/clients/client-123/authz/resource-server/policy/role",
            ))
            .respond_with(ResponseTemplate::new(409).set_body_json(serde_json::json!({
                "errorMessage": "Policy already exists"
            })))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = PolicyCreateParams {
            realm: "master".to_string(),
            client_id: "client-123".to_string(),
            policy_type: "role".to_string(),
            policy: PolicyRepresentation {
                name: Some("Existing Policy".to_string()),
                ..Default::default()
            },
        };

        let result = authz_policy_create(&client, TEST_TOKEN, &params).await;
        match result {
            Err(ApiError::Conflict(msg)) => {
                assert!(msg.contains("Policy already exists"));
            }
            _ => panic!("Expected Conflict error"),
        }
    }

    #[tokio::test]
    async fn test_authz_policy_update_success() {
        let mock_server = MockServer::start().await;

        let updated_policy = PolicyRepresentation {
            id: Some("policy-123".to_string()),
            name: Some("Updated Policy".to_string()),
            policy_type: Some("role".to_string()),
            logic: Some(Logic::Negative),
            ..Default::default()
        };

        Mock::given(method("PUT"))
            .and(path(
                "/admin/realms/master/clients/client-123/authz/resource-server/policy/policy-123",
            ))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .and(body_json(&updated_policy))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = PolicyUpdateParams {
            realm: "master".to_string(),
            client_id: "client-123".to_string(),
            policy_id: "policy-123".to_string(),
            policy: updated_policy,
        };

        let result = authz_policy_update(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_authz_policy_update_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path(
                "/admin/realms/master/clients/client-123/authz/resource-server/policy/nonexistent",
            ))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = PolicyUpdateParams {
            realm: "master".to_string(),
            client_id: "client-123".to_string(),
            policy_id: "nonexistent".to_string(),
            policy: PolicyRepresentation::default(),
        };

        let result = authz_policy_update(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_authz_policy_delete_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path(
                "/admin/realms/master/clients/client-123/authz/resource-server/policy/policy-123",
            ))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = PolicyDeleteParams {
            realm: "master".to_string(),
            client_id: "client-123".to_string(),
            policy_id: "policy-123".to_string(),
        };

        let result = authz_policy_delete(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_authz_policy_delete_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path(
                "/admin/realms/master/clients/client-123/authz/resource-server/policy/nonexistent",
            ))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = PolicyDeleteParams {
            realm: "master".to_string(),
            client_id: "client-123".to_string(),
            policy_id: "nonexistent".to_string(),
        };

        let result = authz_policy_delete(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_authz_policy_with_special_characters() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/my-realm%2Ftest/clients/client%2F123/authz/resource-server/policy/policy%2B1",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(sample_role_policy()))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = PolicyGetParams {
            realm: "my-realm/test".to_string(),
            client_id: "client/123".to_string(),
            policy_id: "policy+1".to_string(),
        };

        let result = authz_policy_get(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }
}
