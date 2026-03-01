# Groups API Reference

The Groups API allows for managing hierarchical groups, group memberships, and role mappings for groups.

## Group Management Tools

| Tool Name | Description | Key Parameters |
|-----------|-------------|----------------|
| `group_list` | List groups in a realm. | `realm`, `search`, `first`, `max` |
| `group_get` | Get a specific group by ID. | `realm`, `groupId` |
| `group_create` | Create a new top-level group. | `realm`, `group` (GroupRepresentation) |
| `group_update` | Update a group's information. | `realm`, `groupId`, `group` |
| `group_delete` | Delete a group. | `realm`, `groupId` |
| `group_count` | Get the count of groups in a realm. | `realm`, `search`, `onlyTopGroups` |
| `group_members_list` | List members of a group. | `realm`, `groupId`, `first`, `max` |
| `group_children_list` | List sub-groups of a group. | `realm`, `groupId` |
| `group_child_create` | Create a sub-group for a group. | `realm`, `groupId`, `group` |
| `group_set_parent` | Set the parent of a group (move group). | `realm`, `groupId`, `parentGroupId` |

## Group Role Mapping Tools

### Realm Roles
| Tool Name | Description | Key Parameters |
|-----------|-------------|----------------|
| `group_realm_roles_list` | List realm roles mapped to a group. | `realm`, `groupId` |
| `group_realm_roles_add` | Add realm roles to a group. | `realm`, `groupId`, `roles` |
| `group_realm_roles_remove` | Remove realm roles from a group. | `realm`, `groupId`, `roles` |
| `group_realm_roles_available` | List realm roles available to be added. | `realm`, `groupId` |

### Client Roles
| Tool Name | Description | Key Parameters |
|-----------|-------------|----------------|
| `group_client_roles_list` | List client roles mapped to a group. | `realm`, `groupId`, `clientId` |
| `group_client_roles_add` | Add client roles to a group. | `realm`, `groupId`, `clientId`, `roles` |
| `group_client_roles_remove` | Remove client roles from a group. | `realm`, `groupId`, `clientId`, `roles` |

## Data Types

### GroupRepresentation
Represents a group in Keycloak.
- `id` (string): Unique identifier.
- `name` (string): Name of the group.
- `path` (string): Full path of the group.
- `attributes` (Map<string, string[]>): Custom attributes.
- `realmRoles` (string[]): List of realm roles.
- `clientRoles` (Map<string, string[]>): Map of client IDs to roles.
- `subGroups` (GroupRepresentation[]): List of sub-groups.

## Example Prompts

### Creating a Hierarchy
"Create a group named 'Engineering' in the 'company' realm, and then create a sub-group under it named 'DevOps'."

### Managing Roles
"Assign the 'developer' realm role to the 'Engineering' group in 'company' realm."

### Finding Members
"List all users who are members of the 'Marketing' group in the 'external' realm."

## Additional Resources
For detailed information on group structures and role inheritance, refer to the [Keycloak Administration API documentation](https://www.keycloak.org/docs-api/latest/rest-api/index.html).
