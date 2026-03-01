//! End-to-end integration tests for Keycloak MCP Server API.
//!
//! These tests run against a real Keycloak instance and are marked with `#[ignore]`
//! so they won't run in CI without explicitly enabling them.
//!
//! To run these tests:
//! 1. Start Keycloak: `docker-compose up -d`
//! 2. Wait for Keycloak to be ready (usually ~30 seconds)
//! 3. Run: `cargo test --test integration -- --ignored`
//!
//! Environment variables:
//! - KEYCLOAK_URL: Base URL of Keycloak (default: http://localhost:8080)
//! - KEYCLOAK_ADMIN: Admin username (default: admin)
//! - KEYCLOAK_ADMIN_PASSWORD: Admin password (default: admin)

use keycloak_mcp_server::api::clients::{
    client_create, client_delete, client_get, ClientCreateParams, ClientDeleteParams,
    ClientGetParams, ClientRepresentation,
};
use keycloak_mcp_server::api::groups::{
    group_create, group_delete, group_get, group_list, GroupCreateParams, GroupDeleteParams,
    GroupGetParams, GroupListParams, GroupRepresentation,
};
use keycloak_mcp_server::api::realms::{realm_list, RealmListParams};
use keycloak_mcp_server::api::roles::{
    realm_role_create, realm_role_delete, realm_role_get, RoleCreateParams, RoleDeleteParams,
    RoleGetParams, RoleRepresentation,
};
use keycloak_mcp_server::api::users::groups::{
    user_group_add, user_group_remove, UserGroupAddParams, UserGroupRemoveParams,
};
use keycloak_mcp_server::api::users::roles::{
    user_realm_roles_add, user_realm_roles_list, user_realm_roles_remove, UserRealmRolesAddParams,
    UserRealmRolesListParams, UserRealmRolesRemoveParams,
};
use keycloak_mcp_server::api::users::{
    user_create, user_delete, user_get, user_list, user_update, UserCreateParams, UserDeleteParams,
    UserGetParams, UserListParams, UserRepresentation, UserUpdateParams,
};
use keycloak_mcp_server::api::KeycloakClient;
use reqwest::Client;
use serde::Deserialize;
use std::env;
use uuid::Uuid;

// ============================================================================
// Test Configuration & Helpers
// ============================================================================

/// Get the Keycloak base URL from environment or use default.
fn keycloak_url() -> String {
    env::var("KEYCLOAK_URL").unwrap_or_else(|_| "http://localhost:8080".to_string())
}

/// Get the admin username from environment or use default.
fn admin_username() -> String {
    env::var("KEYCLOAK_ADMIN").unwrap_or_else(|_| "admin".to_string())
}

/// Get the admin password from environment or use default.
fn admin_password() -> String {
    env::var("KEYCLOAK_ADMIN_PASSWORD").unwrap_or_else(|_| "admin".to_string())
}

/// Response from Keycloak token endpoint.
#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
}

/// Acquire an admin access token from Keycloak using the password grant.
async fn get_admin_token() -> Result<String, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/realms/master/protocol/openid-connect/token",
        keycloak_url()
    );

    let client = Client::new();
    let response = client
        .post(&url)
        .form(&[
            ("client_id", "admin-cli"),
            ("grant_type", "password"),
            ("username", &admin_username()),
            ("password", &admin_password()),
        ])
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("Failed to get token: {} - {}", status, body).into());
    }

    let token_response: TokenResponse = response.json().await?;
    Ok(token_response.access_token)
}

/// Generate a unique test resource name using UUID.
fn unique_name(prefix: &str) -> String {
    format!("{}-{}", prefix, &Uuid::new_v4().to_string()[..8])
}

