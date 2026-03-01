//! Keycloak Client Credentials API module.
//!
//! Provides tools for managing client secrets, registration tokens, and certificates.

use serde::{Deserialize, Serialize};

use crate::api::{ApiError, KeycloakClient};

/// Representation of a client secret.
#[derive(Debug, Clone, Serialize, Deserialize, Default, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClientSecretRepresentation {
    /// The type of secret (e.g., "secret")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    /// The secret value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// Representation of a certificate.
#[derive(Debug, Clone, Serialize, Deserialize, Default, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CertificateRepresentation {
    /// The private key (PEM encoded)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,

    /// The public key (PEM encoded)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,

    /// The certificate (PEM encoded)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,

    /// The key ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kid: Option<String>,
}

/// Parameters for getting a client secret.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClientSecretGetParams {
    /// The realm name
    pub realm: String,

    /// The client UUID (not client_id)
    pub id: String,
}

/// Parameters for regenerating a client secret.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClientSecretRegenerateParams {
    /// The realm name
    pub realm: String,

    /// The client UUID (not client_id)
    pub id: String,
}

/// Parameters for getting a client registration access token.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClientRegistrationTokenGetParams {
    /// The realm name
    pub realm: String,

    /// The client UUID (not client_id)
    pub id: String,
}

/// Parameters for regenerating a client registration access token.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClientRegistrationTokenRegenerateParams {
    /// The realm name
    pub realm: String,

    /// The client UUID (not client_id)
    pub id: String,
}

/// Parameters for getting client certificates.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClientCertificatesGetParams {
    /// The realm name
    pub realm: String,

    /// The client UUID (not client_id)
    pub id: String,

    /// The certificate attribute name (e.g., "jwt.credential")
    pub attr: String,
}

/// Parameters for generating a client certificate.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClientCertificateGenerateParams {
    /// The realm name
    pub realm: String,

    /// The client UUID (not client_id)
    pub id: String,

    /// The certificate attribute name (e.g., "jwt.credential")
    pub attr: String,
}

/// Get the client secret.
///
/// GET /admin/realms/{realm}/clients/{id}/client-secret
pub async fn client_secret_get(
    client: &KeycloakClient,
    token: &str,
    params: &ClientSecretGetParams,
) -> Result<ClientSecretRepresentation, ApiError> {
    let path = format!(
        "/admin/realms/{}/clients/{}/client-secret",
        params.realm,
        urlencoding::encode(&params.id)
    );

    client.get(&path, token).await
}

/// Regenerate the client secret.
///
/// POST /admin/realms/{realm}/clients/{id}/client-secret
pub async fn client_secret_regenerate(
    client: &KeycloakClient,
    token: &str,
    params: &ClientSecretRegenerateParams,
) -> Result<ClientSecretRepresentation, ApiError> {
    let path = format!(
        "/admin/realms/{}/clients/{}/client-secret",
        params.realm,
        urlencoding::encode(&params.id)
    );

    // POST with empty body to regenerate
    client.post(&path, token, &serde_json::Value::Null).await
}

/// Get the registration access token for the client.
///
/// GET /admin/realms/{realm}/clients/{id}/registration-access-token
pub async fn client_registration_token_get(
    client: &KeycloakClient,
    token: &str,
    params: &ClientRegistrationTokenGetParams,
) -> Result<ClientSecretRepresentation, ApiError> {
    let path = format!(
        "/admin/realms/{}/clients/{}/registration-access-token",
        params.realm,
        urlencoding::encode(&params.id)
    );

    client.get(&path, token).await
}

/// Regenerate the registration access token for the client.
///
/// POST /admin/realms/{realm}/clients/{id}/registration-access-token
pub async fn client_registration_token_regenerate(
    client: &KeycloakClient,
    token: &str,
    params: &ClientRegistrationTokenRegenerateParams,
) -> Result<ClientSecretRepresentation, ApiError> {
    let path = format!(
        "/admin/realms/{}/clients/{}/registration-access-token",
        params.realm,
        urlencoding::encode(&params.id)
    );

    // POST with empty body to regenerate
    client.post(&path, token, &serde_json::Value::Null).await
}

