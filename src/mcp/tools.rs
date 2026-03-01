//! Tool registration and routing for Keycloak MCP Server.
//!
//! This module defines all MCP tools available for Keycloak administration.
//! Each tool corresponds to a Keycloak Admin REST API operation.

#![allow(dead_code)]

use rmcp::{
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{CallToolResult, Content},
    tool, tool_router, ErrorData as McpError,
};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::api::{
    authentication::{
        auth_execution_config_get, auth_execution_config_update, auth_execution_create,
        auth_execution_delete, auth_execution_get, auth_execution_lower_priority,
        auth_execution_raise_priority, auth_flow_copy, auth_flow_create, auth_flow_delete,
        auth_flow_executions_list, auth_flow_get, auth_flows_list, auth_required_action_get,
        auth_required_action_lower_priority, auth_required_action_raise_priority,
        auth_required_action_register, auth_required_action_update, auth_required_actions_list,
        auth_unregistered_required_actions_list, AuthExecutionConfigGetParams,
        AuthExecutionConfigUpdateParams, AuthExecutionCreateParams, AuthExecutionDeleteParams,
        AuthExecutionGetParams, AuthExecutionPriorityParams, AuthFlowCopyParams,
        AuthFlowCreateParams, AuthFlowDeleteParams, AuthFlowExecutionsListParams,
        AuthFlowGetParams, AuthFlowsListParams, AuthRequiredActionGetParams,
        AuthRequiredActionLowerPriorityParams, AuthRequiredActionRaisePriorityParams,
        AuthRequiredActionRegisterParams, AuthRequiredActionUpdateParams,
        AuthRequiredActionsListParams, AuthUnregisteredRequiredActionsListParams,
    },
    authorization::{
        authz_evaluate, authz_permission_create, authz_permission_delete, authz_permission_get,
        authz_permission_update, authz_permissions_list, authz_policies_list, authz_policy_create,
        authz_policy_delete, authz_policy_get, authz_policy_update, authz_resource_create,
        authz_resource_delete, authz_resource_get, authz_resource_server_get,
        authz_resource_server_update, authz_resource_update, authz_resources_list,
        authz_scope_create, authz_scope_delete, authz_scopes_list, PermissionCreateParams,
        PermissionDeleteParams, PermissionGetParams, PermissionListParams, PermissionUpdateParams,
        PolicyCreateParams, PolicyDeleteParams, PolicyEvaluateParams, PolicyGetParams,
        PolicyListParams, PolicyUpdateParams, ResourceCreateParams, ResourceDeleteParams,
        ResourceGetParams, ResourceListParams, ResourceServerGetParams, ResourceServerUpdateParams,
        ResourceUpdateParams, ScopeCreateParams, ScopeDeleteParams, ScopeListParams,
    },
    client_scopes::{
        client_scope_create, client_scope_delete, client_scope_get, client_scope_list,
        client_scope_protocol_mapper_create, client_scope_protocol_mapper_delete,
        client_scope_protocol_mapper_get, client_scope_protocol_mapper_update,
        client_scope_protocol_mappers_list, client_scope_scope_mappings_realm_add,
        client_scope_scope_mappings_realm_list, client_scope_update, ClientScopeCreateParams,
        ClientScopeDeleteParams, ClientScopeGetParams, ClientScopeListParams,
        ClientScopeProtocolMapperCreateParams, ClientScopeProtocolMapperDeleteParams,
        ClientScopeProtocolMapperGetParams, ClientScopeProtocolMapperUpdateParams,
        ClientScopeProtocolMappersListParams, ClientScopeScopeMappingsRealmAddParams,
        ClientScopeScopeMappingsRealmListParams, ClientScopeUpdateParams,
    },
    clients::{
        client_certificate_generate, client_certificates_get, client_create, client_delete,
        client_get, client_list, client_protocol_mapper_create, client_protocol_mapper_delete,
        client_protocol_mapper_get, client_protocol_mapper_update, client_protocol_mappers_list,
        client_registration_token_get, client_registration_token_regenerate, client_role_create,
        client_role_delete, client_role_get, client_role_groups_list, client_role_update,
        client_role_users_list, client_roles_list, client_scope_mappings_client_add,
        client_scope_mappings_client_list, client_scope_mappings_client_remove,
        client_scope_mappings_realm_add, client_scope_mappings_realm_list,
        client_scope_mappings_realm_remove, client_secret_get, client_secret_regenerate,
        client_service_account_user_get, client_update, ClientCertificateGenerateParams,
        ClientCertificatesGetParams, ClientCreateParams, ClientDeleteParams, ClientGetParams,
        ClientListParams, ClientProtocolMapperCreateParams, ClientProtocolMapperDeleteParams,
        ClientProtocolMapperGetParams, ClientProtocolMapperUpdateParams,
        ClientProtocolMappersListParams, ClientRegistrationTokenGetParams,
        ClientRegistrationTokenRegenerateParams, ClientRoleCreateParams, ClientRoleDeleteParams,
        ClientRoleGetParams, ClientRoleGroupsListParams, ClientRoleListParams,
        ClientRoleUpdateParams, ClientRoleUsersListParams, ClientScopeMappingsClientAddParams,
        ClientScopeMappingsClientListParams, ClientScopeMappingsClientRemoveParams,
        ClientScopeMappingsRealmAddParams, ClientScopeMappingsRealmListParams,
        ClientScopeMappingsRealmRemoveParams, ClientSecretGetParams, ClientSecretRegenerateParams,
        ClientServiceAccountUserGetParams, ClientUpdateParams,
    },
    groups::{
        group_child_create, group_children_list, group_count, group_create, group_delete,
        group_get, group_list, group_members_list, group_set_parent, group_update,
        role_mappings::{
            group_client_roles_add, group_client_roles_list, group_client_roles_remove,
            group_realm_roles_add, group_realm_roles_available, group_realm_roles_list,
            group_realm_roles_remove, GroupClientRolesAddParams, GroupClientRolesListParams,
            GroupClientRolesRemoveParams, GroupRealmRolesAddParams, GroupRealmRolesAvailableParams,
            GroupRealmRolesListParams, GroupRealmRolesRemoveParams,
        },
        GroupChildCreateParams, GroupChildrenListParams, GroupCountParams, GroupCreateParams,
        GroupDeleteParams, GroupGetParams, GroupListParams, GroupMembersListParams,
        GroupSetParentParams, GroupUpdateParams,
    },
    identity_providers::{
        idp_create, idp_delete, idp_get, idp_list, idp_mapper_create, idp_mapper_delete,
        idp_mapper_get, idp_mapper_types_list, idp_mapper_update, idp_mappers_list,
        idp_providers_list, idp_update, IdpCreateParams, IdpDeleteParams, IdpGetParams,
        IdpListParams, IdpMapperCreateParams, IdpMapperDeleteParams, IdpMapperGetParams,
        IdpMapperTypesListParams, IdpMapperUpdateParams, IdpMappersListParams,
        IdpProvidersGetParams, IdpUpdateParams,
    },
    realms::{
        realm_admin_events_delete, realm_admin_events_list, realm_create,
        realm_default_client_scopes_list, realm_default_group_add, realm_default_group_remove,
        realm_default_groups_list, realm_default_optional_scopes_list, realm_delete,
        realm_events_config_get, realm_events_config_update, realm_events_delete,
        realm_events_list, realm_get, realm_keys_list, realm_list, realm_logout_all,
        realm_push_revocation, realm_sessions_list, realm_update, RealmAdminEventsDeleteParams,
        RealmAdminEventsListParams, RealmCreateParams, RealmDefaultClientScopesListParams,
        RealmDefaultGroupAddParams, RealmDefaultGroupRemoveParams, RealmDefaultGroupsListParams,
        RealmDefaultOptionalScopesListParams, RealmDeleteParams, RealmEventsConfigGetParams,
        RealmEventsConfigUpdateParams, RealmEventsDeleteParams, RealmEventsListParams,
        RealmGetParams, RealmKeysListParams, RealmListParams, RealmLogoutAllParams,
        RealmPushRevocationParams, RealmSessionsListParams, RealmUpdateParams,
    },
    roles::{
        realm_role_composites_add, realm_role_composites_list, realm_role_composites_remove,
        realm_role_create, realm_role_delete, realm_role_get, realm_role_groups_list,
        realm_role_list, realm_role_update, realm_role_users_list, role_by_id_composites_add,
        role_by_id_composites_list, role_by_id_composites_remove, role_by_id_delete,
        role_by_id_get, role_by_id_update, RoleByIdCompositesAddParams,
        RoleByIdCompositesListParams, RoleByIdCompositesRemoveParams, RoleByIdDeleteParams,
        RoleByIdGetParams, RoleByIdUpdateParams, RoleCompositesAddParams, RoleCompositesListParams,
        RoleCompositesRemoveParams, RoleCreateParams, RoleDeleteParams, RoleGetParams,
        RoleGroupsListParams, RoleListParams, RoleUpdateParams, RoleUsersListParams,
    },
    users::{
        credentials::{
            user_credential_delete, user_credentials_list, user_disable_credentials,
            user_execute_actions_email, user_reset_password, user_send_verify_email,
            UserCredentialDeleteParams, UserCredentialsListParams, UserDisableCredentialsParams,
            UserExecuteActionsEmailParams, UserResetPasswordParams, UserSendVerifyEmailParams,
        },
        groups::{
            user_group_add, user_group_count, user_group_remove, user_groups_list,
            UserGroupAddParams, UserGroupCountParams, UserGroupRemoveParams, UserGroupsListParams,
        },
        roles::{
            user_client_roles_add, user_client_roles_list, user_client_roles_remove,
            user_realm_roles_add, user_realm_roles_available, user_realm_roles_list,
            user_realm_roles_remove, UserClientRolesAddParams, UserClientRolesListParams,
            UserClientRolesRemoveParams, UserRealmRolesAddParams, UserRealmRolesAvailableParams,
            UserRealmRolesListParams, UserRealmRolesRemoveParams,
        },
        sessions::{
            user_consent_revoke, user_consents_list, user_federated_identity_add,
            user_federated_identity_list, user_federated_identity_remove, user_logout,
            user_sessions_list, UserConsentRevokeParams, UserConsentsListParams,
            UserFederatedIdentityAddParams, UserFederatedIdentityListParams,
            UserFederatedIdentityRemoveParams, UserLogoutParams, UserSessionsListParams,
        },
        user_create, user_delete, user_get, user_list, user_update, UserCreateParams,
        UserDeleteParams, UserGetParams, UserListParams, UserUpdateParams,
    },
    KeycloakClient,
};