/// Helper to find a user by username and return their ID.
async fn find_user_id(
    client: &KeycloakClient,
    token: &str,
    realm: &str,
    username: &str,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let params = UserListParams {
        realm: realm.to_string(),
        username: Some(username.to_string()),
        exact: Some(true),
        search: None,
        first: None,
        max: None,
        email: None,
        first_name: None,
        last_name: None,
        enabled: None,
    };

    let users = user_list(client, token, &params).await?;
    Ok(users.first().and_then(|u| u.id.clone()))
}

/// Helper to find a group by name and return its ID.
async fn find_group_id(
    client: &KeycloakClient,
    token: &str,
    realm: &str,
    group_name: &str,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let params = GroupListParams {
        realm: realm.to_string(),
        search: Some(group_name.to_string()),
        exact: Some(true),
        first: None,
        max: None,
        brief_representation: None,
        q: None,
    };

    let groups = group_list(client, token, &params).await?;
    Ok(groups
        .iter()
        .find(|g| g.name.as_deref() == Some(group_name))
        .and_then(|g| g.id.clone()))
}

// ============================================================================
// Integration Tests
// ============================================================================

/// Test 1: User lifecycle - create, get, update, delete
#[tokio::test]
#[ignore]
async fn test_user_lifecycle() {
    let token = get_admin_token().await.expect("Failed to get admin token");
    let client = KeycloakClient::new(keycloak_url()).expect("Failed to create client");
    let realm = "master";

    let test_username = unique_name("testuser");
    let test_email = format!("{}@example.com", test_username);

    // Create user
    let create_params = UserCreateParams {
        realm: realm.to_string(),
        user: UserRepresentation {
            username: Some(test_username.clone()),
            email: Some(test_email.clone()),
            enabled: Some(true),
            first_name: Some("Test".to_string()),
            last_name: Some("User".to_string()),
            ..Default::default()
        },
    };

    user_create(&client, &token, &create_params)
        .await
        .expect("Failed to create user");

    // Find the user ID (Keycloak doesn't return ID on create)
    let user_id = find_user_id(&client, &token, realm, &test_username)
        .await
        .expect("Failed to find user")
        .expect("User should exist after creation");

    // Get user
    let get_params = UserGetParams {
        realm: realm.to_string(),
        user_id: user_id.clone(),
    };

    let user = user_get(&client, &token, &get_params)
        .await
        .expect("Failed to get user");

    assert_eq!(user.username.as_deref(), Some(test_username.as_str()));
    assert_eq!(user.email.as_deref(), Some(test_email.as_str()));
    assert_eq!(user.enabled, Some(true));

    // Update user
    let update_params = UserUpdateParams {
        realm: realm.to_string(),
        user_id: user_id.clone(),
        user: UserRepresentation {
            first_name: Some("Updated".to_string()),
            last_name: Some("Name".to_string()),
            ..Default::default()
        },
    };

    user_update(&client, &token, &update_params)
        .await
        .expect("Failed to update user");

    // Verify update
    let updated_user = user_get(&client, &token, &get_params)
        .await
        .expect("Failed to get updated user");

    assert_eq!(updated_user.first_name.as_deref(), Some("Updated"));
    assert_eq!(updated_user.last_name.as_deref(), Some("Name"));

    // Delete user (cleanup)
    let delete_params = UserDeleteParams {
        realm: realm.to_string(),
        user_id: user_id.clone(),
    };

    user_delete(&client, &token, &delete_params)
        .await
        .expect("Failed to delete user");

    // Verify deletion
    let result = user_get(&client, &token, &get_params).await;
    assert!(result.is_err(), "User should not exist after deletion");
}

