//! Type definitions for Keycloak Group representations.
//!
//! This module contains the data structures for representing groups in the
//! Keycloak Admin REST API, including full support for hierarchical subGroups.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a group in Keycloak with full hierarchical support.
///
/// Groups are containers for users and can have parent-child relationships.
/// This representation includes all fields from the Keycloak Groups API.
#[derive(Debug, Clone, Serialize, Deserialize, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GroupRepresentation {
    /// Unique identifier for the group (UUID)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Group name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Hierarchical path to this group (e.g., "/parent/child")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    /// Parent group ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,

    /// Child groups (hierarchical structure)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_groups: Option<Vec<GroupRepresentation>>,

    /// Custom attributes associated with the group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<HashMap<String, Vec<String>>>,

    /// Realm roles directly assigned to this group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub realm_roles: Option<Vec<String>>,

    /// Client roles assigned to this group (client ID -> role names)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_roles: Option<HashMap<String, Vec<String>>>,

    /// Access control settings for the group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access: Option<HashMap<String, bool>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_representation_default() {
        let group = GroupRepresentation::default();
        assert!(group.id.is_none());
        assert!(group.name.is_none());
        assert!(group.path.is_none());
        assert!(group.parent_id.is_none());
        assert!(group.sub_groups.is_none());
        assert!(group.attributes.is_none());
        assert!(group.realm_roles.is_none());
        assert!(group.client_roles.is_none());
        assert!(group.access.is_none());
    }

    #[test]
    fn test_group_representation_serialization() {
        let group = GroupRepresentation {
            id: Some("group-123".to_string()),
            name: Some("Developers".to_string()),
            path: Some("/Developers".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&group).unwrap();
        assert!(json.contains("\"id\":\"group-123\""));
        assert!(json.contains("\"name\":\"Developers\""));
        assert!(json.contains("\"path\":\"/Developers\""));
        // Verify None fields are not serialized
        assert!(!json.contains("\"parentId\""));
        assert!(!json.contains("\"subGroups\""));
    }

    #[test]
    fn test_group_representation_deserialization() {
        let json = r#"{
            "id": "abc-123",
            "name": "Engineering",
            "path": "/Engineering",
            "parentId": null,
            "subGroups": [
                {
                    "id": "def-456",
                    "name": "Backend",
                    "path": "/Engineering/Backend"
                }
            ]
        }"#;

        let group: GroupRepresentation = serde_json::from_str(json).unwrap();
        assert_eq!(group.id.as_deref(), Some("abc-123"));
        assert_eq!(group.name.as_deref(), Some("Engineering"));
        assert_eq!(group.path.as_deref(), Some("/Engineering"));

        let sub_groups = group.sub_groups.unwrap();
        assert_eq!(sub_groups.len(), 1);
        assert_eq!(sub_groups[0].name.as_deref(), Some("Backend"));
    }

    #[test]
    fn test_group_with_attributes() {
        let mut attrs = HashMap::new();
        attrs.insert("department".to_string(), vec!["engineering".to_string()]);
        attrs.insert(
            "locations".to_string(),
            vec!["NYC".to_string(), "SF".to_string()],
        );

        let group = GroupRepresentation {
            name: Some("Developers".to_string()),
            attributes: Some(attrs),
            ..Default::default()
        };

        let json = serde_json::to_string(&group).unwrap();
        assert!(json.contains("\"attributes\""));
        assert!(json.contains("\"department\""));
    }

    #[test]
    fn test_group_with_roles() {
        let mut client_roles = HashMap::new();
        client_roles.insert(
            "my-app".to_string(),
            vec!["viewer".to_string(), "editor".to_string()],
        );

        let group = GroupRepresentation {
            name: Some("Admins".to_string()),
            realm_roles: Some(vec!["admin".to_string(), "user".to_string()]),
            client_roles: Some(client_roles),
            ..Default::default()
        };

        let json = serde_json::to_string(&group).unwrap();
        assert!(json.contains("\"realmRoles\""));
        assert!(json.contains("\"clientRoles\""));
    }

    #[test]
    fn test_group_with_access() {
        let mut access = HashMap::new();
        access.insert("view".to_string(), true);
        access.insert("manage".to_string(), false);
        access.insert("manageMembership".to_string(), true);

        let group = GroupRepresentation {
            name: Some("Restricted".to_string()),
            access: Some(access),
            ..Default::default()
        };

        let json = serde_json::to_string(&group).unwrap();
        assert!(json.contains("\"access\""));
        assert!(json.contains("\"view\":true"));
    }

    #[test]
    fn test_nested_subgroups() {
        let grandchild = GroupRepresentation {
            id: Some("grandchild-1".to_string()),
            name: Some("Junior Devs".to_string()),
            path: Some("/Engineering/Backend/Junior Devs".to_string()),
            sub_groups: Some(vec![]),
            ..Default::default()
        };

        let child = GroupRepresentation {
            id: Some("child-1".to_string()),
            name: Some("Backend".to_string()),
            path: Some("/Engineering/Backend".to_string()),
            sub_groups: Some(vec![grandchild]),
            ..Default::default()
        };

        let parent = GroupRepresentation {
            id: Some("parent-1".to_string()),
            name: Some("Engineering".to_string()),
            path: Some("/Engineering".to_string()),
            sub_groups: Some(vec![child]),
            ..Default::default()
        };

        // Serialize and deserialize to verify round-trip
        let json = serde_json::to_string(&parent).unwrap();
        let restored: GroupRepresentation = serde_json::from_str(&json).unwrap();

        assert_eq!(restored.name.as_deref(), Some("Engineering"));
        let subs = restored.sub_groups.unwrap();
        assert_eq!(subs.len(), 1);
        assert_eq!(subs[0].name.as_deref(), Some("Backend"));

        let nested_subs = subs[0].sub_groups.as_ref().unwrap();
        assert_eq!(nested_subs.len(), 1);
        assert_eq!(nested_subs[0].name.as_deref(), Some("Junior Devs"));
    }

    #[test]
    fn test_empty_group_serializes_to_empty_object() {
        let group = GroupRepresentation::default();
        let json = serde_json::to_string(&group).unwrap();
        assert_eq!(json, "{}");
    }

    #[test]
    fn test_deserialization_with_unknown_fields() {
        // Keycloak might return fields we don't model - ensure we don't fail
        let json = r#"{
            "id": "123",
            "name": "TestGroup",
            "someUnknownField": "value",
            "anotherUnknown": 42
        }"#;

        let group: GroupRepresentation = serde_json::from_str(json).unwrap();
        assert_eq!(group.id.as_deref(), Some("123"));
        assert_eq!(group.name.as_deref(), Some("TestGroup"));
    }
}
