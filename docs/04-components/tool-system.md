# Tool Registration System

The Tool Registration System is the mechanism by which Keycloak Admin functions are exposed as Model Context Protocol (MCP) tools. It utilizes a declarative macro-based approach to minimize boilerplate and ensure consistency across the hundreds of supported operations.

## Purpose

The system provides a structured way to:
- Map MCP tool calls to Keycloak Admin API endpoints.
- Automatically generate JSON Schema descriptions for tool parameters.
- Route incoming requests to the correct handler functions.
- Standardize error handling and response formatting.

## KeycloakToolHandler and ToolRouter

The `KeycloakToolHandler` manages the lifecycle of all registered tools. Internally, it uses a `ToolRouter` to dispatch requests.

```rust
pub struct KeycloakToolHandler {
    client: KeycloakClient,
    router: ToolRouter,
}
```

The `ToolRouter` acts as a registry, storing mapping between tool names and their execution logic. When `call_tool` is invoked, the router identifies the correct handler, deserializes the arguments into the expected struct, and executes the logic.

## Registration Macros

Three primary macros are used to register different categories of operations. These macros automate the creation of the tool description and the routing logic.

### 1. impl_list_tool!

Used for operations that return an array of items, typically supporting pagination and filtering.

```rust
impl_list_tool!(
    get_users,
    "List users in a realm",
    User,
    "/admin/realms/{realm}/users"
);
```
- **Result**: Returns a `Vec<T>`.
- **Arguments**: Automatically includes `first`, `max`, and search-related parameters.

### 2. impl_get_tool!

Used for operations that retrieve a single resource by its identifier.

```rust
impl_get_tool!(
    get_user,
    "Get a specific user by ID",
    User,
    "/admin/realms/{realm}/users/{id}"
);
```
- **Result**: Returns a single object of type `T`.
- **Arguments**: Extracts required IDs from the parameters.

### 3. impl_action_tool!

Used for mutations: creating, updating, or deleting resources.

```rust
impl_action_tool!(
    create_user,
    "Create a new user",
    (), // No response body usually
    "/admin/realms/{realm}/users",
    Method::POST
);
```
- **Result**: Often returns a success indicator or the created object.
- **Methods**: Supports `POST`, `PUT`, `DELETE`.

## Attribute Macros

For more complex tool logic that doesn't fit the standard templates, attribute macros provide fine-grained control.

### #[tool_router]

Applied to the implementation block of `KeycloakToolHandler`, this macro processes the individual `#[tool]` attributes to build the routing table.

### #[tool(description = "...")]

Applied to individual methods to mark them as MCP tools.

```rust
#[tool(description = "Reset a user's password")]
async fn reset_password(&self, params: ResetPasswordParams) -> Result<ToolResponse, McpError> {
    // Custom logic here
}
```

## Parameters Wrapper

Tool arguments are encapsulated in dedicated structs. The system uses these structs to generate the JSON Schema provided to the LLM.

```rust
#[derive(Deserialize, JsonSchema)]
pub struct GetUserParams {
    pub realm: String,
    pub id: String,
}
```

The use of `JsonSchema` (via the `schemars` crate) ensures that the LLM receives precise information about the types and requirements of every parameter.

## Adding a New Tool

To add a new tool to the system, follow these steps:

1.  **Define Parameters**: Create a struct for the tool's input, deriving `Deserialize` and `JsonSchema`.
2.  **Define Response**: Ensure the response type implements `Serialize`.
3.  **Register with Macro**: Use one of the `impl_*_tool!` macros in `src/mcp/tools.rs` for standard REST operations.
4.  **Implement Custom Logic**: If the tool is complex, add a method to `KeycloakToolHandler` with the `#[tool]` attribute.

### Example: Adding `get_realm_roles`

```rust
// In src/mcp/tools.rs

impl_list_tool!(
    get_realm_roles,
    "Get all roles for a specific realm",
    Role,
    "/admin/realms/{realm}/roles"
);
```

This single line generates the MCP tool metadata, the JSON schema for `realm`, and the logic to call the API and return the results.

## Tool Naming Conventions

The system enforces a consistent naming convention to help LLMs discover and use tools effectively:

- **List operations**: `get_{resources}` (e.g., `get_users`, `get_clients`)
- **Get single item**: `get_{resource}` (e.g., `get_user`, `get_realm`)
- **Creation**: `create_{resource}` (e.g., `create_group`)
- **Update**: `update_{resource}` (e.g., `update_user`)
- **Deletion**: `delete_{resource}` (e.g., `delete_client`)
- **Relationships**: `{action}_{resource}_to_{target}` (e.g., `add_user_to_group`)

## Summary

The tool registration system is designed for scale. By leveraging Rust's powerful macro system, the Keycloak MCP Server can expose the vast surface area of the Keycloak Admin API with minimal code duplication, while maintaining a high degree of type safety and documentation accuracy for the consuming LLM.
