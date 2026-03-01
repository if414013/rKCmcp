//! Keycloak Realm Events API module.
//!
//! Provides access to realm events (login, logout, etc.) and admin events.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::api::{ApiError, KeycloakClient};

// ==================== Type Definitions ====================

/// Representation of an event in Keycloak.
///
/// Events track user activities like login, logout, token refresh, etc.
#[derive(Debug, Clone, Serialize, Deserialize, Default, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct EventRepresentation {
    /// Event timestamp (epoch milliseconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,

    /// Event type (e.g., "LOGIN", "LOGOUT", "TOKEN_EXCHANGE")
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,

    /// Realm ID where the event occurred
    #[serde(skip_serializing_if = "Option::is_none")]
    pub realm_id: Option<String>,

    /// Client ID that triggered the event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,

    /// User ID associated with the event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,

    /// Session ID when the event occurred
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,

    /// IP address of the client
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,

    /// Error message if the event represents a failure
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,

    /// Additional event details as key-value pairs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<HashMap<String, String>>,
}

/// Representation of an admin event in Keycloak.
///
/// Admin events track administrative operations like user creation, realm changes, etc.
#[derive(Debug, Clone, Serialize, Deserialize, Default, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AdminEventRepresentation {
    /// Event timestamp (epoch milliseconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,

    /// Realm ID where the event occurred
    #[serde(skip_serializing_if = "Option::is_none")]
    pub realm_id: Option<String>,

    /// Authentication realm (may differ from realm_id for cross-realm operations)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_realm_id: Option<String>,

    /// Client ID that performed the admin operation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_client_id: Option<String>,

    /// User ID of the admin who performed the operation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_user_id: Option<String>,

    /// IP address of the admin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_ip_address: Option<String>,

    /// Operation type (e.g., "CREATE", "UPDATE", "DELETE", "ACTION")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_type: Option<String>,

    /// Resource type (e.g., "USER", "CLIENT", "REALM_ROLE")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,

    /// Resource path (e.g., "users/user-id-123")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_path: Option<String>,

    /// JSON representation of the resource (if details enabled)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub representation: Option<String>,

    /// Error message if the operation failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Configuration for realm event logging.
#[derive(Debug, Clone, Serialize, Deserialize, Default, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RealmEventsConfigRepresentation {
    /// Whether user events are enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events_enabled: Option<bool>,

    /// Event expiration time in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events_expiration: Option<i64>,

    /// List of event listener provider IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events_listeners: Option<Vec<String>>,

    /// List of enabled event types to log
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_event_types: Option<Vec<String>>,

    /// Whether admin events are enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_events_enabled: Option<bool>,

    /// Whether to include resource representations in admin events
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_events_details_enabled: Option<bool>,
}

// ==================== Parameter Definitions ====================

/// Parameters for listing realm events.
///
/// Pagination is required - `first` and `max` control the result window.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RealmEventsListParams {
    /// The realm name
    pub realm: String,

    /// Filter by event type(s) (e.g., "LOGIN", "LOGOUT")
    #[serde(default)]
    pub event_types: Option<Vec<String>>,

    /// Filter by client ID
    #[serde(default)]
    pub client: Option<String>,

    /// Filter by user ID
    #[serde(default)]
    pub user: Option<String>,

    /// Filter events from this date (ISO 8601 format: yyyy-MM-dd)
    #[serde(default)]
    pub date_from: Option<String>,

    /// Filter events to this date (ISO 8601 format: yyyy-MM-dd)
    #[serde(default)]
    pub date_to: Option<String>,

    /// Filter by IP address
    #[serde(default)]
    pub ip_address: Option<String>,

    /// Pagination offset (0-based)
    #[serde(default)]
    pub first: Option<i32>,

    /// Maximum number of results (required for pagination)
    #[serde(default)]
    pub max: Option<i32>,
}

