# Configuration Reference

This document provides a comprehensive reference for configuring the Keycloak MCP Server. The server uses environment variables for configuration, which can be provided via a `.env` file or set directly in the shell or container environment.

## Environment Variables

The following table summarizes all available configuration settings.

| Variable | Description | Default | Required |
|----------|-------------|---------|----------|
| `KEYCLOAK_URL` | The base URL where the Keycloak server is accessible. This should include the protocol and port. | `http://localhost:8080` | Yes |
| `KEYCLOAK_REALM` | The target Keycloak realm used for authentication and management operations. | `master` | No |
| `MCP_PORT` | The TCP port on which the Model Context Protocol server will listen for incoming connections. | `3000` | No |
| `LOG_LEVEL` | Controls the verbosity of system logs. Supported values: `trace`, `debug`, `info`, `warn`, `error`. | `info` | No |
| `JWKS_CACHE_TTL` | Duration in seconds to cache JSON Web Key Sets (JWKS) used for token validation. | `3600` | No |

## Configuration Loading Process

The Keycloak MCP Server follows a strict loading and validation sequence during startup to ensure environment consistency:

1.  **Environment Initialization**: At startup, the server attempts to load variables from the system environment.
2.  **Required Variable Check**: The `from_env()` function specifically checks for the presence of `KEYCLOAK_URL`. If this variable is missing, the server will terminate immediately with a descriptive error message.
3.  **Default Assignment**: For optional variables that are not present in the environment, the server assigns pre-defined default values (e.g., port 3000 for the MCP server).
4.  **Type Validation**: Numeric values such as `MCP_PORT` and `JWKS_CACHE_TTL` are parsed into their respective types. If the string value cannot be converted (e.g., "abc" for a port), a validation error is triggered.
5.  **Server Binding**: Once validation is successful, the server binds to `0.0.0.0` using the configured `MCP_PORT`.

### Error Handling

If any configuration requirement is not met, the server provides graceful error messages:
- **Missing Required**: "Missing required environment variable: KEYCLOAK_URL is required. Please set it to your Keycloak server URL (e.g., http://localhost:8080)"
- **Invalid Values**: "Invalid configuration value: MCP_PORT must be a valid port number (0-65535)" or "Invalid configuration value: JWKS_CACHE_TTL must be a valid number of seconds"

## Example .env File

To simplify local development, you can create a `.env` file in the project root with the following content:

```env
KEYCLOAK_URL=http://localhost:8080
KEYCLOAK_REALM=master
MCP_PORT=3000
LOG_LEVEL=info
JWKS_CACHE_TTL=3600
```

!!! warning "Do not commit secrets"
    Ensure this file is not committed to version control if it contains sensitive information in production environments.

## Docker Compose Configuration

The Keycloak MCP Server is designed to work seamlessly within a containerized environment. A typical `docker-compose.yml` setup includes both the Keycloak instance and the MCP server.

### Keycloak Service
- **Image**: Official Keycloak image (e.g., `quay.io/keycloak/keycloak`).
- **Port**: Maps internal port 8080 to the host port 8080.
- **Environment**: Configured with admin credentials and database settings.

### MCP Server Service
- **Image**: Custom image built from the provided `Dockerfile`.
- **Port**: Maps internal port 3000 to the host port 3000.
- **Network**: Both services should reside on the same bridge network to allow the MCP server to communicate with Keycloak using the service name (e.g., `http://keycloak:8080`).

### Network Configuration example

```yaml
services:
  keycloak:
    image: quay.io/keycloak/keycloak:latest
    ports:
      - "8080:8080"
    networks:
      - mcp-network

  mcp-server:
    build: .
    ports:
      - "3000:3000"
    environment:
      - KEYCLOAK_URL=http://keycloak:8080
    networks:
      - mcp-network

networks:
  mcp-network:
    driver: bridge
```

## Configuration Validation Rules

The following rules are applied to configuration values during the initialization phase:

### KEYCLOAK_URL
- Must be a non-empty string.
- Should be a valid URL including the scheme (http or https).
- Validation fails if the variable is entirely absent from the environment.

### MCP_PORT
- Must be a valid 16-bit unsigned integer (u16).
- Valid range: 0 to 65535.
- Defaults to 3000 if not specified.
- Non-numeric strings result in a startup error.

### LOG_LEVEL
- Case-insensitive matching is performed for log levels.
- Defaults to "info" if an unrecognized level is provided (depending on the logging implementation).
- Recommended values are `trace`, `debug`, `info`, `warn`, or `error`.

### JWKS_CACHE_TTL
- Must be a valid 64-bit unsigned integer (u64).
- Represents the time-to-live in seconds for the JWKS cache.
- Defaults to 3600 seconds (1 hour).
- Must be 0 or greater.

## Advanced Overrides

While most users will stick to standard environment variables, the server's internal `Config` struct (defined in `src/config/mod.rs`) is designed to be extensible. Developers can modify the `from_env()` implementation to include additional security checks or integration with external configuration providers if necessary.

!!! tip "Production deployments"
    For production deployments, it is recommended to use a secrets management system to inject the `KEYCLOAK_URL` and other sensitive parameters into the environment rather than relying on static files.

!!! info "Network binding"
    The server binds to `0.0.0.0` by default to ensure it can receive traffic from outside the container when running in Docker or Kubernetes environments. Ensure that your firewall or security groups allow traffic on the configured `MCP_PORT`.
