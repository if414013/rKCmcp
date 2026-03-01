# Clients API Reference

The Clients API provides tools for managing Keycloak clients, their roles, secrets, scope mappings, and protocol mappers. Clients are entities that can request Keycloak to authenticate a user.

## Core CRUD Operations

These tools manage the lifecycle of client entities within a realm.

| Tool | Description | Key Parameters |
|------|-------------|----------------|
| client_list | List all clients in a realm with optional filtering. | `realm`, `clientId` (optional), `search` (optional), `first`, `max` |
| client_get | Retrieve a specific client by its internal ID. | `realm`, `id` (internal UUID) |
| client_create | Register a new client in the specified realm. | `realm`, `client` (ClientRepresentation object) |
| client_update | Update an existing client configuration. | `realm`, `id`, `client` (Updated ClientRepresentation) |
| client_delete | Remove a client from the realm. | `realm`, `id` |

### Usage Notes
- Use `client_list` with the `clientId` parameter to find the internal ID (UUID) if you only have the human-readable ID.
- The `id` parameter required by most other tools is the internal UUID, not the `clientId`.

---

## Credentials and Secrets

Tools for managing client authentication credentials, including client secrets and certificates.

| Tool | Description | Key Parameters |
|------|-------------|----------------|
| client_secret_get | Retrieve the current client secret. | `realm`, `id` |
| client_secret_regenerate | Generate a new client secret, invalidating the old one. | `realm`, `id` |
| client_registration_token_get | Get the registration access token for the client. | `realm`, `id` |
| client_registration_token_regenerate | Regenerate the registration access token. | `realm`, `id` |
| client_certificates_get | Get certificate information for a specific attribute. | `realm`, `id`, `attr` |
| client_certificate_generate | Generate a new certificate and key pair. | `realm`, `id`, `attr` |

---

## Client Roles

Manage roles that are specific to a single client (as opposed to realm-wide roles).

| Tool | Description | Key Parameters |
|------|-------------|----------------|
| client_roles_list | List all roles defined for a specific client. | `realm`, `id` |
| client_role_get | Retrieve details of a specific client role by name. | `realm`, `id`, `role_name` |
| client_role_create | Create a new role for the client. | `realm`, `id`, `role` (RoleRepresentation) |
| client_role_update | Update an existing client role. | `realm`, `id`, `role_name`, `role` |
| client_role_delete | Delete a role from the client. | `realm`, `id`, `role_name` |
| client_role_users_list | List users who have been assigned this client role. | `realm`, `id`, `role_name` |
| client_role_groups_list | List groups that have been assigned this client role. | `realm`, `id`, `role_name` |

---

## Scope Mappings

Manage which roles (realm or client) are mapped to a client's scope. This determines which roles can be included in the access token for this client.

| Tool | Description | Key Parameters |
|------|-------------|----------------|
| client_scope_mappings_realm_list | List realm-level roles in the client's scope. | `realm`, `id` |
| client_scope_mappings_realm_add | Add realm-level roles to the client's scope. | `realm`, `id`, `roles` (List of RoleRepresentations) |
| client_scope_mappings_realm_remove | Remove realm-level roles from the client's scope. | `realm`, `id`, `roles` |
| client_scope_mappings_client_list | List roles of a target client in this client's scope. | `realm`, `id`, `client` (target client UUID) |
| client_scope_mappings_client_add | Add roles of a target client to this client's scope. | `realm`, `id`, `client`, `roles` |
| client_scope_mappings_client_remove | Remove roles of a target client from the scope. | `realm`, `id`, `client`, `roles` |

---

## Protocol Mappers

Manage protocol mappers which map user attributes to tokens or assertions.

| Tool | Description | Key Parameters |
|------|-------------|----------------|
| client_protocol_mappers_list | List all protocol mappers for a client. | `realm`, `id` |
| client_protocol_mapper_get | Get a specific protocol mapper by its ID. | `realm`, `id`, `mapper_id` |
| client_protocol_mapper_create | Create a new protocol mapper for the client. | `realm`, `id`, `mapper` (ProtocolMapperRepresentation) |
| client_protocol_mapper_update | Update an existing protocol mapper. | `realm`, `id`, `mapper_id`, `mapper` |
| client_protocol_mapper_delete | Remove a protocol mapper from the client. | `realm`, `id`, `mapper_id` |

---

## Service Account

Manage the service account user associated with a client (when service accounts are enabled).

| Tool | Description | Key Parameters |
|------|-------------|----------------|
| client_service_account_user_get | Get the dedicated service account user for this client. | `realm`, `id` |

