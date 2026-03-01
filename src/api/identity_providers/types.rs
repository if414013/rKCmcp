//! Type definitions for Keycloak Identity Provider resources.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Keycloak Identity Provider representation.
///
/// Represents an external identity provider configuration in Keycloak.
/// Identity providers allow users to authenticate using external services
/// like Google, Facebook, SAML IdPs, or OIDC providers.
#[derive(Debug, Clone, Serialize, Deserialize, Default, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct IdentityProviderRepresentation {
    /// Internal ID of the identity provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_id: Option<String>,

    /// Alias used to identify the identity provider (unique within realm)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,

    /// Display name of the identity provider shown to users
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// Provider ID (e.g., "google", "facebook", "oidc", "saml")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_id: Option<String>,

    /// Whether the identity provider is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// Whether to trust email addresses from this provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_email: Option<bool>,

    /// Whether to store tokens from this provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_token: Option<bool>,

    /// Whether to add a link to an existing user (link only, no registration)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_only: Option<bool>,

    /// Alias of the flow to use after first broker login
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_broker_login_flow_alias: Option<String>,

    /// Alias of the flow to use for post broker login
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_broker_login_flow_alias: Option<String>,

    /// Provider-specific configuration (e.g., client_id, authorization_url)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<HashMap<String, String>>,

    /// Whether to update the user profile on login
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_profile_first_login_mode: Option<String>,

    /// Whether users can authenticate using this provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authenticate_by_default: Option<bool>,

    /// Whether to add the identity provider as a read-only option
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_read_token_role_on_create: Option<bool>,
}

/// Parameters for listing identity providers.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct IdpListParams {
    /// The realm name
    pub realm: String,
}

/// Parameters for getting a single identity provider.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct IdpGetParams {
    /// The realm name
    pub realm: String,

    /// The identity provider alias
    pub alias: String,
}

/// Parameters for creating an identity provider.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct IdpCreateParams {
    /// The realm name
    pub realm: String,

    /// The identity provider representation to create
    pub identity_provider: IdentityProviderRepresentation,
}

/// Parameters for updating an identity provider.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct IdpUpdateParams {
    /// The realm name
    pub realm: String,

    /// The identity provider alias
    pub alias: String,

    /// The updated identity provider representation
    pub identity_provider: IdentityProviderRepresentation,
}

/// Parameters for deleting an identity provider.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct IdpDeleteParams {
    /// The realm name
    pub realm: String,

    /// The identity provider alias
    pub alias: String,
}

/// Parameters for getting a provider type description.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct IdpProvidersGetParams {
    /// The realm name
    pub realm: String,

    /// The provider type ID (e.g., "google", "oidc", "saml")
    pub provider_id: String,
}