/// Get the client certificate for a specific attribute.
///
/// GET /admin/realms/{realm}/clients/{id}/certificates/{attr}
pub async fn client_certificates_get(
    client: &KeycloakClient,
    token: &str,
    params: &ClientCertificatesGetParams,
) -> Result<CertificateRepresentation, ApiError> {
    let path = format!(
        "/admin/realms/{}/clients/{}/certificates/{}",
        params.realm,
        urlencoding::encode(&params.id),
        urlencoding::encode(&params.attr)
    );

    client.get(&path, token).await
}

/// Generate a new certificate with new key pair for a specific attribute.
///
/// POST /admin/realms/{realm}/clients/{id}/certificates/{attr}/generate
pub async fn client_certificate_generate(
    client: &KeycloakClient,
    token: &str,
    params: &ClientCertificateGenerateParams,
) -> Result<CertificateRepresentation, ApiError> {
    let path = format!(
        "/admin/realms/{}/clients/{}/certificates/{}/generate",
        params.realm,
        urlencoding::encode(&params.id),
        urlencoding::encode(&params.attr)
    );

    // POST with empty body to generate
    client.post(&path, token, &serde_json::Value::Null).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    const TEST_TOKEN: &str = "test-bearer-token";

    fn sample_secret() -> ClientSecretRepresentation {
        ClientSecretRepresentation {
            r#type: Some("secret".to_string()),
            value: Some("super-secret-value".to_string()),
        }
    }

    fn sample_certificate() -> CertificateRepresentation {
        CertificateRepresentation {
            private_key: Some("-----BEGIN PRIVATE KEY-----\nMIIE...".to_string()),
            public_key: Some("-----BEGIN PUBLIC KEY-----\nMIIB...".to_string()),
            certificate: Some("-----BEGIN CERTIFICATE-----\nMIIC...".to_string()),
            kid: Some("abc-123-kid".to_string()),
        }
    }

    #[tokio::test]
    async fn test_client_secret_get_success() {
        let mock_server = MockServer::start().await;

        let expected_secret = sample_secret();

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/master/clients/client-uuid/client-secret",
            ))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_secret))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientSecretGetParams {
            realm: "master".to_string(),
            id: "client-uuid".to_string(),
        };

        let result = client_secret_get(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());

        let secret = result.expect("Failed to get secret");
        assert_eq!(secret.r#type, Some("secret".to_string()));
        assert_eq!(secret.value, Some("super-secret-value".to_string()));
    }

    #[tokio::test]
    async fn test_client_secret_get_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/master/clients/nonexistent/client-secret",
            ))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientSecretGetParams {
            realm: "master".to_string(),
            id: "nonexistent".to_string(),
        };

        let result = client_secret_get(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_client_secret_get_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/master/clients/client-uuid/client-secret",
            ))
            .respond_with(ResponseTemplate::new(401))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientSecretGetParams {
            realm: "master".to_string(),
            id: "client-uuid".to_string(),
        };

        let result = client_secret_get(&client, "invalid-token", &params).await;
        assert!(matches!(result, Err(ApiError::Unauthorized)));
    }

    #[tokio::test]
    async fn test_client_secret_regenerate_success() {
        let mock_server = MockServer::start().await;

        let expected_secret = ClientSecretRepresentation {
            r#type: Some("secret".to_string()),
            value: Some("new-regenerated-secret".to_string()),
        };

        Mock::given(method("POST"))
            .and(path(
                "/admin/realms/master/clients/client-uuid/client-secret",
            ))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_secret))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientSecretRegenerateParams {
            realm: "master".to_string(),
            id: "client-uuid".to_string(),
        };

        let result = client_secret_regenerate(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());

        let secret = result.expect("Failed to regenerate secret");
        assert_eq!(secret.value, Some("new-regenerated-secret".to_string()));
    }

    #[tokio::test]
    async fn test_client_secret_regenerate_forbidden() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path(
                "/admin/realms/master/clients/client-uuid/client-secret",
            ))
            .respond_with(ResponseTemplate::new(403))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientSecretRegenerateParams {
            realm: "master".to_string(),
            id: "client-uuid".to_string(),
        };

        let result = client_secret_regenerate(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::Forbidden)));
    }

    #[tokio::test]
    async fn test_client_registration_token_get_success() {
        let mock_server = MockServer::start().await;

        let expected_token = ClientSecretRepresentation {
            r#type: Some("RegistrationAccessToken".to_string()),
            value: Some("registration-token-value".to_string()),
        };

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/master/clients/client-uuid/registration-access-token",
            ))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_token))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientRegistrationTokenGetParams {
            realm: "master".to_string(),
            id: "client-uuid".to_string(),
        };

        let result = client_registration_token_get(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());

        let token_rep = result.expect("Failed to get registration token");
        assert_eq!(
            token_rep.r#type,
            Some("RegistrationAccessToken".to_string())
        );
        assert_eq!(
            token_rep.value,
            Some("registration-token-value".to_string())
        );
    }

    #[tokio::test]
    async fn test_client_registration_token_get_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/master/clients/nonexistent/registration-access-token",
            ))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientRegistrationTokenGetParams {
            realm: "master".to_string(),
            id: "nonexistent".to_string(),
        };

        let result = client_registration_token_get(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_client_registration_token_regenerate_success() {
        let mock_server = MockServer::start().await;

        let expected_token = ClientSecretRepresentation {
            r#type: Some("RegistrationAccessToken".to_string()),
            value: Some("new-registration-token".to_string()),
        };

        Mock::given(method("POST"))
            .and(path(
                "/admin/realms/master/clients/client-uuid/registration-access-token",
            ))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_token))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientRegistrationTokenRegenerateParams {
            realm: "master".to_string(),
            id: "client-uuid".to_string(),
        };

        let result = client_registration_token_regenerate(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());

        let token_rep = result.expect("Failed to regenerate registration token");
        assert_eq!(token_rep.value, Some("new-registration-token".to_string()));
    }

    #[tokio::test]
    async fn test_client_certificates_get_success() {
        let mock_server = MockServer::start().await;

        let expected_cert = sample_certificate();

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/master/clients/client-uuid/certificates/jwt.credential",
            ))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_cert))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientCertificatesGetParams {
            realm: "master".to_string(),
            id: "client-uuid".to_string(),
            attr: "jwt.credential".to_string(),
        };

        let result = client_certificates_get(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());

        let cert = result.expect("Failed to get certificates");
        assert!(cert.private_key.is_some());
        assert!(cert.public_key.is_some());
        assert!(cert.certificate.is_some());
        assert_eq!(cert.kid, Some("abc-123-kid".to_string()));
    }

    #[tokio::test]
    async fn test_client_certificates_get_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/master/clients/nonexistent/certificates/jwt.credential",
            ))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientCertificatesGetParams {
            realm: "master".to_string(),
            id: "nonexistent".to_string(),
            attr: "jwt.credential".to_string(),
        };

        let result = client_certificates_get(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_client_certificate_generate_success() {
        let mock_server = MockServer::start().await;

        let expected_cert = CertificateRepresentation {
            private_key: Some("-----BEGIN PRIVATE KEY-----\nNEW...".to_string()),
            public_key: Some("-----BEGIN PUBLIC KEY-----\nNEW...".to_string()),
            certificate: Some("-----BEGIN CERTIFICATE-----\nNEW...".to_string()),
            kid: Some("new-kid-456".to_string()),
        };

        Mock::given(method("POST"))
            .and(path(
                "/admin/realms/master/clients/client-uuid/certificates/jwt.credential/generate",
            ))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_cert))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientCertificateGenerateParams {
            realm: "master".to_string(),
            id: "client-uuid".to_string(),
            attr: "jwt.credential".to_string(),
        };

        let result = client_certificate_generate(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());

        let cert = result.expect("Failed to generate certificate");
        assert_eq!(cert.kid, Some("new-kid-456".to_string()));
    }

    #[tokio::test]
    async fn test_client_certificate_generate_forbidden() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path(
                "/admin/realms/master/clients/client-uuid/certificates/jwt.credential/generate",
            ))
            .respond_with(ResponseTemplate::new(403))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientCertificateGenerateParams {
            realm: "master".to_string(),
            id: "client-uuid".to_string(),
            attr: "jwt.credential".to_string(),
        };

        let result = client_certificate_generate(&client, TEST_TOKEN, &params).await;
        assert!(matches!(result, Err(ApiError::Forbidden)));
    }

    #[tokio::test]
    async fn test_client_secret_get_with_special_characters_in_id() {
        let mock_server = MockServer::start().await;

        let expected_secret = sample_secret();

        // Client ID with special characters that need URL encoding
        Mock::given(method("GET"))
            .and(path(
                "/admin/realms/my-realm/clients/client%2Fwith%2Bspecial/client-secret",
            ))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_secret))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).expect("Failed to create client");
        let params = ClientSecretGetParams {
            realm: "my-realm".to_string(),
            id: "client/with+special".to_string(),
        };

        let result = client_secret_get(&client, TEST_TOKEN, &params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_client_secret_representation_serialization() {
        let secret = ClientSecretRepresentation {
            r#type: Some("secret".to_string()),
            value: Some("my-secret".to_string()),
        };

        let json = serde_json::to_string(&secret).expect("Failed to serialize");
        assert!(json.contains("\"type\":\"secret\""));
        assert!(json.contains("\"value\":\"my-secret\""));
    }

    #[tokio::test]
    async fn test_client_secret_representation_deserialization() {
        let json = r#"{"type": "secret", "value": "test-value"}"#;

        let secret: ClientSecretRepresentation =
            serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(secret.r#type, Some("secret".to_string()));
        assert_eq!(secret.value, Some("test-value".to_string()));
    }

    #[tokio::test]
    async fn test_certificate_representation_serialization() {
        let cert = CertificateRepresentation {
            private_key: Some("private".to_string()),
            public_key: Some("public".to_string()),
            certificate: Some("cert".to_string()),
            kid: Some("kid-123".to_string()),
        };

        let json = serde_json::to_string(&cert).expect("Failed to serialize");
        assert!(json.contains("\"privateKey\":\"private\""));
        assert!(json.contains("\"publicKey\":\"public\""));
        assert!(json.contains("\"certificate\":\"cert\""));
        assert!(json.contains("\"kid\":\"kid-123\""));
    }

    #[tokio::test]
    async fn test_certificate_representation_deserialization() {
        let json = r#"{
            "privateKey": "pk",
            "publicKey": "pub",
            "certificate": "cert",
            "kid": "key-id"
        }"#;

        let cert: CertificateRepresentation =
            serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(cert.private_key, Some("pk".to_string()));
        assert_eq!(cert.public_key, Some("pub".to_string()));
        assert_eq!(cert.certificate, Some("cert".to_string()));
        assert_eq!(cert.kid, Some("key-id".to_string()));
    }

    #[test]
    fn test_client_secret_representation_default() {
        let secret = ClientSecretRepresentation::default();
        assert!(secret.r#type.is_none());
        assert!(secret.value.is_none());
    }

    #[test]
    fn test_certificate_representation_default() {
        let cert = CertificateRepresentation::default();
        assert!(cert.private_key.is_none());
        assert!(cert.public_key.is_none());
        assert!(cert.certificate.is_none());
        assert!(cert.kid.is_none());
    }
}
