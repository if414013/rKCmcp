# Roles API Reference

The Roles API provides tools for managing realm-level roles, composite roles, and role assignments for users and groups.

## Realm Roles Tools

| Tool Name | Description | Key Parameters |
|-----------|-------------|----------------|
| `realm_role_list` | List all roles for a realm. | `realm` |
| `realm_role_get` | Get a specific role by name. | `realm`, `roleName` |
| `realm_role_create` | Create a new realm role. | `realm`, `role` (RoleRepresentation) |
| `realm_role_update` | Update an existing realm role. | `realm`, `roleName`, `role` |
| `realm_role_delete` | Delete a realm role. | `realm`, `roleName` |
| `realm_role_composites_list` | Get composite roles for a role. | `realm`, `roleName` |
| `realm_role_composites_add` | Add composite roles to a role. | `realm`, `roleName`, `roles` |
| `realm_role_composites_remove` | Remove composite roles from a role. | `realm`, `roleName`, `roles` |
| `realm_role_users_list` | List users that have the given role. | `realm`, `roleName`, `first`, `max` |
| `realm_role_groups_list` | List groups that have the given role. | `realm`, `roleName`, `first`, `max` |

## Role By ID Tools

These tools operate on roles using their unique ID rather than their name.

| Tool Name | Description | Key Parameters |
|-----------|-------------|----------------|
| `role_by_id_get` | Get a role by its unique ID. | `realm`, `roleId` |
| `role_by_id_update` | Update a role by its unique ID. | `realm`, `roleId`, `role` |
| `role_by_id_delete` | Delete a role by its unique ID. | `realm`, `roleId` |
| `role_by_id_composites_list` | Get composite roles for a role ID. | `realm`, `roleId` |
| `role_by_id_composites_add` | Add composite roles to a role ID. | `realm`, `roleId`, `roles` |
| `role_by_id_composites_remove` | Remove composite roles from a role ID. | `realm`, `roleId`, `roles` |

## Data Types

### RoleRepresentation
Represents a role in Keycloak.
- `id` (string): Unique identifier.
- `name` (string): Name of the role.
- `description` (string): Description of the role.
- `composite` (boolean): Whether this is a composite role.
- `clientRole` (boolean): Whether this is a client-level role.
- `containerId` (string): ID of the container (realm or client).

## Example Prompts

### Creating a New Role
"Create a new realm role named 'editor' with the description 'Can edit content' in the 'my-realm' realm."

### Adding Composite Roles
"Add the 'viewer' and 'commenter' roles as composites to the 'editor' role in 'my-realm'."

### Listing Users with a Role
"Show me all users who have the 'admin' role in the 'production' realm."

## Additional Resources
For detailed information on role attributes and behavior, refer to the [Keycloak Administration API documentation](https://www.keycloak.org/docs-api/latest/rest-api/index.html).