/// Identity provider type description.
///
/// Contains metadata about an available identity provider type.
#[derive(Debug, Clone, Serialize, Deserialize, Default, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct IdentityProviderTypeRepresentation {
    /// The provider type ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The display name of the provider type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_provider_representation_default() {
        let idp = IdentityProviderRepresentation::default();
        assert!(idp.alias.is_none());
        assert!(idp.provider_id.is_none());
        assert!(idp.enabled.is_none());
    }

    #[test]
    fn test_identity_provider_representation_serialization() {
        let idp = IdentityProviderRepresentation {
            alias: Some("google".to_string()),
            display_name: Some("Google".to_string()),
            provider_id: Some("google".to_string()),
            enabled: Some(true),
            trust_email: Some(true),
            ..Default::default()
        };

        let json = serde_json::to_string(&idp).expect("Failed to serialize");
        assert!(json.contains("\"alias\":\"google\""));
        assert!(json.contains("\"displayName\":\"Google\""));
        assert!(json.contains("\"providerId\":\"google\""));
        assert!(json.contains("\"enabled\":true"));
        assert!(json.contains("\"trustEmail\":true"));
        assert!(!json.contains("\"internalId\""));
    }

    #[test]
    fn test_identity_provider_representation_deserialization() {
        let json = r#"{
            "alias": "my-oidc-idp",
            "displayName": "My OIDC Provider",
            "providerId": "oidc",
            "enabled": true,
            "trustEmail": false,
            "storeToken": true,
            "linkOnly": false,
            "firstBrokerLoginFlowAlias": "first broker login",
            "config": {
                "clientId": "my-client-id",
                "authorizationUrl": "https://example.com/auth"
            }
        }"#;

        let idp: IdentityProviderRepresentation =
            serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(idp.alias, Some("my-oidc-idp".to_string()));
        assert_eq!(idp.display_name, Some("My OIDC Provider".to_string()));
        assert_eq!(idp.provider_id, Some("oidc".to_string()));
        assert_eq!(idp.enabled, Some(true));
        assert_eq!(idp.trust_email, Some(false));
        assert_eq!(idp.store_token, Some(true));
        assert_eq!(idp.link_only, Some(false));
        assert_eq!(
            idp.first_broker_login_flow_alias,
            Some("first broker login".to_string())
        );

        let config = idp.config.expect("Config should be present");
        assert_eq!(config.get("clientId"), Some(&"my-client-id".to_string()));
        assert_eq!(
            config.get("authorizationUrl"),
            Some(&"https://example.com/auth".to_string())
        );
    }

    #[test]
    fn test_identity_provider_representation_with_config() {
        let mut config = HashMap::new();
        config.insert("clientId".to_string(), "test-client".to_string());
        config.insert("clientSecret".to_string(), "secret-value".to_string());

        let idp = IdentityProviderRepresentation {
            alias: Some("test-idp".to_string()),
            provider_id: Some("oidc".to_string()),
            config: Some(config),
            ..Default::default()
        };

        let json = serde_json::to_string(&idp).expect("Failed to serialize");
        assert!(json.contains("\"config\""));
        assert!(json.contains("\"clientId\":\"test-client\""));
    }

    #[test]
    fn test_idp_list_params_deserialization() {
        let json = r#"{"realm": "master"}"#;

        let params: IdpListParams = serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(params.realm, "master");
    }

    #[test]
    fn test_idp_get_params_deserialization() {
        let json = r#"{
            "realm": "master",
            "alias": "google"
        }"#;

        let params: IdpGetParams = serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(params.realm, "master");
        assert_eq!(params.alias, "google");
    }

    #[test]
    fn test_idp_create_params_deserialization() {
        let json = r#"{
            "realm": "master",
            "identityProvider": {
                "alias": "new-idp",
                "providerId": "oidc",
                "enabled": true
            }
        }"#;

        let params: IdpCreateParams = serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(params.realm, "master");
        assert_eq!(params.identity_provider.alias, Some("new-idp".to_string()));
        assert_eq!(
            params.identity_provider.provider_id,
            Some("oidc".to_string())
        );
        assert_eq!(params.identity_provider.enabled, Some(true));
    }

    #[test]
    fn test_idp_update_params_deserialization() {
        let json = r#"{
            "realm": "master",
            "alias": "existing-idp",
            "identityProvider": {
                "alias": "existing-idp",
                "displayName": "Updated Display Name",
                "enabled": false
            }
        }"#;

        let params: IdpUpdateParams = serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(params.realm, "master");
        assert_eq!(params.alias, "existing-idp");
        assert_eq!(
            params.identity_provider.display_name,
            Some("Updated Display Name".to_string())
        );
        assert_eq!(params.identity_provider.enabled, Some(false));
    }

    #[test]
    fn test_idp_delete_params_deserialization() {
        let json = r#"{
            "realm": "master",
            "alias": "idp-to-delete"
        }"#;

        let params: IdpDeleteParams = serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(params.realm, "master");
        assert_eq!(params.alias, "idp-to-delete");
    }

    #[test]
    fn test_idp_providers_get_params_deserialization() {
        let json = r#"{
            "realm": "master",
            "providerId": "oidc"
        }"#;

        let params: IdpProvidersGetParams =
            serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(params.realm, "master");
        assert_eq!(params.provider_id, "oidc");
    }

    #[test]
    fn test_identity_provider_type_representation() {
        let json = r#"{
            "id": "oidc",
            "name": "OpenID Connect v1.0"
        }"#;

        let provider_type: IdentityProviderTypeRepresentation =
            serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(provider_type.id, Some("oidc".to_string()));
        assert_eq!(provider_type.name, Some("OpenID Connect v1.0".to_string()));
    }
}
