# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-02-01

### Added
- Initial release of Keycloak MCP Server.
- Full support for Keycloak Admin REST API:
    - Users CRUD and role mappings.
    - Clients CRUD and configuration.
    - Realms management.
    - Roles (realm and client level).
    - Groups management.
    - Authentication flows and executions.
    - Identity Providers management.
    - Client Scopes.
- OAuth 2.1 Resource Server implementation for secure access.
- Multi-stage Dockerfile for slim production images.
- Comprehensive test suite with 1100+ tests.
- MCP (Model Context Protocol) transport via HTTP.
