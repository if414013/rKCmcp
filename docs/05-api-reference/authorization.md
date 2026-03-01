# Authorization Services API Reference

The Authorization Services API enables fine-grained access control through resources, scopes, policies, and permissions management.

## Resource Server Tools

| Tool Name | Description | Key Parameters |
|-----------|-------------|----------------|
| `authz_resource_server_get` | Get resource server settings for a client. | `realm`, `clientId` |
| `authz_resource_server_update` | Update resource server settings. | `realm`, `clientId`, `settings` |

## Resources Management

| Tool Name | Description | Key Parameters |
|-----------|-------------|----------------|
| `authz_resources_list` | List resources for a client. | `realm`, `clientId`, `name`, `owner`, `type`, `scope`, `first`, `max` |
| `authz_resource_get` | Get a specific resource. | `realm`, `clientId`, `resourceId` |
| `authz_resource_create` | Create a new resource. | `realm`, `clientId`, `resource` |
| `authz_resource_update` | Update an existing resource. | `realm`, `clientId`, `resourceId`, `resource` |
| `authz_resource_delete` | Delete a resource. | `realm`, `clientId`, `resourceId` |

## Scopes Management

| Tool Name | Description | Key Parameters |
|-----------|-------------|----------------|
| `authz_scopes_list` | List authorization scopes for a client. | `realm`, `clientId` |
| `authz_scope_create` | Create a new authorization scope. | `realm`, `clientId`, `scope` |
| `authz_scope_delete` | Delete an authorization scope. | `realm`, `clientId`, `scopeId` |

## Policies and Permissions

| Tool Name | Description | Key Parameters |
|-----------|-------------|----------------|
| `authz_policies_list` | List all policies for a client. | `realm`, `clientId`, `name`, `type`, `resource`, `scope`, `permission` |
| `authz_policy_get` | Get a specific policy. | `realm`, `clientId`, `policyId` |
| `authz_policy_create` | Create a new policy. | `realm`, `clientId`, `policy` |
| `authz_policy_update` | Update an existing policy. | `realm`, `clientId`, `policyId`, `policy` |
| `authz_policy_delete` | Delete a policy. | `realm`, `clientId`, `policyId` |
| `authz_permissions_list` | List all permissions for a client. | `realm`, `clientId` |
| `authz_permission_get` | Get a specific permission. | `realm`, `clientId`, `permissionId` |
| `authz_permission_create` | Create a new permission. | `realm`, `clientId`, `permission` |
| `authz_permission_update` | Update an existing permission. | `realm`, `clientId`, `permissionId`, `permission` |
| `authz_permission_delete` | Delete a permission. | `realm`, `clientId`, `permissionId` |

## Evaluation Tool

| Tool Name | Description | Key Parameters |
|-----------|-------------|----------------|
| `authz_evaluate` | Evaluate permissions for a user and resources. | `realm`, `clientId`, `evaluationRequest` |

## Example Prompts

### Creating a Resource
"Create a resource named 'Invoice API' with type 'urn:my-app:resource:invoice' in the 'billing' client of the 'production' realm."

### Defining a Policy
"Create a 'Role-Based Policy' named 'Finance Only' that requires the 'finance-role' in the 'production' realm."

### Evaluating Permissions
"Evaluate if user 'alice' has access to the 'Invoice API' resource in the 'billing' client."

## Additional Resources
For detailed information on authorization concepts and evaluation logic, refer to the [Keycloak Administration API documentation](https://www.keycloak.org/docs-api/latest/rest-api/index.html).
