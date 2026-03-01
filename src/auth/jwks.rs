//! JWKS (JSON Web Key Set) fetching and caching
//!
//! Implements thread-safe JWKS fetching from Keycloak with TTL-based caching.

use crate::auth::error::AuthError;
use jsonwebtoken::jwk::{Jwk, JwkSet};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

const DEFAULT_CACHE_TTL_SECS: u64 = 300;
const MIN_CACHE_TTL_SECS: u64 = 60;

#[derive(Debug)]
struct CachedJwks {
    keys: JwkSet,
    fetched_at: Instant,
    ttl: Duration,
}

impl CachedJwks {
    fn is_expired(&self) -> bool {
        self.fetched_at.elapsed() > self.ttl
    }
}

/// Thread-safe JWKS cache with automatic refresh
#[derive(Debug)]
pub struct JwksCache {
    jwks_url: String,
    cache: Arc<RwLock<Option<CachedJwks>>>,
    http_client: reqwest::Client,
    ttl: Duration,
}

impl JwksCache {
    pub fn new(jwks_url: String) -> Self {
        Self::with_ttl(jwks_url, Duration::from_secs(DEFAULT_CACHE_TTL_SECS))
    }

    pub fn with_ttl(jwks_url: String, ttl: Duration) -> Self {
        let effective_ttl = if ttl.as_secs() < MIN_CACHE_TTL_SECS {
            Duration::from_secs(MIN_CACHE_TTL_SECS)
        } else {
            ttl
        };

        Self {
            jwks_url,
            cache: Arc::new(RwLock::new(None)),
            http_client: reqwest::Client::new(),
            ttl: effective_ttl,
        }
    }

    #[cfg(test)]
    pub fn with_client(jwks_url: String, client: reqwest::Client) -> Self {
        Self {
            jwks_url,
            cache: Arc::new(RwLock::new(None)),
            http_client: client,
            ttl: Duration::from_secs(DEFAULT_CACHE_TTL_SECS),
        }
    }

    pub async fn fetch_keys(&self) -> Result<JwkSet, AuthError> {
        {
            let cache = self.cache.read().await;
            if let Some(cached) = cache.as_ref() {
                if !cached.is_expired() {
                    return Ok(cached.keys.clone());
                }
            }
        }

        let mut cache = self.cache.write().await;

        // Double-check pattern: another thread may have refreshed while we waited for write lock
        if let Some(cached) = cache.as_ref() {
            if !cached.is_expired() {
                return Ok(cached.keys.clone());
            }
        }

        let response = self
            .http_client
            .get(&self.jwks_url)
            .send()
            .await
            .map_err(|e| AuthError::JwksFetchError(format!("HTTP request failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(AuthError::JwksFetchError(format!(
                "JWKS endpoint returned status {}",
                response.status()
            )));
        }

        let jwks: JwkSet = response
            .json()
            .await
            .map_err(|e| AuthError::JwksFetchError(format!("Failed to parse JWKS: {}", e)))?;

        *cache = Some(CachedJwks {
            keys: jwks.clone(),
            fetched_at: Instant::now(),
            ttl: self.ttl,
        });

        Ok(jwks)
    }

    pub async fn get_key(&self, kid: &str) -> Result<Jwk, AuthError> {
        let jwks = self.fetch_keys().await?;

        jwks.find(kid)
            .cloned()
            .ok_or_else(|| AuthError::KeyNotFound(kid.to_string()))
    }

    pub async fn invalidate(&self) {
        let mut cache = self.cache.write().await;
        *cache = None;
    }
}

pub fn build_jwks_url(keycloak_url: &str, realm: &str) -> String {
    let base = keycloak_url.trim_end_matches('/');
    format!("{}/realms/{}/protocol/openid-connect/certs", base, realm)
}

pub fn build_issuer_url(keycloak_url: &str, realm: &str) -> String {
    let base = keycloak_url.trim_end_matches('/');
    format!("{}/realms/{}", base, realm)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_jwks_url() {
        assert_eq!(
            build_jwks_url("http://localhost:8080", "master"),
            "http://localhost:8080/realms/master/protocol/openid-connect/certs"
        );
    }

    #[test]
    fn test_build_jwks_url_trailing_slash() {
        assert_eq!(
            build_jwks_url("http://localhost:8080/", "test-realm"),
            "http://localhost:8080/realms/test-realm/protocol/openid-connect/certs"
        );
    }

    #[test]
    fn test_build_issuer_url() {
        assert_eq!(
            build_issuer_url("http://localhost:8080", "master"),
            "http://localhost:8080/realms/master"
        );
    }

    #[test]
    fn test_build_issuer_url_trailing_slash() {
        assert_eq!(
            build_issuer_url("https://keycloak.example.com/", "prod"),
            "https://keycloak.example.com/realms/prod"
        );
    }

    #[test]
    fn test_jwks_cache_creation() {
        let cache = JwksCache::new("http://localhost:8080/certs".to_string());
        assert_eq!(cache.jwks_url, "http://localhost:8080/certs");
        assert_eq!(cache.ttl.as_secs(), DEFAULT_CACHE_TTL_SECS);
    }

    #[test]
    fn test_jwks_cache_min_ttl() {
        let cache = JwksCache::with_ttl(
            "http://localhost:8080/certs".to_string(),
            Duration::from_secs(10),
        );
        assert_eq!(cache.ttl.as_secs(), MIN_CACHE_TTL_SECS);
    }

    #[test]
    fn test_cached_jwks_expiration() {
        let cached = CachedJwks {
            keys: JwkSet { keys: vec![] },
            fetched_at: Instant::now() - Duration::from_secs(400),
            ttl: Duration::from_secs(300),
        };
        assert!(cached.is_expired());
    }

    #[test]
    fn test_cached_jwks_not_expired() {
        let cached = CachedJwks {
            keys: JwkSet { keys: vec![] },
            fetched_at: Instant::now(),
            ttl: Duration::from_secs(300),
        };
        assert!(!cached.is_expired());
    }
}
