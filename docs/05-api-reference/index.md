# API Reference Overview

This section provides a comprehensive guide to the Keycloak Administration tools. These tools are designed to interact with the Keycloak Admin REST API, enabling programmatic management of realms, users, clients, and other security components.

The tools are organized into nine distinct categories, reflecting the primary functional areas of Keycloak administration. Each category encompasses a suite of tools for CRUD (Create, Read, Update, Delete) operations and specialized management tasks.

## API Categories

The following table summarizes the available API categories, the approximate number of tools in each, and their primary focus.

| Category | Tools | Description |
|----------|-------|-------------|
| [Users](./users.md) | 30+ | User management, credentials, groups, roles, sessions |
| [Clients](./clients.md) | 35+ | Client management, secrets, roles, mappers |
| [Roles](./roles.md) | 16+ | Realm roles, composites, role-by-id |
| [Groups](./groups.md) | 17+ | Group hierarchy, members, role mappings |
| [Realms](./realms.md) | 20+ | Realm CRUD, keys, events, sessions, defaults |
| [Authentication](./authentication.md) | 18+ | Flows, executions, required actions |
| [Authorization](./authorization.md) | 22+ | Resources, scopes, policies, permissions |
| [Identity Providers](./identity-providers.md) | 12+ | IdP management, mappers |
| [Client Scopes](./client-scopes.md) | 12+ | Scope management, protocol mappers |

## Tool Naming Conventions

To ensure consistency and discoverability, all tools follow a strict naming convention that indicates the target entity and the action being performed.

### Pattern: {entity}_{action}

This is the standard pattern for operations performed directly on a primary entity.

*   `user_list`: Retrieves a list of users.
*   `client_create`: Registers a new client.
*   `realm_delete`: Removes a specific realm.

### Pattern: {entity}_{sub}_{action}

This pattern is used for operations on sub-resources or relationships between entities.

*   `user_realm_roles_add`: Assigns realm-level roles to a specific user.
*   `client_protocol_mapper_create`: Configures a new protocol mapper for a client.
*   `group_members_get`: Retrieves the members of a group.

## Common Parameters

While specific parameters vary by tool, several common parameters are used across the majority of the API reference.

### Realm Specification

*   **realm**: (Required) The name of the Keycloak realm the tool should operate against. This is mandatory for almost all tools as Keycloak administration is scoped to specific realms.

### Identification Parameters

*   **id**: The unique UUID of the entity being targeted.
*   **user_id**: Specifically used for User entity UUIDs.
*   **client_id**: Specifically used for Client entity UUIDs. Note that this often refers to the internal UUID, not the `clientId` string used by OIDC/SAML.

### Pagination and Filtering

Many list-based tools support pagination to handle large datasets efficiently.

*   **first**: (Optional) The index of the first result to return (starting at 0).
*   **max**: (Optional) The maximum number of results to return in a single call.
*   **search**: (Optional) A string to filter entities by name, username, or email.

## Response Format

All tools provide feedback on the outcome of the operation through a standardized response format.

### Success Responses

When a tool completes successfully, it returns data in the following formats:

*   **JSON Array**: Returned by list tools (e.g., `user_list`). Contains a list of entity objects.
*   **JSON Object**: Returned by retrieval tools (e.g., `user_get`) or creation tools that return the newly created entity.
*   **Empty/Status**: Some operations (like delete) return a success confirmation without a JSON body.

### Error Responses

In the event of a failure (e.g., invalid parameters, authentication issues, or entity not found), the tool will return an error message.

*   **Error Message**: The `content` field will contain a descriptive error message explaining the failure.
*   **HTTP Context**: The error messages often reflect the underlying HTTP status codes from the Keycloak API (400 Bad Request, 401 Unauthorized, 404 Not Found, etc.).

## Authentication

Accessing these tools requires appropriate permissions within the target Keycloak instance. Typically, the caller must have an active session with the `admin` role or specific management roles (e.g., `view-users`, `manage-clients`) for the target realm.

The underlying infrastructure handles the token management and authentication headers, provided the environment is correctly configured with the necessary credentials.

## Further Documentation

This reference focuses on the tool interfaces and their usage within this environment. For detailed descriptions of the underlying data models, field types, and advanced configuration options, please refer to the official Keycloak documentation.

*   [Keycloak Admin REST API Documentation](https://www.keycloak.org/docs-api/latest/rest-api/index.html)

The official API documentation provides exhaustive details on every field available in the request and response objects, including constraints and default values.

## Usage Best Practices

1.  **Use UUIDs**: When targeting specific entities, always use the internal UUID (`id`) rather than human-readable names where possible, as names can change.
2.  **Pagination**: Always use `first` and `max` when retrieving lists in production environments to avoid performance degradation and memory issues.
3.  **Error Handling**: Check the response content for error messages to ensure operations were successful before proceeding with dependent logic.
4.  **Least Privilege**: Ensure the credentials used by the tools have only the necessary roles required for the intended tasks.