/// Test 2: OAuth Client lifecycle - create, get, delete
#[tokio::test]
#[ignore]
async fn test_client_lifecycle() {
    let token = get_admin_token().await.expect("Failed to get admin token");
    let client = KeycloakClient::new(keycloak_url()).expect("Failed to create client");
    let realm = "master";

    let test_client_id = unique_name("test-client");

    // Create client
    let create_params = ClientCreateParams {
        realm: realm.to_string(),
        client: ClientRepresentation {
            client_id: Some(test_client_id.clone()),
            name: Some("Test OAuth Client".to_string()),
            enabled: Some(true),
            public_client: Some(false),
            protocol: Some("openid-connect".to_string()),
            ..Default::default()
        },
    };

    client_create(&client, &token, &create_params)
        .await
        .expect("Failed to create client");

    // Find the client ID (internal UUID)
    let clients = keycloak_mcp_server::api::clients::client_list(
        &client,
        &token,
        &keycloak_mcp_server::api::clients::ClientListParams {
            realm: realm.to_string(),
            client_id: Some(test_client_id.clone()),
            search: None,
            viewable_only: None,
            first: None,
            max: None,
        },
    )
    .await
    .expect("Failed to list clients");

    let internal_id = clients
        .first()
        .and_then(|c| c.id.clone())
        .expect("Client should exist after creation");

    // Get client
    let get_params = ClientGetParams {
        realm: realm.to_string(),
        id: internal_id.clone(),
    };

    let fetched_client = client_get(&client, &token, &get_params)
        .await
        .expect("Failed to get client");

    assert_eq!(
        fetched_client.client_id.as_deref(),
        Some(test_client_id.as_str())
    );
    assert_eq!(fetched_client.enabled, Some(true));
    assert_eq!(fetched_client.public_client, Some(false));

    // Delete client (cleanup)
    let delete_params = ClientDeleteParams {
        realm: realm.to_string(),
        id: internal_id.clone(),
    };

    client_delete(&client, &token, &delete_params)
        .await
        .expect("Failed to delete client");

    // Verify deletion
    let result = client_get(&client, &token, &get_params).await;
    assert!(result.is_err(), "Client should not exist after deletion");
}

/// Test 3: Realm Role lifecycle - create, get, assign to user, remove from user, delete
#[tokio::test]
#[ignore]
async fn test_role_lifecycle() {
    let token = get_admin_token().await.expect("Failed to get admin token");
    let client = KeycloakClient::new(keycloak_url()).expect("Failed to create client");
    let realm = "master";

    let test_role_name = unique_name("test-role");
    let test_username = unique_name("roleuser");

    // Create a test user first
    let user_create_params = UserCreateParams {
        realm: realm.to_string(),
        user: UserRepresentation {
            username: Some(test_username.clone()),
            enabled: Some(true),
            ..Default::default()
        },
    };

    user_create(&client, &token, &user_create_params)
        .await
        .expect("Failed to create test user");

    let user_id = find_user_id(&client, &token, realm, &test_username)
        .await
        .expect("Failed to find user")
        .expect("User should exist");

    // Create role
    let role_create_params = RoleCreateParams {
        realm: realm.to_string(),
        name: test_role_name.clone(),
        description: Some("Integration test role".to_string()),
        attributes: None,
    };

    realm_role_create(&client, &token, &role_create_params)
        .await
        .expect("Failed to create role");

    // Get role to verify creation and get ID
    let role_get_params = RoleGetParams {
        realm: realm.to_string(),
        role_name: test_role_name.clone(),
    };

    let role = realm_role_get(&client, &token, &role_get_params)
        .await
        .expect("Failed to get role");

    assert_eq!(role.name.as_deref(), Some(test_role_name.as_str()));
    let role_id = role.id.clone().expect("Role should have ID");

    // Assign role to user
    let add_roles_params = UserRealmRolesAddParams {
        realm: realm.to_string(),
        user_id: user_id.clone(),
        roles: vec![RoleRepresentation {
            id: Some(role_id.clone()),
            name: Some(test_role_name.clone()),
            ..Default::default()
        }],
    };

    user_realm_roles_add(&client, &token, &add_roles_params)
        .await
        .expect("Failed to add role to user");

    // Verify role assignment
    let list_roles_params = UserRealmRolesListParams {
        realm: realm.to_string(),
        user_id: user_id.clone(),
    };

    let user_roles = user_realm_roles_list(&client, &token, &list_roles_params)
        .await
        .expect("Failed to list user roles");

    assert!(
        user_roles
            .iter()
            .any(|r| r.name.as_deref() == Some(&test_role_name)),
        "User should have the test role assigned"
    );

    // Remove role from user
    let remove_roles_params = UserRealmRolesRemoveParams {
        realm: realm.to_string(),
        user_id: user_id.clone(),
        roles: vec![RoleRepresentation {
            id: Some(role_id.clone()),
            name: Some(test_role_name.clone()),
            ..Default::default()
        }],
    };

    user_realm_roles_remove(&client, &token, &remove_roles_params)
        .await
        .expect("Failed to remove role from user");

    // Verify role removal
    let user_roles_after = user_realm_roles_list(&client, &token, &list_roles_params)
        .await
        .expect("Failed to list user roles after removal");

    assert!(
        !user_roles_after
            .iter()
            .any(|r| r.name.as_deref() == Some(&test_role_name)),
        "User should not have the test role after removal"
    );

    // Cleanup: delete role
    let role_delete_params = RoleDeleteParams {
        realm: realm.to_string(),
        role_name: test_role_name.clone(),
    };

    realm_role_delete(&client, &token, &role_delete_params)
        .await
        .expect("Failed to delete role");

    // Cleanup: delete user
    let user_delete_params = UserDeleteParams {
        realm: realm.to_string(),
        user_id,
    };

    user_delete(&client, &token, &user_delete_params)
        .await
        .expect("Failed to delete test user");
}

