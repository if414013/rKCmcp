# Identity Providers API Reference

The Identity Providers API manages external identity providers (IDPs) and their associated mappers for identity federation.

## Identity Provider Management

| Tool Name | Description | Key Parameters |
|-----------|-------------|----------------|
| `idp_list` | List all identity providers in a realm. | `realm` |
| `idp_get` | Get a specific identity provider configuration. | `realm`, `alias` |
| `idp_create` | Create a new identity provider. | `realm`, `idp` (IdentityProviderRepresentation) |
| `idp_update` | Update an existing identity provider. | `realm`, `alias`, `idp` |
| `idp_delete` | Delete an identity provider. | `realm`, `alias` |
| `idp_providers_list` | List available IDP types (e.g., github, google, saml). | `realm`, `providerId` |

## IDP Mapper Management

| Tool Name | Description | Key Parameters |
|-----------|-------------|----------------|
| `idp_mappers_list` | List mappers for a specific identity provider. | `realm`, `alias` |
| `idp_mapper_get` | Get a specific IDP mapper by ID. | `realm`, `alias`, `mapperId` |
| `idp_mapper_create` | Create a new mapper for an IDP. | `realm`, `alias`, `mapper` |
| `idp_mapper_update` | Update an existing IDP mapper. | `realm`, `alias`, `mapperId`, `mapper` |
| `idp_mapper_delete` | Delete an IDP mapper. | `realm`, `alias`, `mapperId` |
| `idp_mapper_types_list` | List available mapper types for an IDP. | `realm`, `alias` |

## Data Types

### IdentityProviderRepresentation
Represents an external IDP.
- `alias` (string): Unique alias for the provider.
- `displayName` (string): User-friendly name.
- `providerId` (string): ID of the provider type (e.g., `oidc`, `saml`).
- `enabled` (boolean): Whether the provider is active.
- `config` (Map<string, string>): Provider-specific configuration (client ID, secrets, etc.).

## Example Prompts

### Adding a Google IDP
"Set up a Google identity provider in the 'external' realm with the provided client ID and secret."

### Configuring Mappers
"Create a mapper for the 'github' IDP that maps the 'email' claim to the user's email attribute in the 'dev' realm."

### Listing Mappers
"Show me all configured mappers for the 'AzureAD' identity provider in the 'enterprise' realm."

## Additional Resources
For detailed information on configuring specific providers and mapper types, refer to the [Keycloak Administration API documentation](https://www.keycloak.org/docs-api/latest/rest-api/index.html).