/// MCP Tool Handler for Keycloak operations.
///
/// This struct provides all Keycloak administration tools via the MCP protocol.
/// It maintains a connection to Keycloak and handles authentication.
#[derive(Clone)]
pub struct KeycloakToolHandler {
    #[allow(dead_code)]
    client: Arc<KeycloakClient>,
    #[allow(dead_code)]
    token: Arc<RwLock<String>>,
    tool_router: ToolRouter<Self>,
}

impl KeycloakToolHandler {
    /// Create a new tool handler with the given Keycloak client and token.
    pub fn new(client: KeycloakClient, token: String) -> Self {
        Self {
            client: Arc::new(client),
            token: Arc::new(RwLock::new(token)),
            tool_router: Self::tool_router(),
        }
    }

    /// Update the authentication token.
    pub async fn set_token(&self, token: String) {
        let mut t = self.token.write().await;
        *t = token;
    }

    /// Get a reference to the tool router.
    pub fn router(&self) -> &ToolRouter<Self> {
        &self.tool_router
    }

    #[allow(dead_code)]
    async fn get_token(&self) -> String {
        self.token.read().await.clone()
    }
}

// Helper macro to reduce boilerplate for tool implementations
macro_rules! impl_list_tool {
    ($fn_name:ident, $api_fn:ident, $params_type:ty, $desc:literal) => {
        #[tool(description = $desc)]
        async fn $fn_name(
            &self,
            params: Parameters<$params_type>,
        ) -> Result<CallToolResult, McpError> {
            let token = self.get_token().await;
            match $api_fn(&self.client, &token, &params.0).await {
                Ok(result) => {
                    let json = serde_json::to_string_pretty(&result)
                        .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                    Ok(CallToolResult::success(vec![Content::text(json)]))
                }
                Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
            }
        }
    };
}

