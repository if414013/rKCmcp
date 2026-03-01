# Realms API Reference

The Realms API provides administrative control over Keycloak realms, including configuration, events, keys, and session management.

## Realm Management Tools

| Tool Name | Description | Key Parameters |
|-----------|-------------|----------------|
| `realm_list` | List all realms. | None |
| `realm_get` | Get a specific realm configuration. | `realm` |
| `realm_create` | Create a new realm. | `realm` (RealmRepresentation) |
| `realm_update` | Update an existing realm configuration. | `realm`, `realmRepresentation` |
| `realm_delete` | Delete a realm. | `realm` |
| `realm_keys_list` | Get the keys for a realm. | `realm` |

## Events and Auditing

| Tool Name | Description | Key Parameters |
|-----------|-------------|----------------|
| `realm_events_list` | Get login events for the realm. | `realm`, `types`, `client`, `user`, `dateFrom`, `dateTo`, `first`, `max` |
| `realm_events_delete` | Clear login events. | `realm` |
| `realm_events_config_get` | Get event configuration. | `realm` |
| `realm_events_config_update` | Update event configuration. | `realm`, `config` |
| `realm_admin_events_list` | Get admin events for the realm. | `realm`, `operationTypes`, `resourcePath`, `dateFrom`, `dateTo`, `first`, `max` |
| `realm_admin_events_delete` | Clear admin events. | `realm` |

## Sessions and Revocation

| Tool Name | Description | Key Parameters |
|-----------|-------------|----------------|
| `realm_sessions_list` | Get all sessions for a specific client in the realm. | `realm`, `clientId`, `first`, `max` |
| `realm_logout_all` | Logout all sessions in the realm. | `realm` |
| `realm_push_revocation` | Push revocation policy to all clients. | `realm` |

## Default Settings Management

| Tool Name | Description | Key Parameters |
|-----------|-------------|----------------|
| `realm_default_groups_list` | List default groups for new users. | `realm` |
| `realm_default_group_add` | Add a default group for new users. | `realm`, `groupId` |
| `realm_default_group_remove` | Remove a default group. | `realm`, `groupId` |
| `realm_default_client_scopes_list` | List default client scopes for the realm. | `realm` |
| `realm_default_optional_scopes_list` | List default optional client scopes. | `realm` |

## Data Types

### RealmRepresentation
Represents a realm configuration.
- `realm` (string): Name of the realm.
- `enabled` (boolean): Whether the realm is enabled.
- `displayName` (string): Display name for the realm.
- `registrationAllowed` (boolean): Whether user registration is enabled.
- `resetPasswordAllowed` (boolean): Whether password reset is enabled.
- `rememberMe` (boolean): Whether remember me is enabled.

## Example Prompts

### Creating a Realm
"Create a new realm named 'saas-prod' with registration allowed and 'SaaS Production' as the display name."

### Managing Default Groups
"Add the 'Users' group as a default group for all new users in the 'saas-prod' realm."

### Auditing Events
"Show me all 'LOGIN_ERROR' events that occurred in the 'staging' realm since yesterday."

## Additional Resources
For detailed information on realm settings and event types, refer to the [Keycloak Administration API documentation](https://www.keycloak.org/docs-api/latest/rest-api/index.html).
