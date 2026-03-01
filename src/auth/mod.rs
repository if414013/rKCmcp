//! OAuth 2.1 Resource Server authentication module

pub mod error;
pub mod jwks;
pub mod jwt;
pub mod middleware;
pub mod protected_resource;

pub use error::AuthError;
pub use jwks::{build_issuer_url, build_jwks_url, JwksCache};
pub use jwt::{extract_bearer_token, Claims, JwtValidator, JwtValidatorConfig, RealmAccess};
pub use middleware::{
    auth_middleware, optional_auth_middleware, require_role, require_scope, AuthState,
};
pub use protected_resource::{
    build_protected_resource_metadata, protected_resource_metadata_handler,
    ProtectedResourceMetadata, ProtectedResourceState,
};
