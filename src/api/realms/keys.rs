//! Keycloak Realm Keys API module.
//!
//! Provides access to realm key metadata (public keys only - private keys are never exposed).

use serde::{Deserialize, Serialize};

use crate::api::{ApiError, KeycloakClient};

// ==================== Type Definitions ====================

/// Representation of a realm's key metadata.
///
/// Contains information about all keys used by the realm for signing tokens,
/// encrypting data, etc. **Note**: Only public key information is exposed -
/// private keys are never included in API responses.
#[derive(Debug, Clone, Serialize, Deserialize, Default, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct KeysMetadataRepresentation {
    /// List of active keys for this realm
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys: Option<Vec<KeyMetadataRepresentation>>,

    /// Active key IDs by algorithm (e.g., {"RS256": "key-id-1", "HS256": "key-id-2"})
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<std::collections::HashMap<String, String>>,
}

/// Representation of a single key's metadata.
///
/// Contains public key information including certificates and algorithm details.
/// **Note**: The `private_key` field is intentionally NOT included as Keycloak
/// never exposes private keys through the Admin API.
#[derive(Debug, Clone, Serialize, Deserialize, Default, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct KeyMetadataRepresentation {
    /// Key ID (kid) used to identify this key in tokens
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kid: Option<String>,

    /// Provider ID that generated this key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_id: Option<String>,

    /// Provider priority (lower numbers = higher priority)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_priority: Option<i64>,

    /// Public key in PEM format (for asymmetric keys)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,

    /// X.509 certificate in PEM format (for asymmetric keys)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,

    /// Key type (e.g., "RSA", "EC", "OCT")
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub key_type: Option<String>,

    /// Algorithm used with this key (e.g., "RS256", "ES256", "HS256")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,

    /// Key use (e.g., "SIG" for signing, "ENC" for encryption)
    #[serde(rename = "use", skip_serializing_if = "Option::is_none")]
    pub key_use: Option<String>,

    /// Key status (e.g., "ACTIVE", "PASSIVE", "DISABLED")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// Validity start time (epoch milliseconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_to: Option<i64>,
}

// ==================== Parameter Definitions ====================

/// Parameters for listing realm keys.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RealmKeysListParams {
    /// The realm name
    pub realm: String,
}

impl RealmKeysListParams {
    /// Validate the parameters.
    pub fn validate(&self) -> Result<(), String> {
        if self.realm.trim().is_empty() {
            return Err("realm cannot be empty".to_string());
        }
        Ok(())
    }
}

// ==================== API Functions ====================