impl RealmEventsListParams {
    pub fn validate(&self) -> Result<(), String> {
        if self.realm.trim().is_empty() {
            return Err("realm cannot be empty".to_string());
        }
        Ok(())
    }
}

/// Parameters for deleting realm events.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RealmEventsDeleteParams {
    /// The realm name
    pub realm: String,
}

impl RealmEventsDeleteParams {
    pub fn validate(&self) -> Result<(), String> {
        if self.realm.trim().is_empty() {
            return Err("realm cannot be empty".to_string());
        }
        Ok(())
    }
}

/// Parameters for getting realm events configuration.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RealmEventsConfigGetParams {
    /// The realm name
    pub realm: String,
}

impl RealmEventsConfigGetParams {
    pub fn validate(&self) -> Result<(), String> {
        if self.realm.trim().is_empty() {
            return Err("realm cannot be empty".to_string());
        }
        Ok(())
    }
}

/// Parameters for updating realm events configuration.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RealmEventsConfigUpdateParams {
    /// The realm name
    pub realm: String,

    /// The events configuration to apply
    pub config: RealmEventsConfigRepresentation,
}

impl RealmEventsConfigUpdateParams {
    pub fn validate(&self) -> Result<(), String> {
        if self.realm.trim().is_empty() {
            return Err("realm cannot be empty".to_string());
        }
        Ok(())
    }
}

/// Parameters for listing admin events.
///
/// Pagination is required - `first` and `max` control the result window.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RealmAdminEventsListParams {
    /// The realm name
    pub realm: String,

    /// Filter by operation type(s) (e.g., "CREATE", "UPDATE", "DELETE")
    #[serde(default)]
    pub operation_types: Option<Vec<String>>,

    /// Filter by resource type(s) (e.g., "USER", "CLIENT")
    #[serde(default)]
    pub resource_types: Option<Vec<String>>,

    /// Filter by resource path pattern
    #[serde(default)]
    pub resource_path: Option<String>,

    /// Filter by authentication realm
    #[serde(default)]
    pub auth_realm: Option<String>,

    /// Filter by authentication client
    #[serde(default)]
    pub auth_client: Option<String>,

    /// Filter by authentication user
    #[serde(default)]
    pub auth_user: Option<String>,

    /// Filter by authentication IP address
    #[serde(default)]
    pub auth_ip_address: Option<String>,

    /// Filter events from this date (ISO 8601 format: yyyy-MM-dd)
    #[serde(default)]
    pub date_from: Option<String>,

    /// Filter events to this date (ISO 8601 format: yyyy-MM-dd)
    #[serde(default)]
    pub date_to: Option<String>,

    /// Pagination offset (0-based)
    #[serde(default)]
    pub first: Option<i32>,

    /// Maximum number of results (required for pagination)
    #[serde(default)]
    pub max: Option<i32>,
}

impl RealmAdminEventsListParams {
    pub fn validate(&self) -> Result<(), String> {
        if self.realm.trim().is_empty() {
            return Err("realm cannot be empty".to_string());
        }
        Ok(())
    }
}

/// Parameters for deleting admin events.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RealmAdminEventsDeleteParams {
    /// The realm name
    pub realm: String,
}

impl RealmAdminEventsDeleteParams {
    pub fn validate(&self) -> Result<(), String> {
        if self.realm.trim().is_empty() {
            return Err("realm cannot be empty".to_string());
        }
        Ok(())
    }
}

// ==================== API Functions ====================