---

## ClientRepresentation Object

When creating or updating clients, you provide a `ClientRepresentation`. Key fields include:

- `id`: Internal UUID (read-only on create).
- `clientId`: The human-readable ID used in OIDC/SAML requests.
- `name`: Display name.
- `description`: Text description.
- `enabled`: Boolean, whether the client is active.
- `protocol`: Usually "openid-connect" or "saml".
- `publicClient`: Boolean. If false, it's a confidential client requiring a secret.
- `bearerOnly`: Boolean. For services that only verify tokens.
- `serviceAccountsEnabled`: Boolean. Enables the service account for this client.
- `redirectUris`: List of valid callback URLs.
- `webOrigins`: List of valid origins for CORS.
- `rootUrl`: Root URL of the application.
- `baseUrl`: Base URL of the application.
- `adminUrl`: URL for administration operations.
- `clientAuthenticatorType`: Authenticator type (e.g., "client-secret").
- `secret`: The client secret (write-only).
- `attributes`: Map of string keys and values for specific configurations.
  - `pkce.code.challenge.method`: PKCE configuration.
  - `backchannel.logout.url`: URL for backchannel logout.
  - `use.refresh.tokens`: Enable refresh tokens.

---

## RoleRepresentation Object

Used in client role management and scope mappings.

- `id`: Internal UUID.
- `name`: Role name.
- `description`: Role description.
- `composite`: Boolean, whether this is a composite role.
- `clientRole`: Boolean, true for client roles.
- `containerId`: Internal ID of the client or realm.

---

## ProtocolMapperRepresentation Object

Used to define token mapping logic.

- `id`: Internal UUID.
- `name`: Mapper name.
- `protocol`: Usually "openid-connect".
- `protocolMapper`: The type of mapper (e.g., "oidc-usermodel-property-mapper").
- `consentRequired`: Boolean.
- `config`: Map of configuration settings.
  - `user.attribute`: Property to map.
  - `claim.name`: Name in the token.
  - `jsonType.label`: Data type (String, JSON, etc.).
  - `id.token.claim`: Include in ID token.
  - `access.token.claim`: Include in Access token.

---

## AI Usage Examples

AI agents can use these tools to automate client management tasks.

### Finding and Updating a Client
**Prompt:** "Find the internal ID for the 'frontend-app' client in the 'production' realm and update its valid redirect URIs to include 'https://app.example.com/callback'."

**Logical Steps:**
1. Call `client_list(realm="production", clientId="frontend-app")`.
2. Extract the `id` from the result.
3. Call `client_update(realm="production", id=uuid, client={"redirectUris": ["https://app.example.com/callback"]})`.

### Managing Client Secrets
**Prompt:** "The 'backend-service' client secret has been compromised. Regenerate it in the 'internal' realm and tell me the new secret."

**Logical Steps:**
1. Call `client_list(realm="internal", clientId="backend-service")` to find the UUID.
2. Call `client_secret_regenerate(realm="internal", id=uuid)`.
3. Report the `value` from the response.

### Auditing Client Roles
**Prompt:** "List all roles for the 'reports-api' client in 'finance' realm and show me which users are assigned to the 'admin' role."

**Logical Steps:**
1. Find UUID via `client_list`.
2. Call `client_roles_list(realm="finance", id=uuid)`.
3. Call `client_role_users_list(realm="finance", id=uuid, role_name="admin")`.

### Configuring Protocol Mappers
**Prompt:** "Add a hardcoded claim 'organization' with value 'engineering' to the ID token for the 'gateway' client in the 'master' realm."

**Logical Steps:**
1. Find UUID via `client_list`.
2. Call `client_protocol_mapper_create(realm="master", id=uuid, mapper={...})`.

### Bulk Client Operations
**Prompt:** "Disable all clients in the 'test' realm except for 'admin-cli' and 'account'."

**Logical Steps:**
1. Call `client_list(realm="test")`.
2. Iterate through clients.
3. For each client not in the exclusion list, call `client_update(realm="test", id=id, client={"enabled": false})`.

---

## Implementation Details

### Internal IDs vs Client IDs
Most API calls require the internal UUID (`id`). Always use `client_list` with a `clientId` filter to resolve the UUID before performing operations. The `clientId` is the one configured in the Keycloak UI and used by the application, whereas `id` is a database-level identifier.

