//! Type definitions for Keycloak Authorization Services API.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Resource Server (Authorization Settings) representation.
///
/// Represents the authorization settings for a client that has authorization services enabled.
#[derive(Debug, Clone, Serialize, Deserialize, Default, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ResourceServerRepresentation {
    /// Unique identifier for the resource server (same as client UUID)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Client ID (same as the client's client_id)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,

    /// Display name for the resource server
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Whether remote resource management is allowed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_remote_resource_management: Option<bool>,

    /// Policy enforcement mode: ENFORCING, PERMISSIVE, or DISABLED
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_enforcement_mode: Option<String>,

    /// Decision strategy: UNANIMOUS or AFFIRMATIVE
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision_strategy: Option<String>,
}

/// Resource representation.
///
/// Represents a protected resource in the authorization server.
#[derive(Debug, Clone, Serialize, Deserialize, Default, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ResourceRepresentation {
    /// Unique identifier for the resource (UUID)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_id")]
    pub id: Option<String>,

    /// Name of the resource
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Display name of the resource
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// URIs associated with this resource
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uris: Option<Vec<String>>,

    /// Type of the resource
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub resource_type: Option<String>,

    /// Icon URI for the resource
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_uri: Option<String>,

    /// Resource owner information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<ResourceOwner>,

    /// Whether the resource is managed by the owner
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_managed_access: Option<bool>,

    /// Scopes associated with this resource
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<ScopeRepresentation>>,

    /// Additional attributes for the resource
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<HashMap<String, Vec<String>>>,
}

/// Resource owner information.
#[derive(Debug, Clone, Serialize, Deserialize, Default, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ResourceOwner {
    /// Owner ID (user or client ID)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Owner name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Scope representation.
///
/// Represents a scope that can be associated with resources.
#[derive(Debug, Clone, Serialize, Deserialize, Default, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ScopeRepresentation {
    /// Unique identifier for the scope (UUID)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Name of the scope
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Display name of the scope
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// Icon URI for the scope
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_uri: Option<String>,
}

/// Parameters for getting a resource server.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ResourceServerGetParams {
    /// The realm name
    pub realm: String,

    /// The client UUID
    pub id: String,
}

/// Parameters for updating a resource server.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ResourceServerUpdateParams {
    /// The realm name
    pub realm: String,

    /// The client UUID
    pub id: String,

    /// The updated resource server representation
    pub resource_server: ResourceServerRepresentation,
}

/// Parameters for listing resources.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ResourceListParams {
    /// The realm name
    pub realm: String,

    /// The client UUID
    pub id: String,

    /// Filter by resource name
    #[serde(default)]
    pub name: Option<String>,

    /// Filter by resource URI
    #[serde(default)]
    pub uri: Option<String>,

    /// Filter by resource owner
    #[serde(default)]
    pub owner: Option<String>,

    /// Filter by resource type
    #[serde(default)]
    pub resource_type: Option<String>,

    /// Filter by scope name
    #[serde(default)]
    pub scope: Option<String>,

    /// Pagination: first result index
    #[serde(default)]
    pub first: Option<i32>,

    /// Pagination: max results to return
    #[serde(default)]
    pub max: Option<i32>,

    /// Whether to deep search
    #[serde(default)]
    pub deep: Option<bool>,

    /// Whether to match exactly
    #[serde(default)]
    pub exact_name: Option<bool>,
}

/// Parameters for getting a single resource.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ResourceGetParams {
    /// The realm name
    pub realm: String,

    /// The client UUID
    pub id: String,

    /// The resource UUID
    pub resource_id: String,
}

/// Parameters for creating a resource.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ResourceCreateParams {
    /// The realm name
    pub realm: String,

    /// The client UUID
    pub id: String,

    /// The resource representation to create
    pub resource: ResourceRepresentation,
}

/// Parameters for updating a resource.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ResourceUpdateParams {
    /// The realm name
    pub realm: String,

    /// The client UUID
    pub id: String,

    /// The resource UUID
    pub resource_id: String,

    /// The updated resource representation
    pub resource: ResourceRepresentation,
}

/// Parameters for deleting a resource.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ResourceDeleteParams {
    /// The realm name
    pub realm: String,

    /// The client UUID
    pub id: String,

    /// The resource UUID
    pub resource_id: String,
}

/// Parameters for listing scopes.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ScopeListParams {
    /// The realm name
    pub realm: String,

    /// The client UUID
    pub id: String,

    /// Filter by scope name
    #[serde(default)]
    pub name: Option<String>,

    /// Pagination: first result index
    #[serde(default)]
    pub first: Option<i32>,

    /// Pagination: max results to return
    #[serde(default)]
    pub max: Option<i32>,
}