/// List realm events with optional filtering and pagination.
///
/// GET /admin/realms/{realm}/events
pub async fn realm_events_list(
    client: &KeycloakClient,
    token: &str,
    params: &RealmEventsListParams,
) -> Result<Vec<EventRepresentation>, ApiError> {
    params.validate().map_err(ApiError::BadRequest)?;

    let mut path = format!(
        "/admin/realms/{}/events",
        urlencoding::encode(&params.realm)
    );

    let mut query_parts: Vec<String> = Vec::new();

    if let Some(ref types) = params.event_types {
        for t in types {
            query_parts.push(format!("type={}", urlencoding::encode(t)));
        }
    }
    if let Some(ref client_id) = params.client {
        query_parts.push(format!("client={}", urlencoding::encode(client_id)));
    }
    if let Some(ref user) = params.user {
        query_parts.push(format!("user={}", urlencoding::encode(user)));
    }
    if let Some(ref date_from) = params.date_from {
        query_parts.push(format!("dateFrom={}", urlencoding::encode(date_from)));
    }
    if let Some(ref date_to) = params.date_to {
        query_parts.push(format!("dateTo={}", urlencoding::encode(date_to)));
    }
    if let Some(ref ip) = params.ip_address {
        query_parts.push(format!("ipAddress={}", urlencoding::encode(ip)));
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

/// Delete all realm events.
///
/// DELETE /admin/realms/{realm}/events
pub async fn realm_events_delete(
    client: &KeycloakClient,
    token: &str,
    params: &RealmEventsDeleteParams,
) -> Result<(), ApiError> {
    params.validate().map_err(ApiError::BadRequest)?;

    let path = format!(
        "/admin/realms/{}/events",
        urlencoding::encode(&params.realm)
    );

    client.delete(&path, token).await
}

/// Get realm events configuration.
///
/// GET /admin/realms/{realm}/events/config
pub async fn realm_events_config_get(
    client: &KeycloakClient,
    token: &str,
    params: &RealmEventsConfigGetParams,
) -> Result<RealmEventsConfigRepresentation, ApiError> {
    params.validate().map_err(ApiError::BadRequest)?;

    let path = format!(
        "/admin/realms/{}/events/config",
        urlencoding::encode(&params.realm)
    );

    client.get(&path, token).await
}

/// Update realm events configuration.
///
/// PUT /admin/realms/{realm}/events/config
pub async fn realm_events_config_update(
    client: &KeycloakClient,
    token: &str,
    params: &RealmEventsConfigUpdateParams,
) -> Result<(), ApiError> {
    params.validate().map_err(ApiError::BadRequest)?;

    let path = format!(
        "/admin/realms/{}/events/config",
        urlencoding::encode(&params.realm)
    );

    client.put(&path, token, &params.config).await
}

/// List admin events with optional filtering and pagination.
///
/// GET /admin/realms/{realm}/admin-events
pub async fn realm_admin_events_list(
    client: &KeycloakClient,
    token: &str,
    params: &RealmAdminEventsListParams,
) -> Result<Vec<AdminEventRepresentation>, ApiError> {
    params.validate().map_err(ApiError::BadRequest)?;

    let mut path = format!(
        "/admin/realms/{}/admin-events",
        urlencoding::encode(&params.realm)
    );

    let mut query_parts: Vec<String> = Vec::new();

    if let Some(ref op_types) = params.operation_types {
        for t in op_types {
            query_parts.push(format!("operationTypes={}", urlencoding::encode(t)));
        }
    }
    if let Some(ref res_types) = params.resource_types {
        for t in res_types {
            query_parts.push(format!("resourceTypes={}", urlencoding::encode(t)));
        }
    }
    if let Some(ref path_filter) = params.resource_path {
        query_parts.push(format!("resourcePath={}", urlencoding::encode(path_filter)));
    }
    if let Some(ref auth_realm) = params.auth_realm {
        query_parts.push(format!("authRealm={}", urlencoding::encode(auth_realm)));
    }
    if let Some(ref auth_client) = params.auth_client {
        query_parts.push(format!("authClient={}", urlencoding::encode(auth_client)));
    }
    if let Some(ref auth_user) = params.auth_user {
        query_parts.push(format!("authUser={}", urlencoding::encode(auth_user)));
    }
    if let Some(ref auth_ip) = params.auth_ip_address {
        query_parts.push(format!("authIpAddress={}", urlencoding::encode(auth_ip)));
    }
    if let Some(ref date_from) = params.date_from {
        query_parts.push(format!("dateFrom={}", urlencoding::encode(date_from)));
    }
    if let Some(ref date_to) = params.date_to {
        query_parts.push(format!("dateTo={}", urlencoding::encode(date_to)));
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

/// Delete all admin events.
///
/// DELETE /admin/realms/{realm}/admin-events
pub async fn realm_admin_events_delete(
    client: &KeycloakClient,
    token: &str,
    params: &RealmAdminEventsDeleteParams,
) -> Result<(), ApiError> {
    params.validate().map_err(ApiError::BadRequest)?;

    let path = format!(
        "/admin/realms/{}/admin-events",
        urlencoding::encode(&params.realm)
    );

    client.delete(&path, token).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{body_json, header, method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    const TEST_TOKEN: &str = "test-bearer-token";

    fn sample_event() -> EventRepresentation {
        let mut details = HashMap::new();
        details.insert("auth_method".to_string(), "openid-connect".to_string());

        EventRepresentation {
            time: Some(1700000000000),
            event_type: Some("LOGIN".to_string()),
            realm_id: Some("test-realm".to_string()),
            client_id: Some("my-client".to_string()),
            user_id: Some("user-123".to_string()),
            session_id: Some("session-456".to_string()),
            ip_address: Some("192.168.1.1".to_string()),
            error: None,
            details: Some(details),
        }
    }

    fn sample_admin_event() -> AdminEventRepresentation {
        AdminEventRepresentation {
            time: Some(1700000000000),
            realm_id: Some("test-realm".to_string()),
            auth_realm_id: Some("master".to_string()),
            auth_client_id: Some("admin-cli".to_string()),
            auth_user_id: Some("admin-user".to_string()),
            auth_ip_address: Some("10.0.0.1".to_string()),
            operation_type: Some("CREATE".to_string()),
            resource_type: Some("USER".to_string()),
            resource_path: Some("users/new-user-id".to_string()),
            representation: Some(r#"{"username":"newuser"}"#.to_string()),
            error: None,
        }
    }

    fn sample_events_config() -> RealmEventsConfigRepresentation {
        RealmEventsConfigRepresentation {
            events_enabled: Some(true),
            events_expiration: Some(86400),
            events_listeners: Some(vec!["jboss-logging".to_string()]),
            enabled_event_types: Some(vec![
                "LOGIN".to_string(),
                "LOGOUT".to_string(),
                "LOGIN_ERROR".to_string(),
            ]),
            admin_events_enabled: Some(true),
            admin_events_details_enabled: Some(true),
        }
    }

    // ==================== realm_events_list tests ====================

    #[tokio::test]
    async fn test_realm_events_list_success() {
        let mock_server = MockServer::start().await;

        let expected_events = vec![sample_event()];

        Mock::given(method("GET"))
            .and(path("/admin/realms/test-realm/events"))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_events))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmEventsListParams {
            realm: "test-realm".to_string(),
            event_types: None,
            client: None,
            user: None,
            date_from: None,
            date_to: None,
            ip_address: None,
            first: None,
            max: None,
        };

        let result = realm_events_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());

        let events = result.expect("Failed to get events");
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event_type, Some("LOGIN".to_string()));
    }

    #[tokio::test]
    async fn test_realm_events_list_with_filters() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/admin/realms/test-realm/events"))
            .and(query_param("type", "LOGIN"))
            .and(query_param("client", "my-client"))
            .and(query_param("user", "user-123"))
            .and(query_param("first", "0"))
            .and(query_param("max", "10"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(Vec::<EventRepresentation>::new()),
            )
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmEventsListParams {
            realm: "test-realm".to_string(),
            event_types: Some(vec!["LOGIN".to_string()]),
            client: Some("my-client".to_string()),
            user: Some("user-123".to_string()),
            date_from: None,
            date_to: None,
            ip_address: None,
            first: Some(0),
            max: Some(10),
        };

        let result = realm_events_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_realm_events_list_with_date_filters() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/admin/realms/test-realm/events"))
            .and(query_param("dateFrom", "2023-01-01"))
            .and(query_param("dateTo", "2023-12-31"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(Vec::<EventRepresentation>::new()),
            )
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmEventsListParams {
            realm: "test-realm".to_string(),
            event_types: None,
            client: None,
            user: None,
            date_from: Some("2023-01-01".to_string()),
            date_to: Some("2023-12-31".to_string()),
            ip_address: None,
            first: None,
            max: None,
        };

        let result = realm_events_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_realm_events_list_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/admin/realms/nonexistent/events"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmEventsListParams {
            realm: "nonexistent".to_string(),
            event_types: None,
            client: None,
            user: None,
            date_from: None,
            date_to: None,
            ip_address: None,
            first: None,
            max: None,
        };

        let result = realm_events_list(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_realm_events_list_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/admin/realms/test-realm/events"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmEventsListParams {
            realm: "test-realm".to_string(),
            event_types: None,
            client: None,
            user: None,
            date_from: None,
            date_to: None,
            ip_address: None,
            first: None,
            max: None,
        };

        let result = realm_events_list(&client, "invalid", &params).await;
        assert!(matches!(result, Err(ApiError::Unauthorized)));
    }

    // ==================== realm_events_delete tests ====================

    #[tokio::test]
    async fn test_realm_events_delete_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/admin/realms/test-realm/events"))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmEventsDeleteParams {
            realm: "test-realm".to_string(),
        };

        let result = realm_events_delete(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_realm_events_delete_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/admin/realms/nonexistent/events"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmEventsDeleteParams {
            realm: "nonexistent".to_string(),
        };

        let result = realm_events_delete(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_realm_events_delete_forbidden() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/admin/realms/test-realm/events"))
            .respond_with(ResponseTemplate::new(403))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmEventsDeleteParams {
            realm: "test-realm".to_string(),
        };

        let result = realm_events_delete(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::Forbidden)));
    }

    // ==================== realm_events_config_get tests ====================

    #[tokio::test]
    async fn test_realm_events_config_get_success() {
        let mock_server = MockServer::start().await;

        let expected_config = sample_events_config();

        Mock::given(method("GET"))
            .and(path("/admin/realms/test-realm/events/config"))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_config))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmEventsConfigGetParams {
            realm: "test-realm".to_string(),
        };

        let result = realm_events_config_get(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());

        let config = result.expect("Failed to get config");
        assert_eq!(config.events_enabled, Some(true));
        assert_eq!(config.admin_events_enabled, Some(true));
    }

    #[tokio::test]
    async fn test_realm_events_config_get_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/admin/realms/nonexistent/events/config"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmEventsConfigGetParams {
            realm: "nonexistent".to_string(),
        };

        let result = realm_events_config_get(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    // ==================== realm_events_config_update tests ====================

    #[tokio::test]
    async fn test_realm_events_config_update_success() {
        let mock_server = MockServer::start().await;

        let config = sample_events_config();

        Mock::given(method("PUT"))
            .and(path("/admin/realms/test-realm/events/config"))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .and(body_json(&config))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmEventsConfigUpdateParams {
            realm: "test-realm".to_string(),
            config,
        };

        let result = realm_events_config_update(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_realm_events_config_update_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path("/admin/realms/nonexistent/events/config"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmEventsConfigUpdateParams {
            realm: "nonexistent".to_string(),
            config: RealmEventsConfigRepresentation::default(),
        };

        let result = realm_events_config_update(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    // ==================== realm_admin_events_list tests ====================

    #[tokio::test]
    async fn test_realm_admin_events_list_success() {
        let mock_server = MockServer::start().await;

        let expected_events = vec![sample_admin_event()];

        Mock::given(method("GET"))
            .and(path("/admin/realms/test-realm/admin-events"))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_events))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmAdminEventsListParams {
            realm: "test-realm".to_string(),
            operation_types: None,
            resource_types: None,
            resource_path: None,
            auth_realm: None,
            auth_client: None,
            auth_user: None,
            auth_ip_address: None,
            date_from: None,
            date_to: None,
            first: None,
            max: None,
        };

        let result = realm_admin_events_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());

        let events = result.expect("Failed to get admin events");
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].operation_type, Some("CREATE".to_string()));
    }

    #[tokio::test]
    async fn test_realm_admin_events_list_with_filters() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/admin/realms/test-realm/admin-events"))
            .and(query_param("operationTypes", "CREATE"))
            .and(query_param("resourceTypes", "USER"))
            .and(query_param("authRealm", "master"))
            .and(query_param("first", "0"))
            .and(query_param("max", "50"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(Vec::<AdminEventRepresentation>::new()),
            )
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmAdminEventsListParams {
            realm: "test-realm".to_string(),
            operation_types: Some(vec!["CREATE".to_string()]),
            resource_types: Some(vec!["USER".to_string()]),
            resource_path: None,
            auth_realm: Some("master".to_string()),
            auth_client: None,
            auth_user: None,
            auth_ip_address: None,
            date_from: None,
            date_to: None,
            first: Some(0),
            max: Some(50),
        };

        let result = realm_admin_events_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_realm_admin_events_list_with_all_auth_filters() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/admin/realms/test-realm/admin-events"))
            .and(query_param("authClient", "admin-cli"))
            .and(query_param("authUser", "admin-user"))
            .and(query_param("authIpAddress", "10.0.0.1"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(Vec::<AdminEventRepresentation>::new()),
            )
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmAdminEventsListParams {
            realm: "test-realm".to_string(),
            operation_types: None,
            resource_types: None,
            resource_path: None,
            auth_realm: None,
            auth_client: Some("admin-cli".to_string()),
            auth_user: Some("admin-user".to_string()),
            auth_ip_address: Some("10.0.0.1".to_string()),
            date_from: None,
            date_to: None,
            first: None,
            max: None,
        };

        let result = realm_admin_events_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_realm_admin_events_list_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/admin/realms/nonexistent/admin-events"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmAdminEventsListParams {
            realm: "nonexistent".to_string(),
            operation_types: None,
            resource_types: None,
            resource_path: None,
            auth_realm: None,
            auth_client: None,
            auth_user: None,
            auth_ip_address: None,
            date_from: None,
            date_to: None,
            first: None,
            max: None,
        };

        let result = realm_admin_events_list(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_realm_admin_events_list_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/admin/realms/test-realm/admin-events"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmAdminEventsListParams {
            realm: "test-realm".to_string(),
            operation_types: None,
            resource_types: None,
            resource_path: None,
            auth_realm: None,
            auth_client: None,
            auth_user: None,
            auth_ip_address: None,
            date_from: None,
            date_to: None,
            first: None,
            max: None,
        };

        let result = realm_admin_events_list(&client, "invalid", &params).await;
        assert!(matches!(result, Err(ApiError::Unauthorized)));
    }

    // ==================== realm_admin_events_delete tests ====================

    #[tokio::test]
    async fn test_realm_admin_events_delete_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/admin/realms/test-realm/admin-events"))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmAdminEventsDeleteParams {
            realm: "test-realm".to_string(),
        };

        let result = realm_admin_events_delete(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_realm_admin_events_delete_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/admin/realms/nonexistent/admin-events"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmAdminEventsDeleteParams {
            realm: "nonexistent".to_string(),
        };

        let result = realm_admin_events_delete(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_realm_admin_events_delete_forbidden() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/admin/realms/test-realm/admin-events"))
            .respond_with(ResponseTemplate::new(403))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmAdminEventsDeleteParams {
            realm: "test-realm".to_string(),
        };

        let result = realm_admin_events_delete(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::Forbidden)));
    }

    // ==================== Parameter Validation Tests ====================

    #[test]
    fn test_realm_events_list_params_validation() {
        let params = RealmEventsListParams {
            realm: "valid-realm".to_string(),
            event_types: None,
            client: None,
            user: None,
            date_from: None,
            date_to: None,
            ip_address: None,
            first: None,
            max: None,
        };
        assert!(params.validate().is_ok());

        let params = RealmEventsListParams {
            realm: "".to_string(),
            event_types: None,
            client: None,
            user: None,
            date_from: None,
            date_to: None,
            ip_address: None,
            first: None,
            max: None,
        };
        assert!(params.validate().is_err());
    }

    #[test]
    fn test_realm_admin_events_list_params_validation() {
        let params = RealmAdminEventsListParams {
            realm: "valid-realm".to_string(),
            operation_types: None,
            resource_types: None,
            resource_path: None,
            auth_realm: None,
            auth_client: None,
            auth_user: None,
            auth_ip_address: None,
            date_from: None,
            date_to: None,
            first: None,
            max: None,
        };
        assert!(params.validate().is_ok());

        let params = RealmAdminEventsListParams {
            realm: "   ".to_string(),
            operation_types: None,
            resource_types: None,
            resource_path: None,
            auth_realm: None,
            auth_client: None,
            auth_user: None,
            auth_ip_address: None,
            date_from: None,
            date_to: None,
            first: None,
            max: None,
        };
        assert!(params.validate().is_err());
    }

    // ==================== Type Serialization Tests ====================

    #[test]
    fn test_event_representation_serialization() {
        let event = sample_event();

        let json = serde_json::to_string(&event).expect("Failed to serialize");
        assert!(json.contains("\"type\":\"LOGIN\""));
        assert!(json.contains("\"userId\":\"user-123\""));
        assert!(json.contains("\"clientId\":\"my-client\""));
    }

    #[test]
    fn test_event_representation_deserialization() {
        let json = r#"{
            "time": 1700000000000,
            "type": "LOGIN",
            "realmId": "test-realm",
            "clientId": "my-client",
            "userId": "user-123",
            "sessionId": "session-456",
            "ipAddress": "192.168.1.1"
        }"#;

        let event: EventRepresentation = serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(event.event_type, Some("LOGIN".to_string()));
        assert_eq!(event.user_id, Some("user-123".to_string()));
    }

    #[test]
    fn test_admin_event_representation_serialization() {
        let event = sample_admin_event();

        let json = serde_json::to_string(&event).expect("Failed to serialize");
        assert!(json.contains("\"operationType\":\"CREATE\""));
        assert!(json.contains("\"resourceType\":\"USER\""));
        assert!(json.contains("\"authUserId\":\"admin-user\""));
    }

    #[test]
    fn test_admin_event_representation_deserialization() {
        let json = r#"{
            "time": 1700000000000,
            "realmId": "test-realm",
            "authRealmId": "master",
            "authClientId": "admin-cli",
            "authUserId": "admin-user",
            "operationType": "UPDATE",
            "resourceType": "CLIENT",
            "resourcePath": "clients/client-id"
        }"#;

        let event: AdminEventRepresentation =
            serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(event.operation_type, Some("UPDATE".to_string()));
        assert_eq!(event.resource_type, Some("CLIENT".to_string()));
    }

    #[test]
    fn test_events_config_serialization() {
        let config = sample_events_config();

        let json = serde_json::to_string(&config).expect("Failed to serialize");
        assert!(json.contains("\"eventsEnabled\":true"));
        assert!(json.contains("\"adminEventsEnabled\":true"));
        assert!(json.contains("\"eventsExpiration\":86400"));
    }

    #[test]
    fn test_events_config_deserialization() {
        let json = r#"{
            "eventsEnabled": true,
            "eventsExpiration": 604800,
            "eventsListeners": ["jboss-logging", "email"],
            "enabledEventTypes": ["LOGIN", "LOGOUT"],
            "adminEventsEnabled": false,
            "adminEventsDetailsEnabled": false
        }"#;

        let config: RealmEventsConfigRepresentation =
            serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(config.events_enabled, Some(true));
        assert_eq!(config.events_expiration, Some(604800));
        assert_eq!(config.admin_events_enabled, Some(false));
    }

    // ==================== URL Encoding Tests ====================

    #[tokio::test]
    async fn test_realm_events_list_with_special_characters() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/admin/realms/my%20realm%2Ftest/events"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(Vec::<EventRepresentation>::new()),
            )
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmEventsListParams {
            realm: "my realm/test".to_string(),
            event_types: None,
            client: None,
            user: None,
            date_from: None,
            date_to: None,
            ip_address: None,
            first: None,
            max: None,
        };

        let result = realm_events_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_realm_admin_events_list_with_special_characters() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/admin/realms/my%20realm%2Ftest/admin-events"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(Vec::<AdminEventRepresentation>::new()),
            )
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmAdminEventsListParams {
            realm: "my realm/test".to_string(),
            operation_types: None,
            resource_types: None,
            resource_path: None,
            auth_realm: None,
            auth_client: None,
            auth_user: None,
            auth_ip_address: None,
            date_from: None,
            date_to: None,
            first: None,
            max: None,
        };

        let result = realm_admin_events_list(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    // ==================== Empty Realm Validation Tests ====================

    #[tokio::test]
    async fn test_realm_events_list_empty_realm() {
        let mock_server = MockServer::start().await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmEventsListParams {
            realm: "".to_string(),
            event_types: None,
            client: None,
            user: None,
            date_from: None,
            date_to: None,
            ip_address: None,
            first: None,
            max: None,
        };

        let result = realm_events_list(&client, TEST_TOKEN, &params).await;
        match result {
            Err(ApiError::BadRequest(msg)) => {
                assert!(msg.contains("cannot be empty"));
            }
            _ => panic!("Expected BadRequest error"),
        }
    }

    #[tokio::test]
    async fn test_realm_events_delete_empty_realm() {
        let mock_server = MockServer::start().await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmEventsDeleteParams {
            realm: "   ".to_string(),
        };

        let result = realm_events_delete(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::BadRequest(_))));
    }

    #[tokio::test]
    async fn test_realm_events_config_get_empty_realm() {
        let mock_server = MockServer::start().await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmEventsConfigGetParams {
            realm: "".to_string(),
        };

        let result = realm_events_config_get(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::BadRequest(_))));
    }

    #[tokio::test]
    async fn test_realm_events_config_update_empty_realm() {
        let mock_server = MockServer::start().await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmEventsConfigUpdateParams {
            realm: "".to_string(),
            config: RealmEventsConfigRepresentation::default(),
        };

        let result = realm_events_config_update(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::BadRequest(_))));
    }

    #[tokio::test]
    async fn test_realm_admin_events_list_empty_realm() {
        let mock_server = MockServer::start().await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmAdminEventsListParams {
            realm: "".to_string(),
            operation_types: None,
            resource_types: None,
            resource_path: None,
            auth_realm: None,
            auth_client: None,
            auth_user: None,
            auth_ip_address: None,
            date_from: None,
            date_to: None,
            first: None,
            max: None,
        };

        let result = realm_admin_events_list(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::BadRequest(_))));
    }

    #[tokio::test]
    async fn test_realm_admin_events_delete_empty_realm() {
        let mock_server = MockServer::start().await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = RealmAdminEventsDeleteParams {
            realm: "".to_string(),
        };

        let result = realm_admin_events_delete(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::BadRequest(_))));
    }
}
