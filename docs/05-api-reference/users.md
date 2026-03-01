# Users API Reference

The Users API provides a comprehensive suite of tools for managing the lifecycle, security, and access control of users within a Keycloak realm. This reference documentation covers operations ranging from basic CRUD to complex role mappings and session management.

## Core CRUD Operations

These tools form the foundation of user management, allowing for the creation, retrieval, modification, and deletion of user records.

| Tool | Description | Key Parameters |
|------|-------------|----------------|
| **user_list** | Searches for users based on criteria such as email, username, or first/last name. Supports pagination via `first` and `max`. | `realm`, `search`, `email`, `username`, `first`, `max` |
| **user_get** | Fetches the full `UserRepresentation` of a specific user. This is typically used to inspect user attributes, group memberships, and account status. | `realm`, `user_id` |
| **user_create** | Registers a new user account. Requires a `UserRepresentation` object which must at least contain a unique username. | `realm`, `user` (UserRepresentation) |
| **user_update** | Updates an existing user's details. Only the fields provided in the `UserRepresentation` will be updated; other fields remain unchanged. | `realm`, `user_id`, `user` |
| **user_delete** | Removes a user and all associated data (roles, groups, credentials) from the realm. This action is irreversible. | `realm`, `user_id` |

### Filtering and Pagination in user_list
When using `user_list`, you can refine your search using the following optional parameters:
- `search`: A global search string applied to username, first name, last name, and email.
- `username`, `email`, `firstName`, `lastName`: Specific field filters.
- `first`: The index of the first result to return (default is 0).
- `max`: The maximum number of results to return (default is 100).

## Credentials Management

Manage user authentication factors, including passwords, OTP devices, and email verification status.

| Tool | Description | Key Parameters |
|------|-------------|----------------|
| **user_reset_password** | Sets a new password for the user. If `temporary` is set to true, the user will be required to change it upon their next login. | `realm`, `user_id`, `password`, `temporary` |
| **user_credentials_list** | Returns a list of all credentials (passwords, OTP, WebAuthn) associated with the user, including their metadata. | `realm`, `user_id` |
| **user_credential_delete** | Deletes a specific credential by its ID. Useful for removing lost OTP devices or old passwords. | `realm`, `user_id`, `credential_id` |
| **user_disable_credentials** | Disables all credentials of the specified types (e.g., "password", "otp") for the user. | `realm`, `user_id`, `credential_types` |
| **user_send_verify_email** | Sends a system-generated email containing a link for the user to verify their email address. | `realm`, `user_id` |
| **user_execute_actions_email** | Triggers specific account actions via email, such as `UPDATE_PASSWORD`, `UPDATE_PROFILE`, or `VERIFY_EMAIL`. | `realm`, `user_id`, `actions` |

## Group Membership

Groups provide a way to organize users and apply bulk role mappings.

| Tool | Description | Key Parameters |
|------|-------------|----------------|
| **user_groups_list** | Retrieves all groups the user is a direct member of. | `realm`, `user_id` |
| **user_group_add** | Associates a user with a group. The user will inherit all roles mapped to that group. | `realm`, `user_id`, `group_id` |
| **user_group_remove** | Dissociates a user from a group, removing any roles inherited from that group. | `realm`, `user_id`, `group_id` |
| **user_group_count** | Returns the total count of groups the user belongs to. Useful for UI badges or summary views. | `realm`, `user_id` |

## Role Mappings

Roles are the primary mechanism for access control. Users can have roles assigned at the realm level or specifically for individual clients.

| Tool | Description | Key Parameters |
|------|-------------|----------------|
| **user_realm_roles_list** | Lists all realm-level roles currently assigned to the user. | `realm`, `user_id` |
| **user_realm_roles_add** | Assigns one or more realm roles to the user. Roles should be passed as a list of role objects. | `realm`, `user_id`, `roles` |
| **user_realm_roles_remove** | Removes assigned realm roles from the user. | `realm`, `user_id`, `roles` |
| **user_realm_roles_available** | Lists all realm roles that exist but are NOT currently assigned to the user. | `realm`, `user_id` |
| **user_client_roles_list** | Lists all roles assigned to the user for a specific client application. | `realm`, `user_id`, `client_id` |
| **user_client_roles_add** | Assigns client-specific roles to the user. | `realm`, `user_id`, `client_id`, `roles` |
| **user_client_roles_remove** | Revokes client-specific roles from the user. | `realm`, `user_id`, `client_id`, `roles` |

