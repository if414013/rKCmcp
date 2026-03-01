//! Type definitions for Keycloak Authentication Management API.

use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Represents an authentication execution step within a flow.
///
/// Used for exporting/importing authentication flows.
#[derive(Debug, Clone, Serialize, Deserialize, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationExecutionExportRepresentation {
    /// The authenticator provider ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authenticator: Option<String>,

    /// Configuration alias for this execution
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authenticator_config: Option<String>,

    /// Whether authentication is required, optional, alternative, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirement: Option<String>,

    /// Priority order of execution
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,

    /// Whether this is an authentication flow reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_flow: Option<bool>,

    /// Sub-flow alias if this is a flow reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_alias: Option<String>,

    /// Whether the user can configure this execution
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_setup_allowed: Option<bool>,
}

/// Represents an authentication flow in Keycloak.
///
/// Authentication flows define a sequence of authentication steps
/// (e.g., username/password, OTP, browser flow).
#[derive(Debug, Clone, Serialize, Deserialize, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationFlowRepresentation {
    /// The unique identifier for this flow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The alias (name) of this flow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,

    /// Description of the flow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The provider type (e.g., "basic-flow", "client-flow")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_id: Option<String>,

    /// Whether this is a top-level flow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_level: Option<bool>,

    /// Whether this is a built-in flow (cannot be deleted)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub built_in: Option<bool>,

    /// List of authentication executions in this flow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_executions: Option<Vec<AuthenticationExecutionExportRepresentation>>,
}

/// Represents execution information for listing executions in a flow.
///
/// This provides detailed information about each execution step in an authentication flow.
#[derive(Debug, Clone, Serialize, Deserialize, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationExecutionInfoRepresentation {
    /// The unique identifier for this execution
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The requirement type (REQUIRED, ALTERNATIVE, DISABLED, CONDITIONAL)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirement: Option<String>,

    /// Display name for the execution
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// Alias for the execution (if it's a flow)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,

    /// Description of the execution
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Available requirement choices
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirement_choices: Option<Vec<String>>,

    /// Whether this execution can be configured
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configurable: Option<bool>,

    /// The authenticator provider ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_id: Option<String>,

    /// Nesting level in the flow hierarchy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<i32>,

    /// Position index within the current level
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,

    /// Whether this is a sub-flow (not a single execution)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_flow: Option<bool>,

    /// ID of the sub-flow (if this is a flow reference)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_id: Option<String>,
}

/// Request body for copying an authentication flow.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct FlowCopyRequest {
    /// The new name for the copied flow
    pub new_name: String,
}

/// Represents an authentication execution within a flow.
///
/// Used when getting a single execution by ID.
#[derive(Debug, Clone, Serialize, Deserialize, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationExecutionRepresentation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub authenticator: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub authenticator_flow: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirement: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_flow: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub authenticator_config: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_id: Option<String>,
}

