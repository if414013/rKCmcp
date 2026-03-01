//! Keycloak MCP Server Library
//!
//! A Model Context Protocol (MCP) server implementation for Keycloak integration.
//!
//! This library provides:
//! - `api`: Keycloak Admin REST API client and operations
//! - `auth`: Authentication and token management
//! - `config`: Configuration management
//! - `error`: Error types and handling
//! - `mcp`: MCP server implementation
//! - `vector`: Vector database integration for semantic search

pub mod api;
pub mod auth;
pub mod config;
pub mod error;
pub mod mcp;
pub mod vector;
