//! Keycloak Admin REST API client module.

pub mod authentication;
pub mod authorization;
pub mod client;
pub mod client_scopes;
pub mod clients;
pub mod error;
pub mod groups;
pub mod identity_providers;
pub mod realms;
pub mod roles;
pub mod users;

pub use client::KeycloakClient;
pub use error::ApiError;