macro_rules! impl_get_tool {
    ($fn_name:ident, $api_fn:ident, $params_type:ty, $desc:literal) => {
        #[tool(description = $desc)]
        async fn $fn_name(
            &self,
            params: Parameters<$params_type>,
        ) -> Result<CallToolResult, McpError> {
            let token = self.get_token().await;
            match $api_fn(&self.client, &token, &params.0).await {
                Ok(result) => {
                    let json = serde_json::to_string_pretty(&result)
                        .map_err(|e| McpError::internal_error(e.to_string(), None))?;
                    Ok(CallToolResult::success(vec![Content::text(json)]))
                }
                Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
            }
        }
    };
}

macro_rules! impl_action_tool {
    ($fn_name:ident, $api_fn:ident, $params_type:ty, $desc:literal, $success_msg:literal) => {
        #[tool(description = $desc)]
        async fn $fn_name(
            &self,
            params: Parameters<$params_type>,
        ) -> Result<CallToolResult, McpError> {
            let token = self.get_token().await;
            match $api_fn(&self.client, &token, &params.0).await {
                Ok(_) => Ok(CallToolResult::success(vec![Content::text($success_msg)])),
                Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
            }
        }
    };
}

#[tool_router]
impl KeycloakToolHandler {
    // ============================================================================
    // USERS API
    // ============================================================================

    impl_list_tool!(
        user_list,
        user_list,
        UserListParams,
        "List users in a realm with optional filters (search, email, username, etc.)"
    );

    impl_get_tool!(user_get, user_get, UserGetParams, "Get a user by ID");

    impl_action_tool!(
        user_create,
        user_create,
        UserCreateParams,
        "Create a new user in a realm",
        "User created successfully"
    );

    impl_action_tool!(
        user_update,
        user_update,
        UserUpdateParams,
        "Update an existing user",
        "User updated successfully"
    );

    impl_action_tool!(
        user_delete,
        user_delete,
        UserDeleteParams,
        "Delete a user from a realm",
        "User deleted successfully"
    );

    // User Credentials
    impl_action_tool!(
        user_reset_password,
        user_reset_password,
        UserResetPasswordParams,
        "Reset a user's password",
        "Password reset successfully"
    );

    impl_list_tool!(
        user_credentials_list,
        user_credentials_list,
        UserCredentialsListParams,
        "List credentials for a user"
    );

    impl_action_tool!(
        user_credential_delete,
        user_credential_delete,
        UserCredentialDeleteParams,
        "Delete a specific credential from a user",
        "Credential deleted successfully"
    );

    impl_action_tool!(
        user_disable_credentials,
        user_disable_credentials,
        UserDisableCredentialsParams,
        "Disable credentials of a specific type for a user",
        "Credentials disabled successfully"
    );

    impl_action_tool!(
        user_send_verify_email,
        user_send_verify_email,
        UserSendVerifyEmailParams,
        "Send a verification email to a user",
        "Verification email sent successfully"
    );

    impl_action_tool!(
        user_execute_actions_email,
        user_execute_actions_email,
        UserExecuteActionsEmailParams,
        "Send an email with required actions to a user",
        "Actions email sent successfully"
    );

    // User Groups
    impl_list_tool!(
        user_groups_list,
        user_groups_list,
        UserGroupsListParams,
        "List groups that a user belongs to"
    );

    impl_action_tool!(
        user_group_add,
        user_group_add,
        UserGroupAddParams,
        "Add a user to a group",
        "User added to group successfully"
    );

    impl_action_tool!(
        user_group_remove,
        user_group_remove,
        UserGroupRemoveParams,
        "Remove a user from a group",
        "User removed from group successfully"
    );

    impl_get_tool!(
        user_group_count,
        user_group_count,
        UserGroupCountParams,
        "Get the count of groups a user belongs to"
    );

    // User Roles
    impl_list_tool!(
        user_realm_roles_list,
        user_realm_roles_list,
        UserRealmRolesListParams,
        "List realm-level roles assigned to a user"
    );

    impl_action_tool!(
        user_realm_roles_add,
        user_realm_roles_add,
        UserRealmRolesAddParams,
        "Add realm-level roles to a user",
        "Realm roles added to user successfully"
    );

    impl_action_tool!(
        user_realm_roles_remove,
        user_realm_roles_remove,
        UserRealmRolesRemoveParams,
        "Remove realm-level roles from a user",
        "Realm roles removed from user successfully"
    );

    impl_list_tool!(
        user_realm_roles_available,
        user_realm_roles_available,
        UserRealmRolesAvailableParams,
        "List available realm-level roles that can be assigned to a user"
    );

    impl_list_tool!(
        user_client_roles_list,
        user_client_roles_list,
        UserClientRolesListParams,
        "List client-level roles assigned to a user"
    );

    impl_action_tool!(
        user_client_roles_add,
        user_client_roles_add,
        UserClientRolesAddParams,
        "Add client-level roles to a user",
        "Client roles added to user successfully"
    );

    impl_action_tool!(
        user_client_roles_remove,
        user_client_roles_remove,
        UserClientRolesRemoveParams,
        "Remove client-level roles from a user",
        "Client roles removed from user successfully"
    );