### Pagination and Search
When using `client_list`, the results are paginated by default. If you have a large number of clients, use `first` and `max` to iterate through them. Generic search using the `search` parameter is helpful but may return multiple partial matches.

### Error Handling
- **404 Not Found**: The realm or the specific client ID does not exist.
- **403 Forbidden**: The current user lacks administrative permissions for the client or realm.
- **409 Conflict**: Attempting to create a client with a `clientId` that already exists.
- **400 Bad Request**: Invalid `ClientRepresentation` or missing required fields.

### Keycloak Documentation
For detailed information on the properties within `ClientRepresentation`, `RoleRepresentation`, or `ProtocolMapperRepresentation`, refer to the official Keycloak Admin REST API documentation:

[Keycloak Admin REST API Reference](https://www.keycloak.org/docs-api/latest/rest-api/index.html)

---

## Summary Table of Tool Groups

| Group | Primary Purpose | Key Focus |
|-------|-----------------|-----------|
| Core CRUD | Create, Read, Update, Delete clients. | Lifecycle management. |
| Credentials | Manage secrets and keys. | Authentication configuration. |
| Roles | Manage client-specific roles. | Authorization definitions. |
| Scope Mappings | Control role propagation. | Token contents control. |
| Protocol Mappers | Customizing token contents. | Data transformation. |
| Service Account | Accessing client's internal user. | Machine-to-machine identity. |

---

## Detailed Parameter Reference

### Realm-wide Parameters
- `realm`: The name of the realm containing the client.

### Client Identifiers
- `id`: The internal UUID of the client. Essential for GET, UPDATE, DELETE, and sub-resource management.
- `clientId`: The user-defined ID. Used for filtering in `client_list`.

### Pagination Parameters
- `first`: Start index for results.
- `max`: Maximum number of results to return.

### Role Parameters
- `role_name`: The name of the role (e.g., "admin", "viewer").
- `role`: A `RoleRepresentation` object.

### Protocol Mapper Parameters
- `mapper_id`: The internal UUID of the protocol mapper.
- `mapper`: A `ProtocolMapperRepresentation` object.

---

## Advanced Scenarios

### Client Scopes vs Scope Mappings
Note that "Client Scopes" (managed via the Client Scopes API) are reusable sets of mappers and roles. "Scope Mappings" within the Clients API refer to the "Scope" tab in the Keycloak UI for a specific client, defining which roles are allowed to be included in that client's token.

### Certificate Management
When using `client_certificate_generate`, the `attr` parameter refers to the prefix used in the client attributes (e.g., `jwt.credential`). This is used for signed JWT client authentication.

### Service Account User
The service account user is a standard Keycloak user but is linked to the client. You can use User API tools on the ID returned by `client_service_account_user_get` to manage its attributes, group memberships, or realm role mappings. This user typically has a username in the format `service-account-<clientId>`.

### Composite Roles in Clients
Client roles can be composites, meaning they include other roles. When managing these via `client_role_update`, ensure you understand the hierarchy to avoid circular references.

---

## Best Practices
1. **Always Fetch Latest Representation**: Before calling `client_update`, call `client_get` to ensure you have the current state and avoid overwriting concurrent changes.
2. **Minimize Redirect URIs**: For security, keep the list of `redirectUris` as specific as possible. Avoid using wildcards (`*`) in production.
3. **Use Confidential Clients for Servers**: Always set `publicClient: false` for server-side applications and use the Credentials API to manage secrets.
4. **Clean Up**: Delete unused client roles or protocol mappers to keep the configuration maintainable and reduce token size.
5. **Automation Audit**: Regularly use `client_list` to audit which clients are enabled and if their configurations match security policies.

---

## Troubleshooting

### Why is my client update failing?
Ensure you are not trying to change immutable fields like the internal `id`. Check that the `ClientRepresentation` you are sending is valid JSON. Keycloak is strict about the structure of the representation.

### Why can't I find my client?
Check if you are searching by `clientId` (the human name) but using a tool that expects the `id` (the UUID). Use `client_list` with `clientId` filter to resolve.

### Secret Regeneration
Regenerating a secret is instantaneous. Ensure any applications using the old secret are updated immediately to prevent service interruption. There is no "grace period" for the old secret.

### Scope Mapping Issues
If a role is assigned to a user but not appearing in the token, check the `client_scope_mappings`. The client must have the role in its scope (either directly or via "Full Scope Allowed").

### Protocol Mapper Not Working
Verify that the `protocol` is set to `openid-connect` and the `protocolMapper` type is correctly spelled. Check the `config` map for required fields like `claim.name`.