/// Test 4: Group lifecycle - create, get, add member, remove member, delete
#[tokio::test]
#[ignore]
async fn test_group_lifecycle() {
    let token = get_admin_token().await.expect("Failed to get admin token");
    let client = KeycloakClient::new(keycloak_url()).expect("Failed to create client");
    let realm = "master";

    let test_group_name = unique_name("test-group");
    let test_username = unique_name("groupuser");

    // Create a test user first
    let user_create_params = UserCreateParams {
        realm: realm.to_string(),
        user: UserRepresentation {
            username: Some(test_username.clone()),
            enabled: Some(true),
            ..Default::default()
        },
    };

    user_create(&client, &token, &user_create_params)
        .await
        .expect("Failed to create test user");

    let user_id = find_user_id(&client, &token, realm, &test_username)
        .await
        .expect("Failed to find user")
        .expect("User should exist");

    // Create group
    let group_create_params = GroupCreateParams {
        realm: realm.to_string(),
        group: GroupRepresentation {
            name: Some(test_group_name.clone()),
            ..Default::default()
        },
    };

    group_create(&client, &token, &group_create_params)
        .await
        .expect("Failed to create group");

    // Find group ID
    let group_id = find_group_id(&client, &token, realm, &test_group_name)
        .await
        .expect("Failed to find group")
        .expect("Group should exist after creation");

    // Get group to verify creation
    let group_get_params = GroupGetParams {
        realm: realm.to_string(),
        group_id: group_id.clone(),
    };

    let group = group_get(&client, &token, &group_get_params)
        .await
        .expect("Failed to get group");

    assert_eq!(group.name.as_deref(), Some(test_group_name.as_str()));

    // Add user to group
    let add_group_params = UserGroupAddParams {
        realm: realm.to_string(),
        user_id: user_id.clone(),
        group_id: group_id.clone(),
    };

    user_group_add(&client, &token, &add_group_params)
        .await
        .expect("Failed to add user to group");

    // Verify user is in group (list group members)
    let members = keycloak_mcp_server::api::groups::group_members_list(
        &client,
        &token,
        &keycloak_mcp_server::api::groups::GroupMembersListParams {
            realm: realm.to_string(),
            group_id: group_id.clone(),
            first: None,
            max: None,
            brief_representation: None,
        },
    )
    .await
    .expect("Failed to list group members");

    assert!(
        members
            .iter()
            .any(|m| m.username.as_deref() == Some(&test_username)),
        "User should be a member of the group"
    );

    // Remove user from group
    let remove_group_params = UserGroupRemoveParams {
        realm: realm.to_string(),
        user_id: user_id.clone(),
        group_id: group_id.clone(),
    };

    user_group_remove(&client, &token, &remove_group_params)
        .await
        .expect("Failed to remove user from group");

    // Verify user is no longer in group
    let members_after = keycloak_mcp_server::api::groups::group_members_list(
        &client,
        &token,
        &keycloak_mcp_server::api::groups::GroupMembersListParams {
            realm: realm.to_string(),
            group_id: group_id.clone(),
            first: None,
            max: None,
            brief_representation: None,
        },
    )
    .await
    .expect("Failed to list group members after removal");

    assert!(
        !members_after
            .iter()
            .any(|m| m.username.as_deref() == Some(&test_username)),
        "User should not be a member of the group after removal"
    );

    // Cleanup: delete group
    let group_delete_params = GroupDeleteParams {
        realm: realm.to_string(),
        group_id,
    };

    group_delete(&client, &token, &group_delete_params)
        .await
        .expect("Failed to delete group");

    // Cleanup: delete user
    let user_delete_params = UserDeleteParams {
        realm: realm.to_string(),
        user_id,
    };

    user_delete(&client, &token, &user_delete_params)
        .await
        .expect("Failed to delete test user");
}