    // User Sessions
    impl_list_tool!(
        user_sessions_list,
        user_sessions_list,
        UserSessionsListParams,
        "List active sessions for a user"
    );

    impl_action_tool!(
        user_logout,
        user_logout,
        UserLogoutParams,
        "Log out a user from all sessions",
        "User logged out successfully"
    );

    impl_list_tool!(
        user_consents_list,
        user_consents_list,
        UserConsentsListParams,
        "List consents granted by a user"
    );

    impl_action_tool!(
        user_consent_revoke,
        user_consent_revoke,
        UserConsentRevokeParams,
        "Revoke a user's consent for a client",
        "Consent revoked successfully"
    );

    impl_list_tool!(
        user_federated_identity_list,
        user_federated_identity_list,
        UserFederatedIdentityListParams,
        "List federated identities linked to a user"
    );

    impl_action_tool!(
        user_federated_identity_add,
        user_federated_identity_add,
        UserFederatedIdentityAddParams,
        "Add a federated identity link to a user",
        "Federated identity added successfully"
    );

    impl_action_tool!(
        user_federated_identity_remove,
        user_federated_identity_remove,
        UserFederatedIdentityRemoveParams,
        "Remove a federated identity link from a user",
        "Federated identity removed successfully"
    );

    // ============================================================================
    // CLIENTS API
    // ============================================================================

    impl_list_tool!(
        client_list,
        client_list,
        ClientListParams,
        "List clients in a realm with optional filters"
    );

    impl_get_tool!(
        client_get,
        client_get,
        ClientGetParams,
        "Get a client by ID"
    );

    impl_action_tool!(
        client_create,
        client_create,
        ClientCreateParams,
        "Create a new client in a realm",
        "Client created successfully"
    );

    impl_action_tool!(
        client_update,
        client_update,
        ClientUpdateParams,
        "Update an existing client",
        "Client updated successfully"
    );

    impl_action_tool!(
        client_delete,
        client_delete,
        ClientDeleteParams,
        "Delete a client from a realm",
        "Client deleted successfully"
    );

    // Client Credentials
    impl_get_tool!(
        client_secret_get,
        client_secret_get,
        ClientSecretGetParams,
        "Get the client secret for a confidential client"
    );

    impl_get_tool!(
        client_secret_regenerate,
        client_secret_regenerate,
        ClientSecretRegenerateParams,
        "Regenerate the client secret"
    );

    impl_get_tool!(
        client_registration_token_get,
        client_registration_token_get,
        ClientRegistrationTokenGetParams,
        "Get the registration access token for a client"
    );

    impl_get_tool!(
        client_registration_token_regenerate,
        client_registration_token_regenerate,
        ClientRegistrationTokenRegenerateParams,
        "Regenerate the registration access token"
    );

    impl_get_tool!(
        client_certificates_get,
        client_certificates_get,
        ClientCertificatesGetParams,
        "Get key info for a client certificate"
    );

    impl_get_tool!(
        client_certificate_generate,
        client_certificate_generate,
        ClientCertificateGenerateParams,
        "Generate a new certificate for a client"
    );

    // Client Roles
    impl_list_tool!(
        client_roles_list,
        client_roles_list,
        ClientRoleListParams,
        "List roles defined for a client"
    );

    impl_get_tool!(
        client_role_get,
        client_role_get,
        ClientRoleGetParams,
        "Get a client role by name"
    );

    impl_action_tool!(
        client_role_create,
        client_role_create,
        ClientRoleCreateParams,
        "Create a new role for a client",
        "Client role created successfully"
    );

    impl_action_tool!(
        client_role_update,
        client_role_update,
        ClientRoleUpdateParams,
        "Update a client role",
        "Client role updated successfully"
    );

    impl_action_tool!(
        client_role_delete,
        client_role_delete,
        ClientRoleDeleteParams,
        "Delete a client role",
        "Client role deleted successfully"
    );

    impl_list_tool!(
        client_role_users_list,
        client_role_users_list,
        ClientRoleUsersListParams,
        "List users that have a specific client role"
    );

    impl_list_tool!(
        client_role_groups_list,
        client_role_groups_list,
        ClientRoleGroupsListParams,
        "List groups that have a specific client role"
    );

    // Client Scope Mappings
    impl_list_tool!(
        client_scope_mappings_realm_list,
        client_scope_mappings_realm_list,
        ClientScopeMappingsRealmListParams,
        "List realm-level scope mappings for a client"
    );

    impl_action_tool!(
        client_scope_mappings_realm_add,
        client_scope_mappings_realm_add,
        ClientScopeMappingsRealmAddParams,
        "Add realm-level scope mappings to a client",
        "Realm scope mappings added successfully"
    );

    impl_action_tool!(
        client_scope_mappings_realm_remove,
        client_scope_mappings_realm_remove,
        ClientScopeMappingsRealmRemoveParams,
        "Remove realm-level scope mappings from a client",
        "Realm scope mappings removed successfully"
    );

    impl_list_tool!(
        client_scope_mappings_client_list,
        client_scope_mappings_client_list,
        ClientScopeMappingsClientListParams,
        "List client-level scope mappings for a client"
    );

    impl_action_tool!(
        client_scope_mappings_client_add,
        client_scope_mappings_client_add,
        ClientScopeMappingsClientAddParams,
        "Add client-level scope mappings to a client",
        "Client scope mappings added successfully"
    );

    impl_action_tool!(
        client_scope_mappings_client_remove,
        client_scope_mappings_client_remove,
        ClientScopeMappingsClientRemoveParams,
        "Remove client-level scope mappings from a client",
        "Client scope mappings removed successfully"
    );

    // Client Protocol Mappers
    impl_list_tool!(
        client_protocol_mappers_list,
        client_protocol_mappers_list,
        ClientProtocolMappersListParams,
        "List protocol mappers for a client"
    );

