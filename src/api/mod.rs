//! Keycloak Admin REST API client module.

pub mod client;
pub mod error;

pub use client::KeycloakClient;
pub use error::ApiError;
