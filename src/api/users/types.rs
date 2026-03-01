//! Type definitions for Keycloak User API.
//!
//! This module contains the data structures for representing users and their
//! related resources in the Keycloak Admin REST API.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a credential in Keycloak.
///
/// Credentials are used for user authentication (passwords, OTP, etc.).
#[derive(Debug, Clone, Serialize, Deserialize, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CredentialRepresentation {
    /// Unique identifier for the credential
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Type of credential (e.g., "password", "otp", "secret")
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub credential_type: Option<String>,

    /// User-friendly label for the credential
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_label: Option<String>,

    /// Timestamp when the credential was created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<i64>,

    /// The secret data (e.g., hashed password)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_data: Option<String>,

    /// Credential metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_data: Option<String>,

    /// Priority of the credential
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,

    /// The credential value (used when setting a new password)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    /// Whether the credential is temporary (must be changed on next login)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temporary: Option<bool>,
}

/// Represents a user in Keycloak.
///
/// This struct contains all the fields that can be present in a user representation
/// when interacting with the Keycloak Admin REST API.
#[derive(Debug, Clone, Serialize, Deserialize, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserRepresentation {
    /// Unique identifier for the user (UUID)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Username (unique within a realm)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    /// Email address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// First name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,

    /// Last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    /// Whether the user account is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// Whether the email has been verified
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_verified: Option<bool>,

    /// Custom user attributes (key-value pairs where values are arrays)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<HashMap<String, Vec<String>>>,

    /// User credentials (passwords, OTP, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Vec<CredentialRepresentation>>,

    /// Required actions the user must perform on next login
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_actions: Option<Vec<String>>,

    /// Groups the user belongs to (group paths)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,

    /// Realm roles assigned to the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub realm_roles: Option<Vec<String>>,

    /// Client roles assigned to the user (client ID -> role names)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_roles: Option<HashMap<String, Vec<String>>>,

    /// Timestamp when the user was created (milliseconds since epoch)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<i64>,

    /// Federated identities linked to this user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub federated_identities: Option<Vec<FederatedIdentityRepresentation>>,

    /// Service account client ID (if this is a service account)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_account_client_id: Option<String>,

    /// Access control settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access: Option<HashMap<String, bool>>,

    /// Count of disabled credentials
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_credential_types: Option<Vec<String>>,

    /// Origin realm (for cross-realm links)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,

    /// Self-link URL
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,

    /// TOTP enabled flag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub totp: Option<bool>,

    /// Federation link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub federation_link: Option<String>,

    /// Not before timestamp (for token invalidation)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_before: Option<i32>,
}