    impl_get_tool!(
        client_protocol_mapper_get,
        client_protocol_mapper_get,
        ClientProtocolMapperGetParams,
        "Get a protocol mapper for a client"
    );

    impl_action_tool!(
        client_protocol_mapper_create,
        client_protocol_mapper_create,
        ClientProtocolMapperCreateParams,
        "Create a protocol mapper for a client",
        "Protocol mapper created successfully"
    );

    impl_action_tool!(
        client_protocol_mapper_update,
        client_protocol_mapper_update,
        ClientProtocolMapperUpdateParams,
        "Update a protocol mapper for a client",
        "Protocol mapper updated successfully"
    );

    impl_action_tool!(
        client_protocol_mapper_delete,
        client_protocol_mapper_delete,
        ClientProtocolMapperDeleteParams,
        "Delete a protocol mapper from a client",
        "Protocol mapper deleted successfully"
    );

    // Client Service Account
    impl_get_tool!(
        client_service_account_user_get,
        client_service_account_user_get,
        ClientServiceAccountUserGetParams,
        "Get the service account user for a client"
    );

    // ============================================================================
    // ROLES API
    // ============================================================================

    impl_list_tool!(
        realm_role_list,
        realm_role_list,
        RoleListParams,
        "List realm-level roles"
    );

    impl_get_tool!(
        realm_role_get,
        realm_role_get,
        RoleGetParams,
        "Get a realm role by name"
    );

    impl_action_tool!(
        realm_role_create,
        realm_role_create,
        RoleCreateParams,
        "Create a new realm role",
        "Realm role created successfully"
    );

    impl_action_tool!(
        realm_role_update,
        realm_role_update,
        RoleUpdateParams,
        "Update a realm role",
        "Realm role updated successfully"
    );

    impl_action_tool!(
        realm_role_delete,
        realm_role_delete,
        RoleDeleteParams,
        "Delete a realm role",
        "Realm role deleted successfully"
    );

    // Role Composites
    impl_list_tool!(
        realm_role_composites_list,
        realm_role_composites_list,
        RoleCompositesListParams,
        "List composite roles for a realm role"
    );

    impl_action_tool!(
        realm_role_composites_add,
        realm_role_composites_add,
        RoleCompositesAddParams,
        "Add composite roles to a realm role",
        "Composite roles added successfully"
    );

    impl_action_tool!(
        realm_role_composites_remove,
        realm_role_composites_remove,
        RoleCompositesRemoveParams,
        "Remove composite roles from a realm role",
        "Composite roles removed successfully"
    );

    impl_list_tool!(
        realm_role_users_list,
        realm_role_users_list,
        RoleUsersListParams,
        "List users that have a specific realm role"
    );

    impl_list_tool!(
        realm_role_groups_list,
        realm_role_groups_list,
        RoleGroupsListParams,
        "List groups that have a specific realm role"
    );

    // Role By ID
    impl_get_tool!(
        role_by_id_get,
        role_by_id_get,
        RoleByIdGetParams,
        "Get a role by its ID"
    );

    impl_action_tool!(
        role_by_id_update,
        role_by_id_update,
        RoleByIdUpdateParams,
        "Update a role by its ID",
        "Role updated successfully"
    );

    impl_action_tool!(
        role_by_id_delete,
        role_by_id_delete,
        RoleByIdDeleteParams,
        "Delete a role by its ID",
        "Role deleted successfully"
    );

    impl_list_tool!(
        role_by_id_composites_list,
        role_by_id_composites_list,
        RoleByIdCompositesListParams,
        "List composite roles for a role (by ID)"
    );

    impl_action_tool!(
        role_by_id_composites_add,
        role_by_id_composites_add,
        RoleByIdCompositesAddParams,
        "Add composite roles to a role (by ID)",
        "Composite roles added successfully"
    );

    impl_action_tool!(
        role_by_id_composites_remove,
        role_by_id_composites_remove,
        RoleByIdCompositesRemoveParams,
        "Remove composite roles from a role (by ID)",
        "Composite roles removed successfully"
    );

    // ============================================================================
    // GROUPS API
    // ============================================================================

    impl_list_tool!(
        group_list,
        group_list,
        GroupListParams,
        "List groups in a realm with optional filters"
    );

    impl_get_tool!(group_get, group_get, GroupGetParams, "Get a group by ID");

    impl_action_tool!(
        group_create,
        group_create,
        GroupCreateParams,
        "Create a new top-level group in a realm",
        "Group created successfully"
    );

    impl_action_tool!(
        group_update,
        group_update,
        GroupUpdateParams,
        "Update an existing group",
        "Group updated successfully"
    );

    impl_action_tool!(
        group_delete,
        group_delete,
        GroupDeleteParams,
        "Delete a group from a realm",
        "Group deleted successfully"
    );

    impl_get_tool!(
        group_count,
        group_count,
        GroupCountParams,
        "Get the count of groups in a realm"
    );

    impl_list_tool!(
        group_members_list,
        group_members_list,
        GroupMembersListParams,
        "List members of a group"
    );

    impl_list_tool!(
        group_children_list,
        group_children_list,
        GroupChildrenListParams,
        "List child groups of a group"
    );

    impl_action_tool!(
        group_child_create,
        group_child_create,
        GroupChildCreateParams,
        "Create a child group under a parent group",
        "Child group created successfully"
    );

    impl_action_tool!(
        group_set_parent,
        group_set_parent,
        GroupSetParentParams,
        "Move a group under a new parent",
        "Group parent set successfully"
    );

    // Group Role Mappings
    impl_list_tool!(
        group_realm_roles_list,
        group_realm_roles_list,
        GroupRealmRolesListParams,
        "List realm-level roles assigned to a group"
    );

    impl_action_tool!(
        group_realm_roles_add,
        group_realm_roles_add,
        GroupRealmRolesAddParams,
        "Add realm-level roles to a group",
        "Realm roles added to group successfully"
    );

