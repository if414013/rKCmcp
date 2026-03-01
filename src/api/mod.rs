//! Keycloak Admin REST API client module.

pub mod client;
pub mod clients;
pub mod error;
pub mod roles;
pub mod users;

pub use client::KeycloakClient;
pub use error::ApiError;
