//! Type definitions for Keycloak Realm resources.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Keycloak Realm representation.
///
/// Represents a realm in Keycloak. A realm manages a set of users, credentials,
/// roles, and groups. A user belongs to and logs into a realm. Realms are isolated
/// from one another and can only manage and authenticate the users that they control.
#[derive(Debug, Clone, Serialize, Deserialize, Default, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RealmRepresentation {
    // ==================== Core Identity ====================
    /// Unique identifier for the realm (UUID)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Realm name (used in URLs and as unique identifier)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub realm: Option<String>,

    /// Display name shown in UI
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// Display name with HTML (for custom branding)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name_html: Option<String>,

    /// Whether the realm is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    // ==================== SSL Configuration ====================
    /// SSL requirement (none, external, all)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_required: Option<String>,

    // ==================== Registration & Login ====================
    /// Allow user self-registration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_allowed: Option<bool>,

    /// Use email as username during registration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_email_as_username: Option<bool>,

    /// Require email verification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verify_email: Option<bool>,

    /// Allow login with email address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_with_email_allowed: Option<bool>,

    /// Allow duplicate email addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate_emails_allowed: Option<bool>,

    /// Allow password reset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_password_allowed: Option<bool>,

    /// Allow users to edit their username
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit_username_allowed: Option<bool>,

    /// Remember me enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remember_me: Option<bool>,

    // ==================== Brute Force Protection ====================
    /// Enable brute force attack protection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brute_force_protected: Option<bool>,

    /// Permanently lock account after max failures
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permanent_lockout: Option<bool>,

    /// Maximum wait time after failure in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_failure_wait_seconds: Option<i32>,

    /// Minimum quick login wait time in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_quick_login_wait_seconds: Option<i32>,

    /// Wait increment for each failure in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_increment_seconds: Option<i32>,

    /// Quick login check time in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quick_login_check_milli_seconds: Option<i64>,

    /// Max delta time for failure counting in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_delta_time_seconds: Option<i32>,

    /// Number of failures before lockout
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_factor: Option<i32>,

    // ==================== Roles & Credentials ====================
    /// Default roles assigned to new users
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_roles: Option<Vec<String>>,

    /// Default role for realm (references role object)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_role: Option<RoleRepresentation>,

    /// Required credential types
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_credentials: Option<Vec<String>>,

    // ==================== Password Policy ====================
    /// Password policy string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_policy: Option<String>,

    // ==================== OTP Policy ====================
    /// OTP policy type (totp, hotp)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub otp_policy_type: Option<String>,

    /// OTP algorithm (HmacSHA1, HmacSHA256, HmacSHA512)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub otp_policy_algorithm: Option<String>,

    /// Number of OTP digits
    #[serde(skip_serializing_if = "Option::is_none")]
    pub otp_policy_digits: Option<i32>,

    /// OTP initial counter (for HOTP)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub otp_policy_initial_counter: Option<i32>,

    /// OTP look ahead window
    #[serde(skip_serializing_if = "Option::is_none")]
    pub otp_policy_look_ahead_window: Option<i32>,

    /// OTP period in seconds (for TOTP)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub otp_policy_period: Option<i32>,

    /// Code re-usable for OTP
    #[serde(skip_serializing_if = "Option::is_none")]
    pub otp_policy_code_reusable: Option<bool>,

    /// OTP supported applications
    #[serde(skip_serializing_if = "Option::is_none")]
    pub otp_supported_applications: Option<Vec<String>>,

    // ==================== WebAuthn Policy ====================
    /// WebAuthn policy for passwordless
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_authn_policy_rp_entity_name: Option<String>,

    /// WebAuthn signature algorithms
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_authn_policy_signature_algorithms: Option<Vec<String>>,

    /// WebAuthn RP ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_authn_policy_rp_id: Option<String>,

    /// WebAuthn attestation preference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_authn_policy_attestation_conveyance_preference: Option<String>,

    /// WebAuthn authenticator attachment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_authn_policy_authenticator_attachment: Option<String>,

    /// WebAuthn require resident key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_authn_policy_require_resident_key: Option<String>,

    /// WebAuthn user verification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_authn_policy_user_verification_requirement: Option<String>,

    /// WebAuthn timeout in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_authn_policy_create_timeout: Option<i32>,

    /// Avoid same authenticator registration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_authn_policy_avoid_same_authenticator_register: Option<bool>,

    /// WebAuthn acceptable AAGUIDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_authn_policy_acceptable_aaguids: Option<Vec<String>>,

    // ==================== Token Lifespans ====================
    /// Access token lifespan in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token_lifespan: Option<i32>,

    /// Access token lifespan for implicit flow in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token_lifespan_for_implicit_flow: Option<i32>,

    /// SSO session idle timeout in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sso_session_idle_timeout: Option<i32>,

    /// SSO session max lifespan in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sso_session_max_lifespan: Option<i32>,

    /// SSO session idle timeout remember me in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sso_session_idle_timeout_remember_me: Option<i32>,

    /// SSO session max lifespan remember me in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sso_session_max_lifespan_remember_me: Option<i32>,

    /// Offline session idle timeout in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offline_session_idle_timeout: Option<i32>,

    /// Offline session max lifespan in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offline_session_max_lifespan: Option<i32>,

    /// Offline session max lifespan enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offline_session_max_lifespan_enabled: Option<bool>,

    /// Client session idle timeout in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_session_idle_timeout: Option<i32>,

    /// Client session max lifespan in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_session_max_lifespan: Option<i32>,

    /// Client offline session idle timeout in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_offline_session_idle_timeout: Option<i32>,

    /// Client offline session max lifespan in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_offline_session_max_lifespan: Option<i32>,

    /// Access code lifespan in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_code_lifespan: Option<i32>,

    /// Access code lifespan for user action in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_code_lifespan_user_action: Option<i32>,

    /// Access code lifespan for login in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_code_lifespan_login: Option<i32>,

    /// Action token lifespan generated by admin in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_token_generated_by_admin_lifespan: Option<i32>,

    /// Action token lifespan generated by user in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_token_generated_by_user_lifespan: Option<i32>,

    /// OAuth2 device code lifespan in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth2_device_code_lifespan: Option<i32>,

    /// OAuth2 device polling interval in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth2_device_polling_interval: Option<i32>,

    // ==================== SMTP Server ====================
    /// SMTP server configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smtp_server: Option<HashMap<String, String>>,

    // ==================== Themes ====================
    /// Login theme name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_theme: Option<String>,

    /// Account management theme name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_theme: Option<String>,

    /// Admin console theme name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_theme: Option<String>,

    /// Email theme name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_theme: Option<String>,

    // ==================== Internationalization ====================
    /// Enable internationalization
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internationalization_enabled: Option<bool>,

    /// Supported locales
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_locales: Option<Vec<String>>,

    /// Default locale
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_locale: Option<String>,

    // ==================== Browser Security Headers ====================
    /// Browser security headers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_security_headers: Option<HashMap<String, String>>,

    // ==================== Events ====================
    /// Enable events logging
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events_enabled: Option<bool>,

    /// Event expiration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events_expiration: Option<i64>,

    /// Event listeners
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events_listeners: Option<Vec<String>>,

    /// Enabled event types
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_event_types: Option<Vec<String>>,

    /// Admin events enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_events_enabled: Option<bool>,

    /// Include representation in admin events
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_events_details_enabled: Option<bool>,

    // ==================== User Profile ====================
    /// User managed access enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_managed_access_allowed: Option<bool>,

    // ==================== Generic Attributes ====================
    /// Custom realm attributes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<HashMap<String, String>>,

    // ==================== Not Before ====================
    /// Not before timestamp (for token revocation)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_before: Option<i32>,

    // ==================== Default Signature Algorithm ====================
    /// Default signature algorithm for realm
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_signature_algorithm: Option<String>,

    // ==================== Revoke Refresh Token ====================
    /// Revoke refresh token on use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoke_refresh_token: Option<bool>,

    /// Refresh token max reuse count
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token_max_reuse: Option<i32>,

    // ==================== Browser Flow ====================
    /// Browser authentication flow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_flow: Option<String>,

    /// Registration authentication flow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_flow: Option<String>,

    /// Direct grant authentication flow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_grant_flow: Option<String>,

    /// Reset credentials authentication flow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_credentials_flow: Option<String>,

    /// Client authentication flow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_authentication_flow: Option<String>,

    /// Docker authentication flow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docker_authentication_flow: Option<String>,

    /// First broker login flow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_broker_login_flow: Option<String>,
}