/// Parameters for creating a scope.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ScopeCreateParams {
    /// The realm name
    pub realm: String,

    /// The client UUID
    pub id: String,

    /// The scope representation to create
    pub scope: ScopeRepresentation,
}

/// Parameters for deleting a scope.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ScopeDeleteParams {
    /// The realm name
    pub realm: String,

    /// The client UUID
    pub id: String,

    /// The scope UUID
    pub scope_id: String,
}

/// Logic operators for policies.
#[derive(Debug, Clone, Serialize, Deserialize, Default, schemars::JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Logic {
    #[default]
    Positive,
    Negative,
}

/// Decision strategy for policies.
#[derive(Debug, Clone, Serialize, Deserialize, Default, schemars::JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DecisionStrategy {
    #[default]
    Unanimous,
    Affirmative,
    Consensus,
}

/// Keycloak Policy representation.
///
/// Represents an authorization policy that defines conditions for granting access.
/// Policy types include: role, user, client, group, js, time, aggregate.
#[derive(Debug, Clone, Serialize, Deserialize, Default, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PolicyRepresentation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub policy_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub logic: Option<Logic>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision_strategy: Option<DecisionStrategy>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<HashMap<String, String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<RoleDefinition>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<GroupDefinition>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub clients: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_before: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_on_or_after: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_month: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_month_end: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub month_end: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hour: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hour_end: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub minute: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub minute_end: Option<String>,
}

/// Role definition for role-based policies.
#[derive(Debug, Clone, Serialize, Deserialize, Default, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RoleDefinition {
    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}

/// Group definition for group-based policies.
#[derive(Debug, Clone, Serialize, Deserialize, Default, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GroupDefinition {
    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extend_children: Option<bool>,
}

/// Keycloak Permission representation.
///
/// Represents an authorization permission that associates policies with resources/scopes.
/// Permission types include: resource, scope.
#[derive(Debug, Clone, Serialize, Deserialize, Default, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PermissionRepresentation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub permission_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub logic: Option<Logic>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision_strategy: Option<DecisionStrategy>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
}

/// Request for policy evaluation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PolicyEvaluationRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_ids: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<ResourceEvaluationRequest>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlements: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<EvaluationContext>,
}

/// Resource evaluation request for policy evaluation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ResourceEvaluationRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub resource_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,
}

/// Context for policy evaluation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct EvaluationContext {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<HashMap<String, Vec<String>>>,
}

/// Response from policy evaluation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PolicyEvaluationResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<EvaluationResult>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlements: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rpt: Option<serde_json::Value>,
}

/// Result of a single resource evaluation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct EvaluationResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<ResourceRepresentation>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<ScopeRepresentation>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<PolicyResultRepresentation>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_scopes: Option<Vec<ScopeRepresentation>>,
}

/// Policy result in evaluation response.
#[derive(Debug, Clone, Serialize, Deserialize, Default, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PolicyResultRepresentation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<PolicyRepresentation>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_policies: Option<Vec<PolicyResultRepresentation>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,
}

/// Parameters for listing policies.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PolicyListParams {
    pub realm: String,

    pub client_id: String,

    #[serde(default)]
    pub name: Option<String>,

    #[serde(default, rename = "type")]
    pub policy_type: Option<String>,

    #[serde(default)]
    pub resource: Option<String>,

    #[serde(default)]
    pub scope: Option<String>,

    #[serde(default)]
    pub permission: Option<bool>,

    #[serde(default)]
    pub first: Option<i32>,

    #[serde(default)]
    pub max: Option<i32>,
}

/// Parameters for getting a single policy.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PolicyGetParams {
    pub realm: String,

    pub client_id: String,

    pub policy_id: String,
}

/// Parameters for creating a policy.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PolicyCreateParams {
    pub realm: String,

    pub client_id: String,

    pub policy_type: String,

    pub policy: PolicyRepresentation,
}

/// Parameters for updating a policy.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PolicyUpdateParams {
    pub realm: String,

    pub client_id: String,

    pub policy_id: String,

    pub policy: PolicyRepresentation,
}

/// Parameters for deleting a policy.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PolicyDeleteParams {
    pub realm: String,

    pub client_id: String,

    pub policy_id: String,
}

