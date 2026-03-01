//! Keycloak Admin REST API client module.

pub mod authentication;
pub mod authorization;
pub mod client;
pub mod client_scopes;
pub mod clients;
pub mod code;
pub mod docs;
pub mod error;
pub mod groups;
pub mod identity_providers;
pub mod realms;
pub mod roles;
pub mod users;

pub use client::KeycloakClient;
pub use error::ApiError;

pub use code::{
    code_search, code_get_stats, CodeSearchService,
    CodeSearchParams, CodeGetStatsParams, CodeSearchResponse,
};
pub use docs::{
    docs_search, docs_get_stats, DocsSearchService,
    DocsSearchParams, DocsGetStatsParams, DocsSearchResponse,
};