/// Minimal role representation for default role reference.
#[derive(Debug, Clone, Serialize, Deserialize, Default, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RoleRepresentation {
    /// Role ID (UUID)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Role name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Role description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether this is a composite role
    #[serde(skip_serializing_if = "Option::is_none")]
    pub composite: Option<bool>,

    /// Whether this is a client role
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_role: Option<bool>,

    /// Container ID (realm name or client UUID)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_id: Option<String>,
}

/// Parameters for listing realms.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RealmListParams {
    /// If true, only return brief realm representations
    #[serde(default)]
    pub brief_representation: Option<bool>,
}

impl RealmListParams {
    /// Validate the parameters.
    pub fn validate(&self) -> Result<(), String> {
        // No required fields to validate
        Ok(())
    }
}

/// Parameters for getting a single realm.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RealmGetParams {
    /// The realm name
    pub realm: String,
}

impl RealmGetParams {
    /// Validate the parameters.
    pub fn validate(&self) -> Result<(), String> {
        if self.realm.trim().is_empty() {
            return Err("realm cannot be empty".to_string());
        }
        Ok(())
    }
}

/// Parameters for creating a realm.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RealmCreateParams {
    /// The realm representation to create
    pub realm: RealmRepresentation,
}

impl RealmCreateParams {
    /// Validate the parameters.
    pub fn validate(&self) -> Result<(), String> {
        // Realm name is required for creation
        match &self.realm.realm {
            Some(name) if !name.trim().is_empty() => Ok(()),
            _ => Err("realm.realm (name) is required and cannot be empty".to_string()),
        }
    }
}

