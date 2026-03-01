# Keycloak MCP Server

An MCP (Model Context Protocol) server for the Keycloak Admin API, enabling AI models to manage Keycloak realms, users, clients, roles, and more.

## Features

- **User Management**: Create, read, update, delete users, manage credentials, and list sessions.
- **Client Management**: Create and manage OIDC/SAML clients, including client secrets and scopes.
- **Role & Group Management**: Create and assign realm and client roles, manage group hierarchies.
- **Realm Management**: Configure realm settings, security browser headers, and themes.
- **Identity Providers**: Configure social and OIDC/SAML identity providers.
- **Authentication**: Manage authentication flows, executions, and requirements.
- **Authorization**: Fine-grained authorization resources, policies, and permissions.
- **OAuth 2.1 Support**: Built-in support for secure OAuth 2.1 resource server implementation.

## Prerequisites

- **Rust**: 1.75 or later.
- **Docker**: For containerized deployment (optional).
- **Keycloak**: 26.0 or later (older versions may work but are not officially tested).

## Quick Start

### 1. Configure the environment
Copy the `.env.example` to `.env` and update the values:

```bash
cp .env.example .env
```

Key required variables:
- `KEYCLOAK_URL`: Your Keycloak base URL (e.g., `http://localhost:8080`)
- `KEYCLOAK_REALM`: The realm to authenticate against (default: `master`)

### 2. Build and Run
```bash
cargo build --release
./target/release/keycloak-mcp-server
```

The server will start listening on the port specified by `MCP_PORT` (default: `3000`).

## Docker Deployment

You can run the server and Keycloak together using Docker Compose:

```bash
docker-compose up -d
```

This will start:
- Keycloak on port 8080
- Keycloak MCP Server on port 3000

## Configuration Options

| Variable | Description | Default |
|----------|-------------|---------|
| `KEYCLOAK_URL` | Base URL of Keycloak | `http://localhost:8080` |
| `KEYCLOAK_REALM` | Realm for Admin API access | `master` |
| `MCP_PORT` | Port for the MCP server | `3000` |
| `LOG_LEVEL` | Logging verbosity (trace/debug/info/warn/error) | `info` |
| `JWKS_CACHE_TTL` | JWKS cache time-to-live in seconds | `3600` |

## API Coverage Summary

The server provides comprehensive coverage for Keycloak Admin REST API:
- **Realms**: Full CRUD and configuration.
- **Users**: Search, CRUD, password management, role mapping.
- **Clients**: CRUD, secrets, scopes, mappers.
- **Roles**: Realm and client level roles.
- **Groups**: Hierarchy management, member management.
- **Authentication**: Flows and executions.
- **Identity Providers**: Configuration and management.

## Documentation

For detailed documentation, see the [docs/](./docs/) directory:

| Document | Description |
|----------|-------------|
| [Overview](./docs/01-overview.md) | Project introduction and quick start guide |
| [Architecture](./docs/02-architecture.md) | System architecture with diagrams |
| [Request Flow](./docs/03-request-flow.md) | Request lifecycle and authentication flow |
| [Components](./docs/04-components/) | Detailed component documentation |
| [API Reference](./docs/05-api-reference/) | Complete tool reference (150+ tools) |
| [Configuration](./docs/06-configuration.md) | Environment variables and settings |
| [Extending](./docs/07-extending.md) | Guide for adding new tools |
| [Troubleshooting](./docs/08-troubleshooting.md) | Common issues and solutions |

## Development Setup

To run tests:
```bash
cargo test
```

To check for linting issues:
```bash
cargo clippy
```

To format code:
```bash
cargo fmt
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
