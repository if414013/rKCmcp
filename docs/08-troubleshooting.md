# Troubleshooting Guide

This guide provides solutions for common issues encountered when using the Keycloak MCP Server.

## Authentication Errors

### 401 Unauthorized
The server rejects the request because the authentication token is missing, expired, or has an invalid signature.

**Solutions:**
* Verify the `Authorization` header is present and uses the `Bearer` scheme.
* Check the token expiration time; tokens must be refreshed periodically.
* Ensure the `KEYCLOAK_URL` environment variable exactly matches the issuer URL in the token (including protocol and port).
* Verify that the realm public key has not changed.

### 403 Forbidden
The token is valid, but the user or service account lacks the necessary permissions to perform the requested action.

**Solutions:**
* Check the user roles assigned in Keycloak.
* For administrative tasks, ensure the client has `realm-management` roles (e.g., `view-users`, `manage-users`).
* Verify that the client ID used for authentication has the correct scopes.

### JWKS Fetch Failed
The server cannot retrieve the JSON Web Key Set (JWKS) from Keycloak to verify token signatures.

**Solutions:**
* Verify that the Keycloak server is running and accessible.
* Check network connectivity between the MCP server and Keycloak.
* Ensure the `KEYCLOAK_URL` and `KEYCLOAK_REALM` are correct, as they form the JWKS endpoint URL.

## Connection Errors

### Connection Refused
The MCP server cannot establish a network connection to the Keycloak instance.

**Solutions:**
* Verify that Keycloak is running and listening on the expected port.
* Check firewall rules and network security groups.
* Verify the `KEYCLOAK_URL` environment variable points to the correct host and port.

### Timeout
Requests from the MCP server to Keycloak timed out before receiving a response.

**Solutions:**
* Check for network congestion or high latency between the servers.
* Ensure the Keycloak server is not overloaded.
* Increase the request timeout in the configuration if working with a slow network.

### SSL/TLS Errors
Certificate validation failed when attempting to connect to Keycloak via HTTPS.

**Solutions:**
* Verify that the Keycloak server is using a valid SSL certificate.
* If using self-signed certificates, ensure the CA certificate is added to the MCP server's trusted store.
* Check that the hostname in `KEYCLOAK_URL` matches the Common Name (CN) or Subject Alternative Name (SAN) in the certificate.

## MCP Protocol Errors

### Method Not Found
The client requested a tool or method that the server does not recognize.

**Solutions:**
* Check the tool name for typos.
* Use the `tools/list` command to see all available tools provided by the server.
* Ensure the server version supports the requested tool.

### Invalid Params
The parameters provided for a tool call failed validation against the defined schema.

**Solutions:**
* Verify that all required parameters are included in the request.
* Check that parameter types (e.g., string, integer) match the schema requirements.
* Consult the tool documentation for specific constraints on parameter values.

### Internal Error
An unexpected error occurred within the MCP server or during an API call to Keycloak.

**Solutions:**
* Check the MCP server logs for detailed error messages and stack traces.
* Verify the Keycloak API response for underlying issues.
* Ensure the server has sufficient resources (CPU, Memory).

## Common Configuration Issues

### Missing KEYCLOAK_URL
The server fails to start because the mandatory `KEYCLOAK_URL` environment variable is not set.

**Solution:**
* Set the `KEYCLOAK_URL` environment variable in your shell or configuration file (e.g., `export KEYCLOAK_URL=https://keycloak.example.com`).

### Invalid Port
The server cannot bind to the specified port because it is already in use or the value is invalid.

**Solutions:**
* Change the `MCP_PORT` environment variable to an available port.
* Identify and stop the service currently using the port.

### Realm Not Found
The specified Keycloak realm does not exist or is misspelled.

**Solution:**
* Verify that `KEYCLOAK_REALM` matches an existing realm name in your Keycloak instance. Note that realm names are case-sensitive.

## Debugging Tips

* **Enable Verbose Logging**: Set the `LOG_LEVEL=debug` environment variable to see detailed request and response information.
* **Health Check**: Use the `/health` endpoint (if available) to verify the server status.
* **Manual Connectivity Test**: Use `curl` from the MCP server host to test direct connectivity to the Keycloak API:
  ```bash
  curl -v ${KEYCLOAK_URL}/realms/${KEYCLOAK_REALM}/.well-known/openid-configuration
  ```
* **Token Verification**: For debugging purposes, decode tokens on [jwt.io](https://jwt.io) to inspect claims and issuer fields. **Warning: Never share production tokens or paste them into third-party websites.**

## Log Analysis

### Log Location
By default, the MCP server writes logs to `stdout`. If running as a service (e.g., via systemd or Docker), use the respective logging tools (e.g., `journalctl -u mcp-server` or `docker logs mcp-server`).

### Common Log Patterns
* `[ERROR] auth: ...` - Indicates an authentication or authorization failure.
* `[ERROR] api: ...` - Indicates an error returned by the Keycloak REST API.
* `[DEBUG] mcp: ...` - Provides information about incoming MCP requests and outgoing responses.

### Interpreting Errors
Error messages generally follow the format: `[Component] Context: Underlying Error`.
Example: `[api] Failed to fetch users: 404 Not Found` indicates the server successfully reached Keycloak, but the requested resource was missing.