/// Parameters for listing permissions.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PermissionListParams {
    pub realm: String,

    pub client_id: String,

    #[serde(default)]
    pub name: Option<String>,

    #[serde(default, rename = "type")]
    pub permission_type: Option<String>,

    #[serde(default)]
    pub resource: Option<String>,

    #[serde(default)]
    pub scope: Option<String>,

    #[serde(default)]
    pub first: Option<i32>,

    #[serde(default)]
    pub max: Option<i32>,
}

/// Parameters for getting a single permission.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PermissionGetParams {
    pub realm: String,

    pub client_id: String,

    pub permission_id: String,
}

/// Parameters for creating a permission.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PermissionCreateParams {
    pub realm: String,

    pub client_id: String,

    pub permission_type: String,

    pub permission: PermissionRepresentation,
}

/// Parameters for updating a permission.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PermissionUpdateParams {
    pub realm: String,

    pub client_id: String,

    pub permission_id: String,

    pub permission: PermissionRepresentation,
}

/// Parameters for deleting a permission.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PermissionDeleteParams {
    pub realm: String,

    pub client_id: String,

    pub permission_id: String,
}

/// Parameters for evaluating policies.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PolicyEvaluateParams {
    pub realm: String,

    pub client_id: String,

    pub request: PolicyEvaluationRequest,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_server_representation_default() {
        let rs = ResourceServerRepresentation::default();
        assert!(rs.id.is_none());
        assert!(rs.client_id.is_none());
        assert!(rs.name.is_none());
        assert!(rs.allow_remote_resource_management.is_none());
        assert!(rs.policy_enforcement_mode.is_none());
        assert!(rs.decision_strategy.is_none());
    }

    #[test]
    fn test_resource_server_representation_serialization() {
        let rs = ResourceServerRepresentation {
            id: Some("abc-123".to_string()),
            client_id: Some("my-client".to_string()),
            name: Some("My Resource Server".to_string()),
            allow_remote_resource_management: Some(true),
            policy_enforcement_mode: Some("ENFORCING".to_string()),
            decision_strategy: Some("UNANIMOUS".to_string()),
        };

        let json = serde_json::to_string(&rs).expect("Failed to serialize");
        assert!(json.contains("\"id\":\"abc-123\""));
        assert!(json.contains("\"clientId\":\"my-client\""));
        assert!(json.contains("\"allowRemoteResourceManagement\":true"));
        assert!(json.contains("\"policyEnforcementMode\":\"ENFORCING\""));
        assert!(json.contains("\"decisionStrategy\":\"UNANIMOUS\""));
    }

    #[test]
    fn test_resource_server_representation_deserialization() {
        let json = r#"{
            "id": "abc-123",
            "clientId": "my-client",
            "name": "My Resource Server",
            "allowRemoteResourceManagement": true,
            "policyEnforcementMode": "ENFORCING",
            "decisionStrategy": "AFFIRMATIVE"
        }"#;

        let rs: ResourceServerRepresentation =
            serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(rs.id, Some("abc-123".to_string()));
        assert_eq!(rs.client_id, Some("my-client".to_string()));
        assert_eq!(rs.name, Some("My Resource Server".to_string()));
        assert_eq!(rs.allow_remote_resource_management, Some(true));
        assert_eq!(rs.policy_enforcement_mode, Some("ENFORCING".to_string()));
        assert_eq!(rs.decision_strategy, Some("AFFIRMATIVE".to_string()));
    }

    #[test]
    fn test_resource_representation_default() {
        let resource = ResourceRepresentation::default();
        assert!(resource.id.is_none());
        assert!(resource.name.is_none());
        assert!(resource.display_name.is_none());
        assert!(resource.uris.is_none());
        assert!(resource.resource_type.is_none());
        assert!(resource.icon_uri.is_none());
        assert!(resource.owner.is_none());
        assert!(resource.owner_managed_access.is_none());
        assert!(resource.scopes.is_none());
        assert!(resource.attributes.is_none());
    }

    #[test]
    fn test_resource_representation_serialization() {
        let resource = ResourceRepresentation {
            id: Some("resource-123".to_string()),
            name: Some("My Resource".to_string()),
            display_name: Some("My Display Name".to_string()),
            uris: Some(vec!["/api/resources/*".to_string()]),
            resource_type: Some("urn:my-app:resources:default".to_string()),
            icon_uri: Some("http://example.com/icon.png".to_string()),
            owner: Some(ResourceOwner {
                id: Some("user-123".to_string()),
                name: Some("testuser".to_string()),
            }),
            owner_managed_access: Some(false),
            scopes: Some(vec![ScopeRepresentation {
                id: Some("scope-1".to_string()),
                name: Some("read".to_string()),
                display_name: None,
                icon_uri: None,
            }]),
            attributes: None,
        };

        let json = serde_json::to_string(&resource).expect("Failed to serialize");
        assert!(json.contains("\"_id\":\"resource-123\""));
        assert!(json.contains("\"name\":\"My Resource\""));
        assert!(json.contains("\"displayName\":\"My Display Name\""));
        assert!(json.contains("\"/api/resources/*\""));
        assert!(json.contains("\"type\":\"urn:my-app:resources:default\""));
        assert!(json.contains("\"ownerManagedAccess\":false"));
    }

    #[test]
    fn test_resource_representation_deserialization() {
        let json = r#"{
            "_id": "resource-123",
            "name": "My Resource",
            "displayName": "My Display Name",
            "uris": ["/api/resources/*"],
            "type": "urn:my-app:resources:default",
            "iconUri": "http://example.com/icon.png",
            "owner": {
                "id": "user-123",
                "name": "testuser"
            },
            "ownerManagedAccess": true,
            "scopes": [
                {"id": "scope-1", "name": "read"}
            ],
            "attributes": {
                "key1": ["value1", "value2"]
            }
        }"#;

        let resource: ResourceRepresentation =
            serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(resource.id, Some("resource-123".to_string()));
        assert_eq!(resource.name, Some("My Resource".to_string()));
        assert_eq!(resource.display_name, Some("My Display Name".to_string()));
        assert_eq!(resource.uris, Some(vec!["/api/resources/*".to_string()]));
        assert_eq!(
            resource.resource_type,
            Some("urn:my-app:resources:default".to_string())
        );
        assert_eq!(
            resource.icon_uri,
            Some("http://example.com/icon.png".to_string())
        );
        assert!(resource.owner.is_some());
        let owner = resource.owner.unwrap();
        assert_eq!(owner.id, Some("user-123".to_string()));
        assert_eq!(owner.name, Some("testuser".to_string()));
        assert_eq!(resource.owner_managed_access, Some(true));
        assert!(resource.scopes.is_some());
        assert!(resource.attributes.is_some());
    }

    #[test]
    fn test_scope_representation_default() {
        let scope = ScopeRepresentation::default();
        assert!(scope.id.is_none());
        assert!(scope.name.is_none());
        assert!(scope.display_name.is_none());
        assert!(scope.icon_uri.is_none());
    }

    #[test]
    fn test_scope_representation_serialization() {
        let scope = ScopeRepresentation {
            id: Some("scope-123".to_string()),
            name: Some("read".to_string()),
            display_name: Some("Read Access".to_string()),
            icon_uri: Some("http://example.com/read-icon.png".to_string()),
        };

        let json = serde_json::to_string(&scope).expect("Failed to serialize");
        assert!(json.contains("\"id\":\"scope-123\""));
        assert!(json.contains("\"name\":\"read\""));
        assert!(json.contains("\"displayName\":\"Read Access\""));
        assert!(json.contains("\"iconUri\":\"http://example.com/read-icon.png\""));
    }

    #[test]
    fn test_scope_representation_deserialization() {
        let json = r#"{
            "id": "scope-123",
            "name": "write",
            "displayName": "Write Access",
            "iconUri": "http://example.com/write-icon.png"
        }"#;

        let scope: ScopeRepresentation = serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(scope.id, Some("scope-123".to_string()));
        assert_eq!(scope.name, Some("write".to_string()));
        assert_eq!(scope.display_name, Some("Write Access".to_string()));
        assert_eq!(
            scope.icon_uri,
            Some("http://example.com/write-icon.png".to_string())
        );
    }

    #[test]
    fn test_resource_list_params_deserialization() {
        let json = r#"{
            "realm": "master",
            "id": "client-123",
            "name": "my-resource",
            "first": 0,
            "max": 100
        }"#;

        let params: ResourceListParams = serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(params.realm, "master");
        assert_eq!(params.id, "client-123");
        assert_eq!(params.name, Some("my-resource".to_string()));
        assert_eq!(params.first, Some(0));
        assert_eq!(params.max, Some(100));
    }

    #[test]
    fn test_resource_list_params_minimal() {
        let json = r#"{"realm": "test-realm", "id": "client-456"}"#;

        let params: ResourceListParams = serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(params.realm, "test-realm");
        assert_eq!(params.id, "client-456");
        assert!(params.name.is_none());
        assert!(params.uri.is_none());
        assert!(params.owner.is_none());
        assert!(params.resource_type.is_none());
        assert!(params.scope.is_none());
        assert!(params.first.is_none());
        assert!(params.max.is_none());
    }

    #[test]
    fn test_scope_list_params_deserialization() {
        let json = r#"{
            "realm": "master",
            "id": "client-123",
            "name": "read",
            "first": 0,
            "max": 50
        }"#;

        let params: ScopeListParams = serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(params.realm, "master");
        assert_eq!(params.id, "client-123");
        assert_eq!(params.name, Some("read".to_string()));
        assert_eq!(params.first, Some(0));
        assert_eq!(params.max, Some(50));
    }

    #[test]
    fn test_resource_owner_deserialization() {
        let json = r#"{
            "id": "owner-123",
            "name": "Resource Owner"
        }"#;

        let owner: ResourceOwner = serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(owner.id, Some("owner-123".to_string()));
        assert_eq!(owner.name, Some("Resource Owner".to_string()));
    }

    #[test]
    fn test_logic_serialization() {
        let positive = Logic::Positive;
        let negative = Logic::Negative;

        assert_eq!(serde_json::to_string(&positive).unwrap(), "\"POSITIVE\"");
        assert_eq!(serde_json::to_string(&negative).unwrap(), "\"NEGATIVE\"");
    }

    #[test]
    fn test_decision_strategy_serialization() {
        let unanimous = DecisionStrategy::Unanimous;
        let affirmative = DecisionStrategy::Affirmative;
        let consensus = DecisionStrategy::Consensus;

        assert_eq!(serde_json::to_string(&unanimous).unwrap(), "\"UNANIMOUS\"");
        assert_eq!(
            serde_json::to_string(&affirmative).unwrap(),
            "\"AFFIRMATIVE\""
        );
        assert_eq!(serde_json::to_string(&consensus).unwrap(), "\"CONSENSUS\"");
    }

    #[test]
    fn test_policy_representation_default() {
        let policy = PolicyRepresentation::default();
        assert!(policy.id.is_none());
        assert!(policy.name.is_none());
        assert!(policy.policy_type.is_none());
    }

    #[test]
    fn test_policy_representation_serialization() {
        let policy = PolicyRepresentation {
            name: Some("admin-policy".to_string()),
            policy_type: Some("role".to_string()),
            logic: Some(Logic::Positive),
            decision_strategy: Some(DecisionStrategy::Unanimous),
            ..Default::default()
        };

        let json = serde_json::to_string(&policy).expect("Failed to serialize");
        assert!(json.contains("\"name\":\"admin-policy\""));
        assert!(json.contains("\"type\":\"role\""));
        assert!(json.contains("\"logic\":\"POSITIVE\""));
        assert!(json.contains("\"decisionStrategy\":\"UNANIMOUS\""));
    }

    #[test]
    fn test_policy_representation_deserialization() {
        let json = r#"{
            "id": "policy-123",
            "name": "Admin Role Policy",
            "description": "Policy for admin role",
            "type": "role",
            "logic": "POSITIVE",
            "decisionStrategy": "UNANIMOUS",
            "roles": [{"id": "role-1", "required": true}]
        }"#;

        let policy: PolicyRepresentation =
            serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(policy.id, Some("policy-123".to_string()));
        assert_eq!(policy.name, Some("Admin Role Policy".to_string()));
        assert_eq!(policy.policy_type, Some("role".to_string()));
        assert!(matches!(policy.logic, Some(Logic::Positive)));
        assert!(matches!(
            policy.decision_strategy,
            Some(DecisionStrategy::Unanimous)
        ));
        assert!(policy.roles.is_some());
        let roles = policy.roles.unwrap();
        assert_eq!(roles.len(), 1);
        assert_eq!(roles[0].id, "role-1");
        assert_eq!(roles[0].required, Some(true));
    }

    #[test]
    fn test_permission_representation_default() {
        let perm = PermissionRepresentation::default();
        assert!(perm.id.is_none());
        assert!(perm.name.is_none());
    }

    #[test]
    fn test_permission_representation_serialization() {
        let perm = PermissionRepresentation {
            name: Some("resource-permission".to_string()),
            permission_type: Some("resource".to_string()),
            resources: Some(vec!["resource-1".to_string()]),
            policies: Some(vec!["policy-1".to_string()]),
            ..Default::default()
        };

        let json = serde_json::to_string(&perm).expect("Failed to serialize");
        assert!(json.contains("\"name\":\"resource-permission\""));
        assert!(json.contains("\"type\":\"resource\""));
        assert!(json.contains("\"resources\":[\"resource-1\"]"));
        assert!(json.contains("\"policies\":[\"policy-1\"]"));
    }

    #[test]
    fn test_permission_representation_deserialization() {
        let json = r#"{
            "id": "perm-123",
            "name": "View Resource Permission",
            "type": "resource",
            "logic": "POSITIVE",
            "decisionStrategy": "AFFIRMATIVE",
            "resources": ["res-1", "res-2"],
            "scopes": ["view", "edit"],
            "policies": ["policy-1"]
        }"#;

        let perm: PermissionRepresentation =
            serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(perm.id, Some("perm-123".to_string()));
        assert_eq!(perm.name, Some("View Resource Permission".to_string()));
        assert_eq!(perm.permission_type, Some("resource".to_string()));
        assert_eq!(
            perm.resources,
            Some(vec!["res-1".to_string(), "res-2".to_string()])
        );
        assert_eq!(
            perm.scopes,
            Some(vec!["view".to_string(), "edit".to_string()])
        );
        assert_eq!(perm.policies, Some(vec!["policy-1".to_string()]));
    }

    #[test]
    fn test_policy_evaluation_request_serialization() {
        let request = PolicyEvaluationRequest {
            user_id: Some("user-123".to_string()),
            resources: Some(vec![ResourceEvaluationRequest {
                name: Some("document".to_string()),
                scopes: Some(vec!["view".to_string()]),
                ..Default::default()
            }]),
            ..Default::default()
        };

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        assert!(json.contains("\"userId\":\"user-123\""));
        assert!(json.contains("\"name\":\"document\""));
        assert!(json.contains("\"scopes\":[\"view\"]"));
    }

    #[test]
    fn test_policy_evaluation_response_deserialization() {
        let json = r#"{
            "results": [{
                "resource": {"_id": "res-1", "name": "Document"},
                "status": "PERMIT",
                "scopes": [{"id": "scope-1", "name": "view"}]
            }],
            "status": "PERMIT"
        }"#;

        let response: PolicyEvaluationResponse =
            serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(response.status, Some("PERMIT".to_string()));
        assert!(response.results.is_some());
        let results = response.results.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].status, Some("PERMIT".to_string()));
    }

    #[test]
    fn test_policy_list_params_deserialization() {
        let json = r#"{
            "realm": "master",
            "clientId": "client-123",
            "name": "admin",
            "type": "role",
            "first": 0,
            "max": 10
        }"#;

        let params: PolicyListParams = serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(params.realm, "master");
        assert_eq!(params.client_id, "client-123");
        assert_eq!(params.name, Some("admin".to_string()));
        assert_eq!(params.policy_type, Some("role".to_string()));
        assert_eq!(params.first, Some(0));
        assert_eq!(params.max, Some(10));
    }

    #[test]
    fn test_policy_create_params_deserialization() {
        let json = r#"{
            "realm": "master",
            "clientId": "client-123",
            "policyType": "role",
            "policy": {
                "name": "Admin Policy",
                "type": "role"
            }
        }"#;

        let params: PolicyCreateParams = serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(params.realm, "master");
        assert_eq!(params.client_id, "client-123");
        assert_eq!(params.policy_type, "role");
        assert_eq!(params.policy.name, Some("Admin Policy".to_string()));
    }

    #[test]
    fn test_permission_list_params_deserialization() {
        let json = r#"{
            "realm": "test-realm",
            "clientId": "client-456",
            "name": "view",
            "type": "resource"
        }"#;

        let params: PermissionListParams =
            serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(params.realm, "test-realm");
        assert_eq!(params.client_id, "client-456");
        assert_eq!(params.name, Some("view".to_string()));
        assert_eq!(params.permission_type, Some("resource".to_string()));
    }

    #[test]
    fn test_role_definition_serialization() {
        let role = RoleDefinition {
            id: "role-123".to_string(),
            required: Some(true),
        };

        let json = serde_json::to_string(&role).expect("Failed to serialize");
        assert!(json.contains("\"id\":\"role-123\""));
        assert!(json.contains("\"required\":true"));
    }

    #[test]
    fn test_group_definition_serialization() {
        let group = GroupDefinition {
            id: "group-123".to_string(),
            extend_children: Some(true),
        };

        let json = serde_json::to_string(&group).expect("Failed to serialize");
        assert!(json.contains("\"id\":\"group-123\""));
        assert!(json.contains("\"extendChildren\":true"));
    }
}
