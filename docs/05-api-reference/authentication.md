# Authentication API Reference

The Authentication API provides tools for managing authentication flows, executions, required actions, and their configurations.

## Authentication Flows

| Tool Name | Description | Key Parameters |
|-----------|-------------|----------------|
| `auth_flows_list` | List all authentication flows in a realm. | `realm` |
| `auth_flow_get` | Get a specific authentication flow by ID. | `realm`, `flowId` |
| `auth_flow_create` | Create a new authentication flow. | `realm`, `flow` (AuthenticationFlowRepresentation) |
| `auth_flow_copy` | Copy an existing flow to a new name. | `realm`, `flowAlias`, `newName` |
| `auth_flow_delete` | Delete an authentication flow. | `realm`, `flowId` |

## Flow Executions

| Tool Name | Description | Key Parameters |
|-----------|-------------|----------------|
| `auth_flow_executions_list` | List executions for a specific flow. | `realm`, `flowAlias` |
| `auth_execution_get` | Get a specific execution by ID. | `realm`, `executionId` |
| `auth_execution_create` | Add an execution to a flow. | `realm`, `flowAlias`, `execution` |
| `auth_execution_delete` | Remove an execution from a flow. | `realm`, `executionId` |
| `auth_execution_raise_priority` | Raise the priority of an execution. | `realm`, `executionId` |
| `auth_execution_lower_priority` | Lower the priority of an execution. | `realm`, `executionId` |

## Execution Configuration

| Tool Name | Description | Key Parameters |
|-----------|-------------|----------------|
| `auth_execution_config_get` | Get configuration for an execution. | `realm`, `configId` |
| `auth_execution_config_update` | Update configuration for an execution. | `realm`, `configId`, `config` |

## Required Actions

| Tool Name | Description | Key Parameters |
|-----------|-------------|----------------|
| `auth_required_actions_list` | List registered required actions. | `realm` |
| `auth_required_action_get` | Get a specific required action. | `realm`, `alias` |
| `auth_required_action_update` | Update a required action configuration. | `realm`, `alias`, `action` |
| `auth_required_action_raise_priority` | Raise priority of a required action. | `realm`, `alias` |
| `auth_required_action_lower_priority` | Lower priority of a required action. | `realm`, `alias` |
| `auth_unregistered_required_actions_list` | List actions not yet registered. | `realm` |
| `auth_required_action_register` | Register an unregistered action. | `realm`, `action` |

## Example Prompts

### Copying a Flow
"Copy the 'browser' flow to a new flow named 'browser-custom' in the 'main' realm."

### Adding an Execution
"Add the 'WebAuthn Authenticator' execution to the 'browser-custom' flow in 'main'."

### Managing Required Actions
"Increase the priority of the 'Terms and Conditions' required action in the 'customer' realm."

## Additional Resources
For detailed information on authentication flows and execution types, refer to the [Keycloak Administration API documentation](https://www.keycloak.org/docs-api/latest/rest-api/index.html).