    impl_action_tool!(
        group_realm_roles_remove,
        group_realm_roles_remove,
        GroupRealmRolesRemoveParams,
        "Remove realm-level roles from a group",
        "Realm roles removed from group successfully"
    );

    impl_list_tool!(
        group_realm_roles_available,
        group_realm_roles_available,
        GroupRealmRolesAvailableParams,
        "List available realm-level roles that can be assigned to a group"
    );

    impl_list_tool!(
        group_client_roles_list,
        group_client_roles_list,
        GroupClientRolesListParams,
        "List client-level roles assigned to a group"
    );

    impl_action_tool!(
        group_client_roles_add,
        group_client_roles_add,
        GroupClientRolesAddParams,
        "Add client-level roles to a group",
        "Client roles added to group successfully"
    );

    impl_action_tool!(
        group_client_roles_remove,
        group_client_roles_remove,
        GroupClientRolesRemoveParams,
        "Remove client-level roles from a group",
        "Client roles removed from group successfully"
    );

    // ============================================================================
    // REALMS API
    // ============================================================================

    impl_list_tool!(realm_list, realm_list, RealmListParams, "List all realms");

    impl_get_tool!(realm_get, realm_get, RealmGetParams, "Get a realm by name");

    impl_action_tool!(
        realm_create,
        realm_create,
        RealmCreateParams,
        "Create a new realm",
        "Realm created successfully"
    );

    impl_action_tool!(
        realm_update,
        realm_update,
        RealmUpdateParams,
        "Update an existing realm",
        "Realm updated successfully"
    );

    impl_action_tool!(
        realm_delete,
        realm_delete,
        RealmDeleteParams,
        "Delete a realm",
        "Realm deleted successfully"
    );

    // Realm Keys
    impl_list_tool!(
        realm_keys_list,
        realm_keys_list,
        RealmKeysListParams,
        "List cryptographic keys for a realm"
    );

    // Realm Events
    impl_list_tool!(
        realm_events_list,
        realm_events_list,
        RealmEventsListParams,
        "List events for a realm with optional filters"
    );

    impl_action_tool!(
        realm_events_delete,
        realm_events_delete,
        RealmEventsDeleteParams,
        "Delete all events for a realm",
        "Events deleted successfully"
    );

    impl_get_tool!(
        realm_events_config_get,
        realm_events_config_get,
        RealmEventsConfigGetParams,
        "Get events configuration for a realm"
    );

    impl_action_tool!(
        realm_events_config_update,
        realm_events_config_update,
        RealmEventsConfigUpdateParams,
        "Update events configuration for a realm",
        "Events configuration updated successfully"
    );

    impl_list_tool!(
        realm_admin_events_list,
        realm_admin_events_list,
        RealmAdminEventsListParams,
        "List admin events for a realm with optional filters"
    );

    impl_action_tool!(
        realm_admin_events_delete,
        realm_admin_events_delete,
        RealmAdminEventsDeleteParams,
        "Delete all admin events for a realm",
        "Admin events deleted successfully"
    );

    // Realm Sessions
    impl_list_tool!(
        realm_sessions_list,
        realm_sessions_list,
        RealmSessionsListParams,
        "List active sessions in a realm"
    );

    impl_action_tool!(
        realm_logout_all,
        realm_logout_all,
        RealmLogoutAllParams,
        "Log out all users from a realm",
        "All users logged out successfully"
    );

    impl_action_tool!(
        realm_push_revocation,
        realm_push_revocation,
        RealmPushRevocationParams,
        "Push a not-before revocation policy to all clients",
        "Revocation pushed successfully"
    );

    // Realm Defaults
    impl_list_tool!(
        realm_default_groups_list,
        realm_default_groups_list,
        RealmDefaultGroupsListParams,
        "List default groups for a realm"
    );

    impl_action_tool!(
        realm_default_group_add,
        realm_default_group_add,
        RealmDefaultGroupAddParams,
        "Add a group to the realm's default groups",
        "Default group added successfully"
    );

    impl_action_tool!(
        realm_default_group_remove,
        realm_default_group_remove,
        RealmDefaultGroupRemoveParams,
        "Remove a group from the realm's default groups",
        "Default group removed successfully"
    );

    impl_list_tool!(
        realm_default_client_scopes_list,
        realm_default_client_scopes_list,
        RealmDefaultClientScopesListParams,
        "List default client scopes for a realm"
    );

    impl_list_tool!(
        realm_default_optional_scopes_list,
        realm_default_optional_scopes_list,
        RealmDefaultOptionalScopesListParams,
        "List optional client scopes for a realm"
    );

    // ============================================================================
    // AUTHENTICATION API
    // ============================================================================

    impl_list_tool!(
        auth_flows_list,
        auth_flows_list,
        AuthFlowsListParams,
        "List authentication flows in a realm"
    );

    impl_get_tool!(
        auth_flow_get,
        auth_flow_get,
        AuthFlowGetParams,
        "Get an authentication flow by ID"
    );

    impl_action_tool!(
        auth_flow_create,
        auth_flow_create,
        AuthFlowCreateParams,
        "Create a new authentication flow",
        "Authentication flow created successfully"
    );

    impl_action_tool!(
        auth_flow_copy,
        auth_flow_copy,
        AuthFlowCopyParams,
        "Copy an authentication flow",
        "Authentication flow copied successfully"
    );

    impl_action_tool!(
        auth_flow_delete,
        auth_flow_delete,
        AuthFlowDeleteParams,
        "Delete an authentication flow",
        "Authentication flow deleted successfully"
    );

    impl_list_tool!(
        auth_flow_executions_list,
        auth_flow_executions_list,
        AuthFlowExecutionsListParams,
        "List executions in an authentication flow"
    );

    // Authentication Executions
    impl_get_tool!(
        auth_execution_get,
        auth_execution_get,
        AuthExecutionGetParams,
        "Get an authentication execution by ID"
    );