## Sessions and Identity

Tools for auditing active logins and managing external identity provider (IdP) links.

| Tool | Description | Key Parameters |
|------|-------------|----------------|
| **user_sessions_list** | Retrieves a list of active browser sessions for the user, including IP address, start time, and client application. | `realm`, `user_id` |
| **user_logout** | Invalidates all active sessions for the user across all clients, forcing a re-login. | `realm`, `user_id` |
| **user_consents_list** | Lists all OAuth2/OpenID Connect consents granted by the user to various clients. | `realm`, `user_id` |
| **user_consent_revoke** | Removes a user's consent for a specific client, requiring them to re-approve permissions on next login. | `realm`, `user_id`, `client_id` |
| **user_federated_identity_list** | Lists all social or SAML/OIDC identity providers linked to the user account (e.g., Google, GitHub). | `realm`, `user_id` |
| **user_federated_identity_add** | Manually links a user to an identity in an external provider. | `realm`, `user_id`, `provider`, `identity` |
| **user_federated_identity_remove** | Removes the link between the user and an external identity provider. | `realm`, `user_id`, `provider` |

## UserRepresentation Schema

The `UserRepresentation` object is used for both input and output. Below are the most commonly used fields:

- **id**: The unique UUID assigned by Keycloak. (Read-only)
- **username**: The login name for the user. (Required)
- **email**: The user's primary email address.
- **firstName**: The user's given name.
- **lastName**: The user's family name.
- **enabled**: Set to `true` to allow the user to log in.
- **emailVerified**: Whether the email address has been confirmed via the verification link.
- **attributes**: A map of custom metadata (e.g., `{"department": ["Sales"], "employeeId": ["12345"]}`).
- **requiredActions**: A list of actions the user must complete upon login (e.g., `["UPDATE_PASSWORD", "CONFIGURE_TOTP"]`).
- **federationLink**: The ID of the storage provider if the user is synced from LDAP or Active Directory.
- **groups**: A list of group names or paths the user should be added to.
- **notBefore**: A timestamp before which tokens issued for this user are invalid.

## AI Usage Examples

The following prompts illustrate how an AI agent can utilize these tools to perform common administrative tasks.

### User Discovery
- "Find all users in the 'internal' realm with an @example.com email address."
- "Show me the details for the user with ID '550e8400-e29b-41d4-a716-446655440000'."
- "Check how many users are currently registered in the 'customer-portal' realm."

### Account Setup
- "Create a new user 'bwayne' in the 'gotham' realm with email 'bruce@wayne.corp' and set his first name to 'Bruce'."
- "Register a new user 'clark.kent' and make sure he is required to change his password on his first login."

### Security and Access
- "Reset the password for user 'joker' to 'P@ssword123' and make it temporary."
- "Send a verification email to 'harley.quinn' so she can confirm her account."
- "Give user 'abc-123' the 'admin' realm role."
- "Remove the 'editor' role from user 'xyz-789' for the 'blog-app' client."

### Maintenance and Auditing
- "List all active sessions for 'lex.luthor' and log him out of all devices."
- "Which groups does user 'diana.prince' belong to?"
- "Unlink the GitHub account from user 'victor.stone' in the 'justice-league' realm."
- "Delete the user account with ID 'dead-beef-1234' from the master realm."

## Additional Resources

For exhaustive details on specific API behaviors, internal logic, or advanced configurations, please consult the:
- [Keycloak Administration REST API Documentation](https://www.keycloak.org/docs-api/latest/rest-api/index.html)
- [Keycloak User Representation Java Docs](https://www.keycloak.org/docs-api/latest/javadocs/org/keycloak/representations/idm/UserRepresentation.html)