/// Represents an authenticator configuration.
///
/// Used to configure authenticator behavior (e.g., OTP settings).
#[derive(Debug, Clone, Serialize, Deserialize, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticatorConfigRepresentation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<HashMap<String, String>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_flow_representation_default() {
        let flow = AuthenticationFlowRepresentation::default();
        assert!(flow.id.is_none());
        assert!(flow.alias.is_none());
        assert!(flow.description.is_none());
        assert!(flow.provider_id.is_none());
        assert!(flow.top_level.is_none());
        assert!(flow.built_in.is_none());
        assert!(flow.authentication_executions.is_none());
    }

    #[test]
    fn test_auth_flow_representation_serialization() {
        let flow = AuthenticationFlowRepresentation {
            id: Some("flow-123".to_string()),
            alias: Some("browser".to_string()),
            description: Some("Browser authentication flow".to_string()),
            provider_id: Some("basic-flow".to_string()),
            top_level: Some(true),
            built_in: Some(true),
            authentication_executions: None,
        };

        let json = serde_json::to_string(&flow).unwrap();
        assert!(json.contains("\"id\":\"flow-123\""));
        assert!(json.contains("\"alias\":\"browser\""));
        assert!(json.contains("\"description\":\"Browser authentication flow\""));
        assert!(json.contains("\"providerId\":\"basic-flow\""));
        assert!(json.contains("\"topLevel\":true"));
        assert!(json.contains("\"builtIn\":true"));
        assert!(!json.contains("authenticationExecutions"));
    }

    #[test]
    fn test_auth_flow_representation_deserialization() {
        let json = r#"{
            "id": "abc-456",
            "alias": "my-flow",
            "description": "Custom flow",
            "providerId": "basic-flow",
            "topLevel": false,
            "builtIn": false,
            "authenticationExecutions": [
                {
                    "authenticator": "auth-username-form",
                    "requirement": "REQUIRED",
                    "priority": 10
                }
            ]
        }"#;

        let flow: AuthenticationFlowRepresentation = serde_json::from_str(json).unwrap();
        assert_eq!(flow.id, Some("abc-456".to_string()));
        assert_eq!(flow.alias, Some("my-flow".to_string()));
        assert_eq!(flow.description, Some("Custom flow".to_string()));
        assert_eq!(flow.provider_id, Some("basic-flow".to_string()));
        assert_eq!(flow.top_level, Some(false));
        assert_eq!(flow.built_in, Some(false));

        let executions = flow.authentication_executions.unwrap();
        assert_eq!(executions.len(), 1);
        assert_eq!(
            executions[0].authenticator,
            Some("auth-username-form".to_string())
        );
        assert_eq!(executions[0].requirement, Some("REQUIRED".to_string()));
        assert_eq!(executions[0].priority, Some(10));
    }

    #[test]
    fn test_execution_info_representation_default() {
        let exec = AuthenticationExecutionInfoRepresentation::default();
        assert!(exec.id.is_none());
        assert!(exec.requirement.is_none());
        assert!(exec.display_name.is_none());
        assert!(exec.configurable.is_none());
        assert!(exec.level.is_none());
        assert!(exec.index.is_none());
    }

    #[test]
    fn test_execution_info_representation_serialization() {
        let exec = AuthenticationExecutionInfoRepresentation {
            id: Some("exec-123".to_string()),
            requirement: Some("REQUIRED".to_string()),
            display_name: Some("Username Password Form".to_string()),
            alias: None,
            description: Some("Validates username/password".to_string()),
            requirement_choices: Some(vec![
                "REQUIRED".to_string(),
                "ALTERNATIVE".to_string(),
                "DISABLED".to_string(),
            ]),
            configurable: Some(false),
            provider_id: Some("auth-username-password-form".to_string()),
            level: Some(0),
            index: Some(0),
            authentication_flow: Some(false),
            flow_id: None,
        };

        let json = serde_json::to_string(&exec).unwrap();
        assert!(json.contains("\"id\":\"exec-123\""));
        assert!(json.contains("\"requirement\":\"REQUIRED\""));
        assert!(json.contains("\"displayName\":\"Username Password Form\""));
        assert!(json.contains("\"requirementChoices\":[\"REQUIRED\",\"ALTERNATIVE\",\"DISABLED\"]"));
        assert!(json.contains("\"configurable\":false"));
        assert!(json.contains("\"level\":0"));
        assert!(json.contains("\"authenticationFlow\":false"));
    }

    #[test]
    fn test_execution_info_representation_deserialization() {
        let json = r#"{
            "id": "exec-456",
            "requirement": "ALTERNATIVE",
            "displayName": "OTP Form",
            "description": "One-time password validation",
            "requirementChoices": ["REQUIRED", "ALTERNATIVE", "DISABLED", "CONDITIONAL"],
            "configurable": true,
            "providerId": "auth-otp-form",
            "level": 1,
            "index": 2,
            "authenticationFlow": false,
            "flowId": null
        }"#;

        let exec: AuthenticationExecutionInfoRepresentation = serde_json::from_str(json).unwrap();
        assert_eq!(exec.id, Some("exec-456".to_string()));
        assert_eq!(exec.requirement, Some("ALTERNATIVE".to_string()));
        assert_eq!(exec.display_name, Some("OTP Form".to_string()));
        assert_eq!(
            exec.description,
            Some("One-time password validation".to_string())
        );
        assert_eq!(exec.configurable, Some(true));
        assert_eq!(exec.provider_id, Some("auth-otp-form".to_string()));
        assert_eq!(exec.level, Some(1));
        assert_eq!(exec.index, Some(2));
        assert_eq!(exec.authentication_flow, Some(false));

        let choices = exec.requirement_choices.unwrap();
        assert_eq!(choices.len(), 4);
        assert!(choices.contains(&"CONDITIONAL".to_string()));
    }

    #[test]
    fn test_execution_export_representation_default() {
        let exec = AuthenticationExecutionExportRepresentation::default();
        assert!(exec.authenticator.is_none());
        assert!(exec.authenticator_config.is_none());
        assert!(exec.requirement.is_none());
        assert!(exec.priority.is_none());
        assert!(exec.authentication_flow.is_none());
        assert!(exec.flow_alias.is_none());
        assert!(exec.user_setup_allowed.is_none());
    }

    #[test]
    fn test_execution_export_representation_serialization() {
        let exec = AuthenticationExecutionExportRepresentation {
            authenticator: Some("auth-cookie".to_string()),
            authenticator_config: None,
            requirement: Some("ALTERNATIVE".to_string()),
            priority: Some(10),
            authentication_flow: Some(false),
            flow_alias: None,
            user_setup_allowed: Some(false),
        };

        let json = serde_json::to_string(&exec).unwrap();
        assert!(json.contains("\"authenticator\":\"auth-cookie\""));
        assert!(json.contains("\"requirement\":\"ALTERNATIVE\""));
        assert!(json.contains("\"priority\":10"));
        assert!(json.contains("\"authenticationFlow\":false"));
        assert!(json.contains("\"userSetupAllowed\":false"));
        assert!(!json.contains("authenticatorConfig"));
        assert!(!json.contains("flowAlias"));
    }

    #[test]
    fn test_flow_copy_request_serialization() {
        let req = FlowCopyRequest {
            new_name: "my-custom-flow".to_string(),
        };

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"newName\":\"my-custom-flow\""));
    }

    #[test]
    fn test_flow_copy_request_deserialization() {
        let json = r#"{"newName": "copied-browser-flow"}"#;
        let req: FlowCopyRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.new_name, "copied-browser-flow");
    }
}