    impl_action_tool!(
        auth_execution_create,
        auth_execution_create,
        AuthExecutionCreateParams,
        "Create a new execution in an authentication flow",
        "Authentication execution created successfully"
    );

    impl_action_tool!(
        auth_execution_delete,
        auth_execution_delete,
        AuthExecutionDeleteParams,
        "Delete an authentication execution",
        "Authentication execution deleted successfully"
    );

    impl_action_tool!(
        auth_execution_raise_priority,
        auth_execution_raise_priority,
        AuthExecutionPriorityParams,
        "Raise the priority of an authentication execution",
        "Execution priority raised successfully"
    );

    impl_action_tool!(
        auth_execution_lower_priority,
        auth_execution_lower_priority,
        AuthExecutionPriorityParams,
        "Lower the priority of an authentication execution",
        "Execution priority lowered successfully"
    );

    impl_get_tool!(
        auth_execution_config_get,
        auth_execution_config_get,
        AuthExecutionConfigGetParams,
        "Get configuration for an authentication execution"
    );

    impl_action_tool!(
        auth_execution_config_update,
        auth_execution_config_update,
        AuthExecutionConfigUpdateParams,
        "Update configuration for an authentication execution",
        "Execution configuration updated successfully"
    );

    // Required Actions
    impl_list_tool!(
        auth_required_actions_list,
        auth_required_actions_list,
        AuthRequiredActionsListParams,
        "List required actions in a realm"
    );

    impl_get_tool!(
        auth_required_action_get,
        auth_required_action_get,
        AuthRequiredActionGetParams,
        "Get a required action by alias"
    );

    impl_action_tool!(
        auth_required_action_update,
        auth_required_action_update,
        AuthRequiredActionUpdateParams,
        "Update a required action",
        "Required action updated successfully"
    );

    impl_action_tool!(
        auth_required_action_raise_priority,
        auth_required_action_raise_priority,
        AuthRequiredActionRaisePriorityParams,
        "Raise the priority of a required action",
        "Required action priority raised successfully"
    );

    impl_action_tool!(
        auth_required_action_lower_priority,
        auth_required_action_lower_priority,
        AuthRequiredActionLowerPriorityParams,
        "Lower the priority of a required action",
        "Required action priority lowered successfully"
    );

    impl_list_tool!(
        auth_unregistered_required_actions_list,
        auth_unregistered_required_actions_list,
        AuthUnregisteredRequiredActionsListParams,
        "List unregistered required actions in a realm"
    );

    impl_action_tool!(
        auth_required_action_register,
        auth_required_action_register,
        AuthRequiredActionRegisterParams,
        "Register a new required action",
        "Required action registered successfully"
    );

    // ============================================================================
    // IDENTITY PROVIDERS API
    // ============================================================================

    impl_list_tool!(
        idp_list,
        idp_list,
        IdpListParams,
        "List identity providers in a realm"
    );

    impl_get_tool!(
        idp_get,
        idp_get,
        IdpGetParams,
        "Get an identity provider by alias"
    );

    impl_action_tool!(
        idp_create,
        idp_create,
        IdpCreateParams,
        "Create a new identity provider",
        "Identity provider created successfully"
    );

    impl_action_tool!(
        idp_update,
        idp_update,
        IdpUpdateParams,
        "Update an identity provider",
        "Identity provider updated successfully"
    );

    impl_action_tool!(
        idp_delete,
        idp_delete,
        IdpDeleteParams,
        "Delete an identity provider",
        "Identity provider deleted successfully"
    );

    impl_list_tool!(
        idp_providers_list,
        idp_providers_list,
        IdpProvidersGetParams,
        "List available identity provider types"
    );

    // Identity Provider Mappers
    impl_list_tool!(
        idp_mappers_list,
        idp_mappers_list,
        IdpMappersListParams,
        "List mappers for an identity provider"
    );

    impl_get_tool!(
        idp_mapper_get,
        idp_mapper_get,
        IdpMapperGetParams,
        "Get an identity provider mapper by ID"
    );

    impl_action_tool!(
        idp_mapper_create,
        idp_mapper_create,
        IdpMapperCreateParams,
        "Create a mapper for an identity provider",
        "Identity provider mapper created successfully"
    );

    impl_action_tool!(
        idp_mapper_update,
        idp_mapper_update,
        IdpMapperUpdateParams,
        "Update an identity provider mapper",
        "Identity provider mapper updated successfully"
    );

    impl_action_tool!(
        idp_mapper_delete,
        idp_mapper_delete,
        IdpMapperDeleteParams,
        "Delete an identity provider mapper",
        "Identity provider mapper deleted successfully"
    );

    impl_list_tool!(
        idp_mapper_types_list,
        idp_mapper_types_list,
        IdpMapperTypesListParams,
        "List available mapper types for an identity provider"
    );

    // ============================================================================
    // CLIENT SCOPES API
    // ============================================================================

    impl_list_tool!(
        client_scope_list,
        client_scope_list,
        ClientScopeListParams,
        "List client scopes in a realm"
    );

    impl_get_tool!(
        client_scope_get,
        client_scope_get,
        ClientScopeGetParams,
        "Get a client scope by ID"
    );

    impl_action_tool!(
        client_scope_create,
        client_scope_create,
        ClientScopeCreateParams,
        "Create a new client scope",
        "Client scope created successfully"
    );

    impl_action_tool!(
        client_scope_update,
        client_scope_update,
        ClientScopeUpdateParams,
        "Update a client scope",
        "Client scope updated successfully"
    );

    impl_action_tool!(
        client_scope_delete,
        client_scope_delete,
        ClientScopeDeleteParams,
        "Delete a client scope",
        "Client scope deleted successfully"
    );

