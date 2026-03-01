# Keycloak MCP Server Overview

This document provides a comprehensive introduction to the Keycloak Model Context Protocol (MCP) Server, detailing its purpose, features, design philosophy, and quick start instructions.

## Table of Contents

1. [Project Introduction](#project-introduction)
2. [What is MCP?](#what-is-mcp)
3. [Key Features](#key-features)
   - [API Categories and Tooling](#api-categories-and-tooling)
   - [OAuth 2.1 and Security](#oauth-21-and-security)
4. [Design Philosophy](#design-philosophy)
   - [Safety and Validation](#safety-and-validation)
   - [Transparency and Error Handling](#transparency-and-error-handling)
5. [Use Cases](#use-cases)
   - [Administrative Automation](#administrative-automation)
   - [Security Compliance](#security-compliance)
6. [Technology Stack](#technology-stack)
   - [Rust and Axum](#rust-and-axum)
   - [The rmcp SDK](#the-rmcp-sdk)
7. [Quick Start](#quick-start)
   - [Prerequisites](#prerequisites)
   - [Installation](#installation)
   - [Configuration](#configuration)
   - [Running the Server](#running-the-server)
8. [Monitoring and Health](#monitoring-and-health)
9. [Integration Guidelines](#integration-guidelines)
10. [Documentation Navigation](#documentation-navigation)

## Project Introduction

The Keycloak MCP Server is a high-performance, specialized interface designed to bridge the gap between Keycloak's robust Identity and Access Management (IAM) capabilities and the emerging ecosystem of Large Language Model (LLM) powered agents. By implementing the Model Context Protocol, this server transforms the complex and often overwhelming Keycloak Admin REST API into a set of structured, discoverable, and easily invokable tools for AI models.

In modern enterprise environments, managing identity and access is a critical but labor-intensive task. The Keycloak MCP Server enables organizations to leverage AI for automating routine administrative duties, performing complex security audits, and streamlining developer onboarding workflows. It acts as an intelligent proxy that understands both the protocol requirements of LLMs and the intricate domain model of Keycloak.

The server is built with a focus on type safety, performance, and security, ensuring that even as agents perform complex operations, the underlying system remains stable and protected. It allows developers to interact with Keycloak using natural language through their favorite MCP-compatible AI assistants.

## What is MCP?

The Model Context Protocol (MCP) is an open-source standard that enables secure, standardized communication between AI applications and external data sources or services. Developed to solve the "integration wall" faced by many LLM applications, MCP provides:

- **Universal Connectivity**: A single protocol to connect models to any supporting data source or service without writing custom glue code for every integration.
- **Standardized Tool Discovery**: A formal mechanism for models to discover what actions a server can perform, including the required parameters and expected output formats.
- **Structured Interaction**: A JSON-RPC based communication model that ensures predictable and verifiable exchanges between the client and the server.
- **Secure Transport**: Support for various transport layers, including standard input/output (stdio) for local tools and HTTP with Server-Sent Events (SSE) for remote services.

By adopting MCP, the Keycloak MCP Server allows any compatible client (such as IDE extensions, standalone AI agents, or custom LLM wrappers) to interact with Keycloak without requiring the developer to understand the nuances of the Keycloak Admin API directly.

## Key Features

The Keycloak MCP Server is designed to be the definitive MCP implementation for Keycloak, offering features that go beyond simple API mapping.

### API Categories and Tooling

The server exposes over 150+ tools, organized into nine logical categories. This structure helps agents navigate the vast capabilities of Keycloak and allows for granular permissioning:

1. **Users**: Comprehensive management of the user lifecycle. Tools include `create_user`, `update_user_password`, `add_user_to_group`, and `search_users_by_attribute`.
2. **Clients**: Management of OpenID Connect (OIDC) and SAML 2.0 clients. Tools facilitate `generate_client_secret`, `configure_redirect_uris`, and `manage_client_protocol_mappers`.
3. **Roles**: Granular control over realm and client roles. Includes management of role hierarchies and composite roles.
4. **Groups**: Management of organizational structures within Keycloak, allowing for hierarchical group management and membership synchronization.
5. **Realms**: Global configuration of realm-level security policies, including token lifetimes, password policies, and SMTP settings.
6. **Authentication**: Dynamic management of authentication flows. Tools allow for the modification of execution priorities and the configuration of required actions like MFA.
7. **Authorization**: Support for Keycloak's Authorization Services, enabling agents to define resources, scopes, and complex policies (JavaScript, Role-based, Time-based).
8. **Identity Providers**: Simplified configuration of social logins (Google, GitHub) and enterprise federations (OIDC, SAML), including brokering configurations.
9. **Client Scopes**: Management of shared client scopes to ensure consistent token content across multiple applications.

### OAuth 2.1 and Security

Security is at the heart of the Keycloak MCP Server. It is designed to operate as a modern OAuth 2.1 Resource Server:

- **Token Validation**: The server validates incoming Bearer tokens using Keycloak's JWKS endpoint, ensuring that only authenticated requests are processed.
- **Scope Verification**: Fine-grained access control ensures that the calling agent has the necessary permissions to execute specific tools based on the scopes present in its access token.
- **Audit Logging**: Every tool execution is logged with context, providing a clear trail of actions taken by AI agents for compliance and debugging.

## Design Philosophy

The development of the Keycloak MCP Server is guided by three core principles:

### Safety and Validation

Given that Keycloak manages critical identity data, the server implements strict validation on all inputs. It uses Rust's type system to ensure that data conforms to the expected schema before it is ever sent to the Keycloak API. This prevents common errors such as malformed JSON or invalid parameter types from causing issues in the identity provider.

### Transparency and Error Handling

Agents need to know why an action failed to provide useful feedback to the user. The server provides detailed, human-readable error messages and status codes that are meaningful to both human developers and LLMs. When a tool fails, the server returns context-rich errors that help the agent decide whether to retry or ask the user for clarification.

## Use Cases

The integration of Keycloak with MCP opens up several powerful use cases for enterprise environments:

### Administrative Automation

An agent can receive a request to "set up a new developer," and proceed to create the user, assign them to the correct groups, and provision necessary client roles. This reduces the time spent on manual configuration in the Keycloak admin console.

### Security Compliance

Agents can periodically scan realms for insecure configurations, such as clients with broad redirect URIs or users with missing MFA, and generate remediation reports. They can even be authorized to apply fixes automatically, ensuring the environment remains compliant with security policies.

## Technology Stack

The server is engineered for maximum reliability and efficiency using the following technologies:

### Rust and Axum

- **Rust**: Provides the performance of C++ with the memory safety guarantees required for security-critical infrastructure. The language's async/await model is perfectly suited for handling the I/O-bound nature of API proxying.
- **Axum**: A high-performance web framework built on top of `tokio` and `tower`. It handles the HTTP transport layer for the MCP server, providing robust routing and middleware support.

### The rmcp SDK

- **rmcp**: The project utilizes the `rmcp` SDK, a native Rust implementation of the Model Context Protocol. This SDK facilitates the registration of tools, handling of JSON-RPC messages, and management of server-side state.
- **StreamableHttpService**: For HTTP-based transport, the server uses a streaming-friendly architecture that ensures low-latency responses, even when dealing with large datasets from Keycloak.

## Quick Start

### Prerequisites

Ensure your environment meets the following requirements:

- **Rust Toolchain**: 1.75.0 or later (recommended to use `rustup`).
- **Keycloak Instance**: Version 26.0.0 or later.
- **Network Access**: The server must be able to reach the Keycloak `auth` endpoint.

### Installation

Clone the repository and compile the source code using Cargo:

```bash
git clone https://github.com/your-org/keycloak-mcp-server.git
cd keycloak-mcp-server
cargo build --release
```

The resulting binary will be located at `./target/release/keycloak-mcp-server`.

### Configuration

The server is primarily configured via environment variables. You can set these in your shell or use a `.env` file in the root directory:

- `KEYCLOAK_BASE_URL`: The full URL to your Keycloak instance.
- `KEYCLOAK_REALM`: The administrative realm.
- `KEYCLOAK_ADMIN_USER`: Username for an admin-privileged user.
- `KEYCLOAK_ADMIN_PASSWORD`: Password for the admin user.
- `SERVER_PORT`: Port the MCP server will bind to (defaults to `3000`).

### Running the Server

Start the binary to begin listening for MCP requests:

```bash
./target/release/keycloak-mcp-server
```

The server will log its initialization process, including the registration of available tools.

## Monitoring and Health

The server provides built-in endpoints for monitoring its operational status:

- **Health Check**: `GET /health` returns a 200 OK status if the server is healthy.

## Integration Guidelines

When integrating this server with an MCP client (like Claude Desktop or a custom agent):

1. Ensure the client is configured to use the SSE transport pointed at `/mcp`.
2. Verify that the agent has been granted sufficient permissions within Keycloak to perform the requested actions.
3. Use the `list_tools` capability to verify which Keycloak domains are currently manageable.

## Documentation Navigation

Explore the rest of the documentation for more in-depth technical details:

- [Architecture](./02-architecture.md): Understanding the internal modules and design patterns.
- [Request Flow](./03-request-flow.md): Tracing a request from the LLM to Keycloak and back.
- [Components](./04-components/): Deep dive into the tool registry and Keycloak client.
- [API Reference](./05-api-reference/): A complete catalog of all 150+ tools with schemas.
- [Configuration](./06-configuration.md): Detailed explanation of all environment variables.
- [Extending](./07-extending.md): Instructions for adding new tool categories or custom logic.
- [Troubleshooting](./08-troubleshooting.md): Solutions for common connectivity issues.

---
*Keycloak MCP Server Documentation - Version 1.0.0*
*Built with Rust and the Model Context Protocol*
