# Client Scopes API Reference

The Client Scopes API allows for managing common sets of protocol mappers and role scope mappings that can be shared across multiple clients.

## Client Scope Management

| Tool Name | Description | Key Parameters |
|-----------|-------------|----------------|
| `client_scope_list` | List all client scopes in a realm. | `realm` |
| `client_scope_get` | Get a specific client scope by ID. | `realm`, `clientScopeId` |
| `client_scope_create` | Create a new client scope. | `realm`, `clientScope` |
| `client_scope_update` | Update an existing client scope. | `realm`, `clientScopeId`, `clientScope` |
| `client_scope_delete` | Delete a client scope. | `realm`, `clientScopeId` |

## Protocol Mapper Tools

| Tool Name | Description | Key Parameters |
|-----------|-------------|----------------|
| `client_scope_protocol_mappers_list` | List protocol mappers for a client scope. | `realm`, `clientScopeId` |
| `client_scope_protocol_mapper_get` | Get a specific protocol mapper by ID. | `realm`, `clientScopeId`, `mapperId` |
| `client_scope_protocol_mapper_create` | Create a new protocol mapper. | `realm`, `clientScopeId`, `mapper` |
| `client_scope_protocol_mapper_update` | Update an existing protocol mapper. | `realm`, `clientScopeId`, `mapperId`, `mapper` |
| `client_scope_protocol_mapper_delete` | Delete a protocol mapper. | `realm`, `clientScopeId`, `mapperId` |

## Scope Mapping Tools

| Tool Name | Description | Key Parameters |
|-----------|-------------|----------------|
| `client_scope_scope_mappings_realm_list` | List realm roles associated with a scope. | `realm`, `clientScopeId` |
| `client_scope_scope_mappings_realm_add` | Add realm roles to a scope mapping. | `realm`, `clientScopeId`, `roles` |

## Data Types

### ClientScopeRepresentation
Represents a client scope.
- `id` (string): Unique identifier.
- `name` (string): Name of the scope.
- `description` (string): Description.
- `protocol` (string): Protocol (e.g., `openid-connect`, `saml`).
- `attributes` (Map<string, string>): Scope attributes.

## Example Prompts

### Creating a Common Scope
"Create a client scope named 'api-access' with description 'General API access scope' in the 'main' realm."

### Managing Protocol Mappers
"Add a 'Hardcoded Claim' protocol mapper to the 'api-access' scope that adds a 'version' claim with value '1.0'."

### Updating Scope Mappings
"Assign the 'user-role' realm role to the 'api-access' client scope mapping."

## Additional Resources
For detailed information on protocol mappers and scope inheritance, refer to the [Keycloak Administration API documentation](https://www.keycloak.org/docs-api/latest/rest-api/index.html).
