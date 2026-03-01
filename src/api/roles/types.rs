//! Type definitions for Keycloak Role representations.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a role within Keycloak, which can be assigned to users or groups.
///
/// Roles can be realm-level or client-specific, and can be composite (containing other roles).
#[derive(Debug, Clone, Serialize, Deserialize, Default, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RoleRepresentation {
    /// The unique identifier for the role
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The name of the role
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// A description of the role
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Indicates if the role is a composite role (contains other roles)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub composite: Option<bool>,

    /// Indicates if this is a client-specific role
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_role: Option<bool>,

    /// The ID of the realm or client that contains this role
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_id: Option<String>,

    /// Custom attributes associated with the role
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<HashMap<String, Vec<String>>>,
}

/// Parameters for listing realm roles.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RoleListParams {
    /// The realm name (not id!)
    pub realm: String,

    /// Search string to filter roles by name
    #[serde(default)]
    pub search: Option<String>,

    /// Pagination offset (0-based)
    #[serde(default)]
    pub first: Option<i32>,

    /// Maximum number of results to return
    #[serde(default)]
    pub max: Option<i32>,

    /// If true, returns only basic role info (id, name, description)
    #[serde(default)]
    pub brief_representation: Option<bool>,
}

/// Parameters for getting a single realm role by name.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RoleGetParams {
    /// The realm name (not id!)
    pub realm: String,

    /// The role name (not id!)
    pub role_name: String,
}

/// Parameters for creating a new realm role.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RoleCreateParams {
    /// The realm name (not id!)
    pub realm: String,

    /// The name of the new role (required)
    pub name: String,

    /// A description of the role
    #[serde(default)]
    pub description: Option<String>,

    /// Custom attributes for the role
    #[serde(default)]
    pub attributes: Option<HashMap<String, Vec<String>>>,
}

/// Parameters for updating an existing realm role.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RoleUpdateParams {
    /// The realm name (not id!)
    pub realm: String,

    /// The current role name (not id!)
    pub role_name: String,

    /// The new name for the role (if renaming)
    #[serde(default)]
    pub new_name: Option<String>,

    /// The new description for the role
    #[serde(default)]
    pub description: Option<String>,

    /// Custom attributes for the role
    #[serde(default)]
    pub attributes: Option<HashMap<String, Vec<String>>>,
}

/// Parameters for deleting a realm role.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RoleDeleteParams {
    /// The realm name (not id!)
    pub realm: String,

    /// The role name (not id!)
    pub role_name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_role_representation_default() {
        let role = RoleRepresentation::default();
        assert!(role.id.is_none());
        assert!(role.name.is_none());
        assert!(role.description.is_none());
        assert!(role.composite.is_none());
        assert!(role.client_role.is_none());
        assert!(role.container_id.is_none());
        assert!(role.attributes.is_none());
    }

    #[test]
    fn test_role_representation_serialization() {
        let role = RoleRepresentation {
            id: Some("123".to_string()),
            name: Some("admin".to_string()),
            description: Some("Admin role".to_string()),
            composite: Some(false),
            client_role: Some(false),
            container_id: Some("realm-id".to_string()),
            attributes: None,
        };

        let json = serde_json::to_string(&role).unwrap();
        assert!(json.contains("\"id\":\"123\""));
        assert!(json.contains("\"name\":\"admin\""));
        assert!(json.contains("\"description\":\"Admin role\""));
        assert!(json.contains("\"composite\":false"));
        assert!(json.contains("\"clientRole\":false"));
        assert!(json.contains("\"containerId\":\"realm-id\""));
        assert!(!json.contains("attributes"));
    }

    #[test]
    fn test_role_representation_deserialization() {
        let json = r#"{
            "id": "456",
            "name": "user",
            "description": "Regular user role",
            "composite": true,
            "clientRole": false,
            "containerId": "my-realm",
            "attributes": {"key": ["value1", "value2"]}
        }"#;

        let role: RoleRepresentation = serde_json::from_str(json).unwrap();
        assert_eq!(role.id, Some("456".to_string()));
        assert_eq!(role.name, Some("user".to_string()));
        assert_eq!(role.description, Some("Regular user role".to_string()));
        assert_eq!(role.composite, Some(true));
        assert_eq!(role.client_role, Some(false));
        assert_eq!(role.container_id, Some("my-realm".to_string()));

        let attrs = role.attributes.unwrap();
        assert_eq!(
            attrs.get("key").unwrap(),
            &vec!["value1".to_string(), "value2".to_string()]
        );
    }

    #[test]
    fn test_role_representation_skips_none_fields() {
        let role = RoleRepresentation {
            name: Some("test-role".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&role).unwrap();
        assert!(json.contains("\"name\":\"test-role\""));
        assert!(!json.contains("\"id\""));
        assert!(!json.contains("\"description\""));
        assert!(!json.contains("\"composite\""));
    }
}