    // Client Scope Protocol Mappers
    impl_list_tool!(
        client_scope_protocol_mappers_list,
        client_scope_protocol_mappers_list,
        ClientScopeProtocolMappersListParams,
        "List protocol mappers for a client scope"
    );

    impl_get_tool!(
        client_scope_protocol_mapper_get,
        client_scope_protocol_mapper_get,
        ClientScopeProtocolMapperGetParams,
        "Get a protocol mapper for a client scope"
    );

    impl_action_tool!(
        client_scope_protocol_mapper_create,
        client_scope_protocol_mapper_create,
        ClientScopeProtocolMapperCreateParams,
        "Create a protocol mapper for a client scope",
        "Protocol mapper created successfully"
    );

    impl_action_tool!(
        client_scope_protocol_mapper_update,
        client_scope_protocol_mapper_update,
        ClientScopeProtocolMapperUpdateParams,
        "Update a protocol mapper for a client scope",
        "Protocol mapper updated successfully"
    );

    impl_action_tool!(
        client_scope_protocol_mapper_delete,
        client_scope_protocol_mapper_delete,
        ClientScopeProtocolMapperDeleteParams,
        "Delete a protocol mapper from a client scope",
        "Protocol mapper deleted successfully"
    );

    // Client Scope Mappings
    impl_list_tool!(
        client_scope_scope_mappings_realm_list,
        client_scope_scope_mappings_realm_list,
        ClientScopeScopeMappingsRealmListParams,
        "List realm-level scope mappings for a client scope"
    );

    impl_action_tool!(
        client_scope_scope_mappings_realm_add,
        client_scope_scope_mappings_realm_add,
        ClientScopeScopeMappingsRealmAddParams,
        "Add realm-level scope mappings to a client scope",
        "Scope mappings added successfully"
    );

    // ============================================================================
    // AUTHORIZATION API
    // ============================================================================

    impl_get_tool!(
        authz_resource_server_get,
        authz_resource_server_get,
        ResourceServerGetParams,
        "Get the authorization resource server settings for a client"
    );

    impl_action_tool!(
        authz_resource_server_update,
        authz_resource_server_update,
        ResourceServerUpdateParams,
        "Update the authorization resource server settings",
        "Resource server settings updated successfully"
    );

    // Resources
    impl_list_tool!(
        authz_resources_list,
        authz_resources_list,
        ResourceListParams,
        "List authorization resources for a client"
    );

    impl_get_tool!(
        authz_resource_get,
        authz_resource_get,
        ResourceGetParams,
        "Get an authorization resource by ID"
    );

    impl_get_tool!(
        authz_resource_create,
        authz_resource_create,
        ResourceCreateParams,
        "Create an authorization resource"
    );

    impl_action_tool!(
        authz_resource_update,
        authz_resource_update,
        ResourceUpdateParams,
        "Update an authorization resource",
        "Resource updated successfully"
    );

    impl_action_tool!(
        authz_resource_delete,
        authz_resource_delete,
        ResourceDeleteParams,
        "Delete an authorization resource",
        "Resource deleted successfully"
    );

    // Scopes
    impl_list_tool!(
        authz_scopes_list,
        authz_scopes_list,
        ScopeListParams,
        "List authorization scopes for a client"
    );

    impl_get_tool!(
        authz_scope_create,
        authz_scope_create,
        ScopeCreateParams,
        "Create an authorization scope"
    );

    impl_action_tool!(
        authz_scope_delete,
        authz_scope_delete,
        ScopeDeleteParams,
        "Delete an authorization scope",
        "Scope deleted successfully"
    );

    // Policies
    impl_list_tool!(
        authz_policies_list,
        authz_policies_list,
        PolicyListParams,
        "List authorization policies for a client"
    );

    impl_get_tool!(
        authz_policy_get,
        authz_policy_get,
        PolicyGetParams,
        "Get an authorization policy by ID"
    );

    impl_get_tool!(
        authz_policy_create,
        authz_policy_create,
        PolicyCreateParams,
        "Create an authorization policy"
    );

    impl_action_tool!(
        authz_policy_update,
        authz_policy_update,
        PolicyUpdateParams,
        "Update an authorization policy",
        "Policy updated successfully"
    );

    impl_action_tool!(
        authz_policy_delete,
        authz_policy_delete,
        PolicyDeleteParams,
        "Delete an authorization policy",
        "Policy deleted successfully"
    );

    // Permissions
    impl_list_tool!(
        authz_permissions_list,
        authz_permissions_list,
        PermissionListParams,
        "List authorization permissions for a client"
    );

    impl_get_tool!(
        authz_permission_get,
        authz_permission_get,
        PermissionGetParams,
        "Get an authorization permission by ID"
    );

    impl_get_tool!(
        authz_permission_create,
        authz_permission_create,
        PermissionCreateParams,
        "Create an authorization permission"
    );

    impl_action_tool!(
        authz_permission_update,
        authz_permission_update,
        PermissionUpdateParams,
        "Update an authorization permission",
        "Permission updated successfully"
    );

    impl_action_tool!(
        authz_permission_delete,
        authz_permission_delete,
        PermissionDeleteParams,
        "Delete an authorization permission",
        "Permission deleted successfully"
    );

    // Evaluate
    impl_get_tool!(
        authz_evaluate,
        authz_evaluate,
        PolicyEvaluateParams,
        "Evaluate authorization policies for a client"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_handler_creation() {
        let client = KeycloakClient::new("http://localhost:8080").unwrap();
        let handler = KeycloakToolHandler::new(client, "test-token".to_string());
        let _router = handler.router();
    }

    #[tokio::test]
    async fn test_set_token() {
        let client = KeycloakClient::new("http://localhost:8080").unwrap();
        let handler = KeycloakToolHandler::new(client, "initial-token".to_string());

        assert_eq!(handler.get_token().await, "initial-token");

        handler.set_token("updated-token".to_string()).await;
        assert_eq!(handler.get_token().await, "updated-token");
    }
}