/// Test 5: Realm operations - list realms (read-only)
#[tokio::test]
#[ignore]
async fn test_realm_list() {
    let token = get_admin_token().await.expect("Failed to get admin token");
    let client = KeycloakClient::new(keycloak_url()).expect("Failed to create client");

    let params = RealmListParams {
        brief_representation: Some(true),
    };

    let realms = realm_list(&client, &token, &params)
        .await
        .expect("Failed to list realms");

    // At minimum, the master realm should exist
    assert!(!realms.is_empty(), "Should have at least one realm");
    assert!(
        realms.iter().any(|r| r.realm.as_deref() == Some("master")),
        "Master realm should exist"
    );
}

/// Test 6: Token acquisition - verify we can get a valid token
#[tokio::test]
#[ignore]
async fn test_token_acquisition() {
    let token = get_admin_token().await;
    assert!(token.is_ok(), "Should be able to acquire admin token");

    let token_value = token.unwrap();
    assert!(!token_value.is_empty(), "Token should not be empty");

    // Verify token works by making an API call
    let client = KeycloakClient::new(keycloak_url()).expect("Failed to create client");
    let params = RealmListParams {
        brief_representation: Some(true),
    };

    let result = realm_list(&client, &token_value, &params).await;
    assert!(result.is_ok(), "Token should be valid for API calls");
}

/// Test 7: Error handling - verify proper errors for non-existent resources
#[tokio::test]
#[ignore]
async fn test_error_handling() {
    let token = get_admin_token().await.expect("Failed to get admin token");
    let client = KeycloakClient::new(keycloak_url()).expect("Failed to create client");
    let realm = "master";

    // Try to get a non-existent user
    let get_params = UserGetParams {
        realm: realm.to_string(),
        user_id: "non-existent-user-id-12345".to_string(),
    };

    let result = user_get(&client, &token, &get_params).await;
    assert!(result.is_err(), "Should fail for non-existent user");

    // Try to get a non-existent role
    let role_params = RoleGetParams {
        realm: realm.to_string(),
        role_name: "non-existent-role-12345".to_string(),
    };

    let result = realm_role_get(&client, &token, &role_params).await;
    assert!(result.is_err(), "Should fail for non-existent role");

    // Try to get a non-existent group
    let group_params = GroupGetParams {
        realm: realm.to_string(),
        group_id: "non-existent-group-id-12345".to_string(),
    };

    let result = group_get(&client, &token, &group_params).await;
    assert!(result.is_err(), "Should fail for non-existent group");
}