/// Get realm key metadata.
///
/// Returns metadata about all keys used by the realm, including public keys
/// and certificates. Private keys are never exposed through this endpoint.
///
/// GET /admin/realms/{realm}/keys
pub async fn realm_keys_list(
    client: &KeycloakClient,
    token: &str,
    params: &RealmKeysListParams,
) -> Result<KeysMetadataRepresentation, ApiError> {
    params.validate().map_err(ApiError::BadRequest)?;

    let path = format!("/admin/realms/{}/keys", urlencoding::encode(&params.realm));

    client.get(&path, token).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    const TEST_TOKEN: &str = "test-bearer-token";

    fn sample_keys_metadata() -> KeysMetadataRepresentation {
        let mut active = std::collections::HashMap::new();
        active.insert("RS256".to_string(), "key-id-rs256".to_string());
        active.insert("HS256".to_string(), "key-id-hs256".to_string());

        KeysMetadataRepresentation {
            keys: Some(vec![
                KeyMetadataRepresentation {
                    kid: Some("key-id-rs256".to_string()),
                    provider_id: Some("rsa-generated".to_string()),
                    provider_priority: Some(100),
                    public_key: Some("-----BEGIN PUBLIC KEY-----\nMIIB...".to_string()),
                    certificate: Some("-----BEGIN CERTIFICATE-----\nMIIC...".to_string()),
                    key_type: Some("RSA".to_string()),
                    algorithm: Some("RS256".to_string()),
                    key_use: Some("SIG".to_string()),
                    status: Some("ACTIVE".to_string()),
                    valid_to: None,
                },
                KeyMetadataRepresentation {
                    kid: Some("key-id-hs256".to_string()),
                    provider_id: Some("hmac-generated".to_string()),
                    provider_priority: Some(100),
                    public_key: None,
                    certificate: None,
                    key_type: Some("OCT".to_string()),
                    algorithm: Some("HS256".to_string()),
                    key_use: Some("SIG".to_string()),
                    status: Some("ACTIVE".to_string()),
                    valid_to: None,
                },
            ]),
            active: Some(active),
        }
    }

    // ==================== realm_keys_list tests ====================

    #[tokio::test]
    async fn test_realm_keys_list_success() {
        let mock_server = MockServer::start().await;

        let expected_keys = sample_keys_metadata();

        Mock::given(method("GET"))
            .and(path("/admin/realms/test-realm/keys"))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_keys))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmKeysListParams {
            realm: "test-realm".to_string(),
        };

        let result = realm_keys_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());

        let keys = result.expect("Failed to get keys");
        assert!(keys.keys.is_some());
        let key_list = keys.keys.unwrap();
        assert_eq!(key_list.len(), 2);
        assert_eq!(key_list[0].kid, Some("key-id-rs256".to_string()));
        assert_eq!(key_list[0].algorithm, Some("RS256".to_string()));
    }

    #[tokio::test]
    async fn test_realm_keys_list_with_special_characters() {
        let mock_server = MockServer::start().await;

        let expected_keys = KeysMetadataRepresentation::default();

        Mock::given(method("GET"))
            .and(path("/admin/realms/my%20realm%2Ftest/keys"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_keys))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmKeysListParams {
            realm: "my realm/test".to_string(),
        };

        let result = realm_keys_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_realm_keys_list_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/admin/realms/nonexistent/keys"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmKeysListParams {
            realm: "nonexistent".to_string(),
        };

        let result = realm_keys_list(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_realm_keys_list_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/admin/realms/test-realm/keys"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmKeysListParams {
            realm: "test-realm".to_string(),
        };

        let result = realm_keys_list(&client, "invalid-token", &params).await;
        assert!(matches!(result, Err(ApiError::Unauthorized)));
    }

    #[tokio::test]
    async fn test_realm_keys_list_forbidden() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/admin/realms/test-realm/keys"))
            .respond_with(ResponseTemplate::new(403))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmKeysListParams {
            realm: "test-realm".to_string(),
        };

        let result = realm_keys_list(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::Forbidden)));
    }

    #[tokio::test]
    async fn test_realm_keys_list_empty_realm_validation() {
        let mock_server = MockServer::start().await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmKeysListParams {
            realm: "".to_string(),
        };

        let result = realm_keys_list(&client, TEST_TOKEN, &params).await;
        match result {
            Err(ApiError::BadRequest(msg)) => {
                assert!(msg.contains("cannot be empty"));
            }
            _ => panic!("Expected BadRequest error"),
        }
    }

    // ==================== Type Serialization Tests ====================

    #[test]
    fn test_keys_metadata_serialization() {
        let keys = sample_keys_metadata();

        let json = serde_json::to_string(&keys).expect("Failed to serialize");
        assert!(json.contains("\"kid\":\"key-id-rs256\""));
        assert!(json.contains("\"algorithm\":\"RS256\""));
        assert!(json.contains("\"publicKey\""));
        // Verify no privateKey field is serialized
        assert!(!json.contains("privateKey"));
    }

    #[test]
    fn test_keys_metadata_deserialization() {
        let json = r#"{
            "keys": [
                {
                    "kid": "key-123",
                    "providerId": "rsa-generated",
                    "providerPriority": 100,
                    "publicKey": "-----BEGIN PUBLIC KEY-----\ntest\n-----END PUBLIC KEY-----",
                    "certificate": "-----BEGIN CERTIFICATE-----\ntest\n-----END CERTIFICATE-----",
                    "type": "RSA",
                    "algorithm": "RS256",
                    "use": "SIG",
                    "status": "ACTIVE"
                }
            ],
            "active": {
                "RS256": "key-123"
            }
        }"#;

        let keys: KeysMetadataRepresentation =
            serde_json::from_str(json).expect("Failed to deserialize");

        assert!(keys.keys.is_some());
        let key_list = keys.keys.unwrap();
        assert_eq!(key_list.len(), 1);
        assert_eq!(key_list[0].kid, Some("key-123".to_string()));
        assert_eq!(key_list[0].key_type, Some("RSA".to_string()));
        assert_eq!(key_list[0].key_use, Some("SIG".to_string()));

        assert!(keys.active.is_some());
        let active = keys.active.unwrap();
        assert_eq!(active.get("RS256"), Some(&"key-123".to_string()));
    }

    #[test]
    fn test_key_metadata_representation_default() {
        let key = KeyMetadataRepresentation::default();
        assert!(key.kid.is_none());
        assert!(key.public_key.is_none());
        assert!(key.algorithm.is_none());
    }

    #[test]
    fn test_realm_keys_list_params_validation() {
        let params = RealmKeysListParams {
            realm: "valid-realm".to_string(),
        };
        assert!(params.validate().is_ok());

        let params = RealmKeysListParams {
            realm: "".to_string(),
        };
        assert!(params.validate().is_err());

        let params = RealmKeysListParams {
            realm: "   ".to_string(),
        };
        assert!(params.validate().is_err());
    }
}