/// Parameters for updating a realm.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RealmUpdateParams {
    /// The realm name to update
    pub realm_name: String,

    /// The updated realm representation (partial update supported)
    pub realm: RealmRepresentation,
}

impl RealmUpdateParams {
    /// Validate the parameters.
    pub fn validate(&self) -> Result<(), String> {
        if self.realm_name.trim().is_empty() {
            return Err("realm_name cannot be empty".to_string());
        }
        Ok(())
    }
}

/// Parameters for deleting a realm.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RealmDeleteParams {
    /// The realm name to delete
    pub realm: String,
}

impl RealmDeleteParams {
    /// Validate the parameters.
    pub fn validate(&self) -> Result<(), String> {
        if self.realm.trim().is_empty() {
            return Err("realm cannot be empty".to_string());
        }

        // Prevent deletion of master realm
        if self.realm.eq_ignore_ascii_case("master") {
            return Err("Cannot delete the master realm".to_string());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_realm_representation_default() {
        let realm = RealmRepresentation::default();
        assert!(realm.id.is_none());
        assert!(realm.realm.is_none());
        assert!(realm.enabled.is_none());
        assert!(realm.display_name.is_none());
    }

    #[test]
    fn test_realm_representation_serialization() {
        let realm = RealmRepresentation {
            realm: Some("my-realm".to_string()),
            enabled: Some(true),
            display_name: Some("My Realm".to_string()),
            ssl_required: Some("external".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&realm).expect("Failed to serialize");
        assert!(json.contains("\"realm\":\"my-realm\""));
        assert!(json.contains("\"enabled\":true"));
        assert!(json.contains("\"displayName\":\"My Realm\""));
        assert!(json.contains("\"sslRequired\":\"external\""));
        // Verify None fields are not serialized
        assert!(!json.contains("\"id\""));
        assert!(!json.contains("\"displayNameHtml\""));
    }

    #[test]
    fn test_realm_representation_deserialization() {
        let json = r#"{
            "id": "realm-123",
            "realm": "test-realm",
            "displayName": "Test Realm",
            "enabled": true,
            "sslRequired": "all",
            "registrationAllowed": false,
            "verifyEmail": true,
            "loginWithEmailAllowed": true,
            "bruteForceProtected": true,
            "failureFactor": 30,
            "accessTokenLifespan": 300,
            "ssoSessionIdleTimeout": 1800
        }"#;

        let realm: RealmRepresentation = serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(realm.id, Some("realm-123".to_string()));
        assert_eq!(realm.realm, Some("test-realm".to_string()));
        assert_eq!(realm.display_name, Some("Test Realm".to_string()));
        assert_eq!(realm.enabled, Some(true));
        assert_eq!(realm.ssl_required, Some("all".to_string()));
        assert_eq!(realm.registration_allowed, Some(false));
        assert_eq!(realm.verify_email, Some(true));
        assert_eq!(realm.login_with_email_allowed, Some(true));
        assert_eq!(realm.brute_force_protected, Some(true));
        assert_eq!(realm.failure_factor, Some(30));
        assert_eq!(realm.access_token_lifespan, Some(300));
        assert_eq!(realm.sso_session_idle_timeout, Some(1800));
    }

    #[test]
    fn test_realm_representation_with_smtp() {
        let mut smtp_config = HashMap::new();
        smtp_config.insert("host".to_string(), "smtp.example.com".to_string());
        smtp_config.insert("port".to_string(), "587".to_string());
        smtp_config.insert("from".to_string(), "noreply@example.com".to_string());

        let realm = RealmRepresentation {
            realm: Some("with-smtp".to_string()),
            smtp_server: Some(smtp_config),
            ..Default::default()
        };

        let json = serde_json::to_string(&realm).expect("Failed to serialize");
        assert!(json.contains("\"smtpServer\""));
        assert!(json.contains("smtp.example.com"));
    }

    #[test]
    fn test_realm_representation_with_themes() {
        let realm = RealmRepresentation {
            realm: Some("themed".to_string()),
            login_theme: Some("keycloak".to_string()),
            account_theme: Some("keycloak".to_string()),
            admin_theme: Some("keycloak".to_string()),
            email_theme: Some("keycloak".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&realm).expect("Failed to serialize");
        assert!(json.contains("\"loginTheme\":\"keycloak\""));
        assert!(json.contains("\"accountTheme\":\"keycloak\""));
        assert!(json.contains("\"adminTheme\":\"keycloak\""));
        assert!(json.contains("\"emailTheme\":\"keycloak\""));
    }

    #[test]
    fn test_realm_representation_with_locales() {
        let realm = RealmRepresentation {
            realm: Some("i18n".to_string()),
            internationalization_enabled: Some(true),
            supported_locales: Some(vec!["en".to_string(), "es".to_string(), "fr".to_string()]),
            default_locale: Some("en".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&realm).expect("Failed to serialize");
        assert!(json.contains("\"internationalizationEnabled\":true"));
        assert!(json.contains("\"supportedLocales\""));
        assert!(json.contains("\"defaultLocale\":\"en\""));
    }

    #[test]
    fn test_realm_representation_with_security_headers() {
        let mut headers = HashMap::new();
        headers.insert(
            "contentSecurityPolicy".to_string(),
            "frame-src 'self'".to_string(),
        );
        headers.insert("xFrameOptions".to_string(), "SAMEORIGIN".to_string());

        let realm = RealmRepresentation {
            realm: Some("secure".to_string()),
            browser_security_headers: Some(headers),
            ..Default::default()
        };

        let json = serde_json::to_string(&realm).expect("Failed to serialize");
        assert!(json.contains("\"browserSecurityHeaders\""));
    }

    // ==================== Parameter Validation Tests ====================

    #[test]
    fn test_realm_list_params_validate() {
        let params = RealmListParams {
            brief_representation: Some(true),
        };
        assert!(params.validate().is_ok());

        let params = RealmListParams {
            brief_representation: None,
        };
        assert!(params.validate().is_ok());
    }

    #[test]
    fn test_realm_get_params_validate_success() {
        let params = RealmGetParams {
            realm: "my-realm".to_string(),
        };
        assert!(params.validate().is_ok());
    }

    #[test]
    fn test_realm_get_params_validate_empty() {
        let params = RealmGetParams {
            realm: "".to_string(),
        };
        let result = params.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cannot be empty"));
    }

    #[test]
    fn test_realm_get_params_validate_whitespace() {
        let params = RealmGetParams {
            realm: "   ".to_string(),
        };
        let result = params.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_realm_create_params_validate_success() {
        let params = RealmCreateParams {
            realm: RealmRepresentation {
                realm: Some("new-realm".to_string()),
                ..Default::default()
            },
        };
        assert!(params.validate().is_ok());
    }

    #[test]
    fn test_realm_create_params_validate_missing_name() {
        let params = RealmCreateParams {
            realm: RealmRepresentation::default(),
        };
        let result = params.validate();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("realm.realm (name) is required"));
    }

    #[test]
    fn test_realm_create_params_validate_empty_name() {
        let params = RealmCreateParams {
            realm: RealmRepresentation {
                realm: Some("".to_string()),
                ..Default::default()
            },
        };
        let result = params.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_realm_update_params_validate_success() {
        let params = RealmUpdateParams {
            realm_name: "existing-realm".to_string(),
            realm: RealmRepresentation {
                display_name: Some("Updated Name".to_string()),
                ..Default::default()
            },
        };
        assert!(params.validate().is_ok());
    }

    #[test]
    fn test_realm_update_params_validate_empty_name() {
        let params = RealmUpdateParams {
            realm_name: "".to_string(),
            realm: RealmRepresentation::default(),
        };
        let result = params.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("realm_name cannot be empty"));
    }

    #[test]
    fn test_realm_delete_params_validate_success() {
        let params = RealmDeleteParams {
            realm: "test-realm".to_string(),
        };
        assert!(params.validate().is_ok());
    }

    #[test]
    fn test_realm_delete_params_validate_empty() {
        let params = RealmDeleteParams {
            realm: "".to_string(),
        };
        let result = params.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cannot be empty"));
    }

    #[test]
    fn test_realm_delete_params_validate_master_realm() {
        let params = RealmDeleteParams {
            realm: "master".to_string(),
        };
        let result = params.validate();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("Cannot delete the master realm"));
    }

    #[test]
    fn test_realm_delete_params_validate_master_case_insensitive() {
        let params = RealmDeleteParams {
            realm: "MASTER".to_string(),
        };
        let result = params.validate();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("Cannot delete the master realm"));

        let params = RealmDeleteParams {
            realm: "Master".to_string(),
        };
        let result = params.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_realm_list_params_deserialization() {
        let json = r#"{"briefRepresentation": true}"#;
        let params: RealmListParams = serde_json::from_str(json).expect("Failed to deserialize");
        assert_eq!(params.brief_representation, Some(true));
    }

    #[test]
    fn test_realm_get_params_deserialization() {
        let json = r#"{"realm": "my-test-realm"}"#;
        let params: RealmGetParams = serde_json::from_str(json).expect("Failed to deserialize");
        assert_eq!(params.realm, "my-test-realm");
    }

    #[test]
    fn test_realm_create_params_deserialization() {
        let json = r#"{
            "realm": {
                "realm": "new-realm",
                "enabled": true,
                "displayName": "New Realm"
            }
        }"#;
        let params: RealmCreateParams = serde_json::from_str(json).expect("Failed to deserialize");
        assert_eq!(params.realm.realm, Some("new-realm".to_string()));
        assert_eq!(params.realm.enabled, Some(true));
        assert_eq!(params.realm.display_name, Some("New Realm".to_string()));
    }

    #[test]
    fn test_realm_update_params_deserialization() {
        let json = r#"{
            "realmName": "existing-realm",
            "realm": {
                "displayName": "Updated Display Name",
                "enabled": false
            }
        }"#;
        let params: RealmUpdateParams = serde_json::from_str(json).expect("Failed to deserialize");
        assert_eq!(params.realm_name, "existing-realm");
        assert_eq!(
            params.realm.display_name,
            Some("Updated Display Name".to_string())
        );
        assert_eq!(params.realm.enabled, Some(false));
    }

    #[test]
    fn test_realm_delete_params_deserialization() {
        let json = r#"{"realm": "to-delete"}"#;
        let params: RealmDeleteParams = serde_json::from_str(json).expect("Failed to deserialize");
        assert_eq!(params.realm, "to-delete");
    }

    #[test]
    fn test_role_representation_serialization() {
        let role = RoleRepresentation {
            id: Some("role-123".to_string()),
            name: Some("admin".to_string()),
            composite: Some(false),
            ..Default::default()
        };

        let json = serde_json::to_string(&role).expect("Failed to serialize");
        assert!(json.contains("\"id\":\"role-123\""));
        assert!(json.contains("\"name\":\"admin\""));
    }
}