/// Represents a federated identity linked to a user.
#[derive(Debug, Clone, Serialize, Deserialize, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct FederatedIdentityRepresentation {
    /// Identity provider alias
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_provider: Option<String>,

    /// User ID in the identity provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,

    /// Username in the identity provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_representation_default() {
        let user = UserRepresentation::default();
        assert!(user.id.is_none());
        assert!(user.username.is_none());
        assert!(user.email.is_none());
    }

    #[test]
    fn test_user_representation_serialization() {
        let user = UserRepresentation {
            username: Some("testuser".to_string()),
            email: Some("test@example.com".to_string()),
            enabled: Some(true),
            ..Default::default()
        };

        let json = serde_json::to_string(&user).unwrap();
        assert!(json.contains("\"username\":\"testuser\""));
        assert!(json.contains("\"email\":\"test@example.com\""));
        assert!(json.contains("\"enabled\":true"));
        // Verify None fields are not serialized
        assert!(!json.contains("\"id\""));
    }

    #[test]
    fn test_user_representation_deserialization() {
        let json = r#"{
            "id": "123e4567-e89b-12d3-a456-426614174000",
            "username": "testuser",
            "email": "test@example.com",
            "firstName": "Test",
            "lastName": "User",
            "enabled": true,
            "emailVerified": false,
            "createdTimestamp": 1609459200000
        }"#;

        let user: UserRepresentation = serde_json::from_str(json).unwrap();
        assert_eq!(
            user.id.as_deref(),
            Some("123e4567-e89b-12d3-a456-426614174000")
        );
        assert_eq!(user.username.as_deref(), Some("testuser"));
        assert_eq!(user.email.as_deref(), Some("test@example.com"));
        assert_eq!(user.first_name.as_deref(), Some("Test"));
        assert_eq!(user.last_name.as_deref(), Some("User"));
        assert_eq!(user.enabled, Some(true));
        assert_eq!(user.email_verified, Some(false));
        assert_eq!(user.created_timestamp, Some(1609459200000));
    }

    #[test]
    fn test_user_with_attributes() {
        let mut attrs = HashMap::new();
        attrs.insert("department".to_string(), vec!["engineering".to_string()]);
        attrs.insert(
            "roles".to_string(),
            vec!["admin".to_string(), "user".to_string()],
        );

        let user = UserRepresentation {
            username: Some("attruser".to_string()),
            attributes: Some(attrs),
            ..Default::default()
        };

        let json = serde_json::to_string(&user).unwrap();
        assert!(json.contains("\"attributes\""));
        assert!(json.contains("\"department\""));
        assert!(json.contains("\"engineering\""));
    }

    #[test]
    fn test_user_with_credentials() {
        let cred = CredentialRepresentation {
            credential_type: Some("password".to_string()),
            value: Some("secret123".to_string()),
            temporary: Some(false),
            ..Default::default()
        };

        let user = UserRepresentation {
            username: Some("creduser".to_string()),
            credentials: Some(vec![cred]),
            ..Default::default()
        };

        let json = serde_json::to_string(&user).unwrap();
        assert!(json.contains("\"credentials\""));
        assert!(json.contains("\"type\":\"password\""));
        assert!(json.contains("\"temporary\":false"));
    }

    #[test]
    fn test_credential_representation_serialization() {
        let cred = CredentialRepresentation {
            credential_type: Some("password".to_string()),
            value: Some("newpassword".to_string()),
            temporary: Some(true),
            ..Default::default()
        };

        let json = serde_json::to_string(&cred).unwrap();
        assert!(json.contains("\"type\":\"password\""));
        assert!(json.contains("\"value\":\"newpassword\""));
        assert!(json.contains("\"temporary\":true"));
    }

    #[test]
    fn test_federated_identity_serialization() {
        let identity = FederatedIdentityRepresentation {
            identity_provider: Some("google".to_string()),
            user_id: Some("google-123".to_string()),
            user_name: Some("googleuser".to_string()),
        };

        let json = serde_json::to_string(&identity).unwrap();
        assert!(json.contains("\"identityProvider\":\"google\""));
        assert!(json.contains("\"userId\":\"google-123\""));
    }

    #[test]
    fn test_user_with_client_roles() {
        let mut client_roles = HashMap::new();
        client_roles.insert(
            "my-client".to_string(),
            vec!["view".to_string(), "edit".to_string()],
        );

        let user = UserRepresentation {
            username: Some("roleuser".to_string()),
            client_roles: Some(client_roles),
            ..Default::default()
        };

        let json = serde_json::to_string(&user).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert!(parsed["clientRoles"]["my-client"].is_array());
    }

    #[test]
    fn test_empty_user_serializes_to_empty_object() {
        let user = UserRepresentation::default();
        let json = serde_json::to_string(&user).unwrap();
        assert_eq!(json, "{}");
    }

    #[test]
    fn test_deserialization_with_unknown_fields() {
        // Keycloak might return fields we don't model - ensure we don't fail
        let json = r#"{
            "id": "123",
            "username": "testuser",
            "someUnknownField": "value",
            "anotherUnknown": 42
        }"#;

        let user: UserRepresentation = serde_json::from_str(json).unwrap();
        assert_eq!(user.id.as_deref(), Some("123"));
        assert_eq!(user.username.as_deref(), Some("testuser"));
    }
}
