//! Keycloak HTTP client for Admin REST API operations.

use reqwest::{header, Client, Response, StatusCode};
use serde::{de::DeserializeOwned, Serialize};

use super::error::{ApiError, KeycloakErrorResponse};

/// HTTP client for communicating with Keycloak Admin REST API.
pub struct KeycloakClient {
    /// Base URL of the Keycloak server (e.g., "http://localhost:8080")
    base_url: String,
    /// Reqwest HTTP client instance
    client: Client,
}

impl KeycloakClient {
    /// Create a new KeycloakClient with the specified base URL.
    pub fn new(base_url: impl Into<String>) -> Result<Self, ApiError> {
        let client = Client::builder().build().map_err(ApiError::HttpError)?;

        Ok(Self {
            base_url: base_url.into().trim_end_matches('/').to_string(),
            client,
        })
    }

    /// Create a KeycloakClient with a custom reqwest Client.
    pub fn with_client(base_url: impl Into<String>, client: Client) -> Self {
        Self {
            base_url: base_url.into().trim_end_matches('/').to_string(),
            client,
        }
    }

    /// Get the base URL of this client.
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Perform a GET request and deserialize the response.
    pub async fn get<T: DeserializeOwned>(&self, path: &str, token: &str) -> Result<T, ApiError> {
        let url = format!("{}{}", self.base_url, path);

        let response = self
            .client
            .get(&url)
            .header(header::AUTHORIZATION, format!("Bearer {}", token))
            .header(header::ACCEPT, "application/json")
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Perform a POST request with a JSON body and deserialize the response.
    pub async fn post<T: Serialize, R: DeserializeOwned>(
        &self,
        path: &str,
        token: &str,
        body: &T,
    ) -> Result<R, ApiError> {
        let url = format!("{}{}", self.base_url, path);

        let response = self
            .client
            .post(&url)
            .header(header::AUTHORIZATION, format!("Bearer {}", token))
            .header(header::ACCEPT, "application/json")
            .json(body)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Perform a POST request with a JSON body, expecting no response body (201/204).
    pub async fn post_no_response<T: Serialize>(
        &self,
        path: &str,
        token: &str,
        body: &T,
    ) -> Result<(), ApiError> {
        let url = format!("{}{}", self.base_url, path);

        let response = self
            .client
            .post(&url)
            .header(header::AUTHORIZATION, format!("Bearer {}", token))
            .header(header::ACCEPT, "application/json")
            .json(body)
            .send()
            .await?;

        self.handle_empty_response(response).await
    }

    /// Perform a PUT request with a JSON body (typically returns no content).
    pub async fn put<T: Serialize>(
        &self,
        path: &str,
        token: &str,
        body: &T,
    ) -> Result<(), ApiError> {
        let url = format!("{}{}", self.base_url, path);

        let response = self
            .client
            .put(&url)
            .header(header::AUTHORIZATION, format!("Bearer {}", token))
            .header(header::ACCEPT, "application/json")
            .json(body)
            .send()
            .await?;

        self.handle_empty_response(response).await
    }

    /// Perform a DELETE request (typically returns no content).
    pub async fn delete(&self, path: &str, token: &str) -> Result<(), ApiError> {
        let url = format!("{}{}", self.base_url, path);

        let response = self
            .client
            .delete(&url)
            .header(header::AUTHORIZATION, format!("Bearer {}", token))
            .send()
            .await?;

        self.handle_empty_response(response).await
    }

    /// Perform a DELETE request with a JSON body (for operations like removing composite roles).
    pub async fn delete_with_body<T: Serialize>(
        &self,
        path: &str,
        token: &str,
        body: &T,
    ) -> Result<(), ApiError> {
        let url = format!("{}{}", self.base_url, path);

        let response = self
            .client
            .delete(&url)
            .header(header::AUTHORIZATION, format!("Bearer {}", token))
            .header(header::ACCEPT, "application/json")
            .json(body)
            .send()
            .await?;

        self.handle_empty_response(response).await
    }

    /// Perform a paginated GET request.
    ///
    /// Keycloak uses `first` (offset) and `max` (limit) query parameters for pagination.
    pub async fn get_paginated<T: DeserializeOwned>(
        &self,
        path: &str,
        token: &str,
        first: Option<i32>,
        max: Option<i32>,
    ) -> Result<Vec<T>, ApiError> {
        let mut url = format!("{}{}", self.base_url, path);

        let mut params = Vec::new();
        if let Some(f) = first {
            params.push(format!("first={}", f));
        }
        if let Some(m) = max {
            params.push(format!("max={}", m));
        }

        if !params.is_empty() {
            let separator = if url.contains('?') { "&" } else { "?" };
            url = format!("{}{}{}", url, separator, params.join("&"));
        }

        let response = self
            .client
            .get(&url)
            .header(header::AUTHORIZATION, format!("Bearer {}", token))
            .header(header::ACCEPT, "application/json")
            .send()
            .await?;

        self.handle_response(response).await
    }

    async fn handle_response<T: DeserializeOwned>(
        &self,
        response: Response,
    ) -> Result<T, ApiError> {
        let status = response.status();

        if status.is_success() {
            let body = response.text().await?;
            serde_json::from_str(&body).map_err(ApiError::from)
        } else {
            self.handle_error(status, response).await
        }
    }

    async fn handle_empty_response(&self, response: Response) -> Result<(), ApiError> {
        let status = response.status();

        if status.is_success() {
            Ok(())
        } else {
            self.handle_error(status, response).await
        }
    }

    async fn handle_error<T>(&self, status: StatusCode, response: Response) -> Result<T, ApiError> {
        let error_response = response
            .text()
            .await
            .ok()
            .and_then(|text| serde_json::from_str::<KeycloakErrorResponse>(&text).ok());

        Err(ApiError::from_response(status, error_response))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use wiremock::matchers::{header, method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct TestUser {
        id: String,
        username: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct CreateUserRequest {
        username: String,
    }

    const TEST_TOKEN: &str = "test-bearer-token";

    #[tokio::test]
    async fn test_get_success() {
        let mock_server = MockServer::start().await;

        let expected_user = TestUser {
            id: "123".to_string(),
            username: "testuser".to_string(),
        };

        Mock::given(method("GET"))
            .and(path("/admin/realms/master/users/123"))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_user))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).unwrap();
        let user: TestUser = client
            .get("/admin/realms/master/users/123", TEST_TOKEN)
            .await
            .unwrap();

        assert_eq!(user, expected_user);
    }

    #[tokio::test]
    async fn test_get_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/admin/realms/master/users/999"))
            .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
                "error": "User not found"
            })))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).unwrap();
        let result: Result<TestUser, _> = client
            .get("/admin/realms/master/users/999", TEST_TOKEN)
            .await;

        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_get_unauthorized() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/admin/realms/master/users"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).unwrap();
        let result: Result<Vec<TestUser>, _> = client
            .get("/admin/realms/master/users", "invalid-token")
            .await;

        assert!(matches!(result, Err(ApiError::Unauthorized)));
    }

    #[tokio::test]
    async fn test_get_forbidden() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/admin/realms/master/users"))
            .respond_with(ResponseTemplate::new(403))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).unwrap();
        let result: Result<Vec<TestUser>, _> =
            client.get("/admin/realms/master/users", TEST_TOKEN).await;

        assert!(matches!(result, Err(ApiError::Forbidden)));
    }

    #[tokio::test]
    async fn test_post_with_response() {
        let mock_server = MockServer::start().await;

        let request_body = CreateUserRequest {
            username: "newuser".to_string(),
        };

        let expected_response = TestUser {
            id: "456".to_string(),
            username: "newuser".to_string(),
        };

        Mock::given(method("POST"))
            .and(path("/admin/realms/master/users"))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .and(header("content-type", "application/json"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_response))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).unwrap();
        let user: TestUser = client
            .post("/admin/realms/master/users", TEST_TOKEN, &request_body)
            .await
            .unwrap();

        assert_eq!(user, expected_response);
    }

    #[tokio::test]
    async fn test_post_no_response_success() {
        let mock_server = MockServer::start().await;

        let request_body = CreateUserRequest {
            username: "newuser".to_string(),
        };

        Mock::given(method("POST"))
            .and(path("/admin/realms/master/users"))
            .respond_with(ResponseTemplate::new(201))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).unwrap();
        let result = client
            .post_no_response("/admin/realms/master/users", TEST_TOKEN, &request_body)
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_post_conflict() {
        let mock_server = MockServer::start().await;

        let request_body = CreateUserRequest {
            username: "existinguser".to_string(),
        };

        Mock::given(method("POST"))
            .and(path("/admin/realms/master/users"))
            .respond_with(ResponseTemplate::new(409).set_body_json(serde_json::json!({
                "errorMessage": "User exists with same username"
            })))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).unwrap();
        let result = client
            .post_no_response("/admin/realms/master/users", TEST_TOKEN, &request_body)
            .await;

        match result {
            Err(ApiError::Conflict(msg)) => {
                assert!(msg.contains("User exists"));
            }
            _ => panic!("Expected Conflict error"),
        }
    }

    #[tokio::test]
    async fn test_put_success() {
        let mock_server = MockServer::start().await;

        let update_body = TestUser {
            id: "123".to_string(),
            username: "updateduser".to_string(),
        };

        Mock::given(method("PUT"))
            .and(path("/admin/realms/master/users/123"))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).unwrap();
        let result = client
            .put("/admin/realms/master/users/123", TEST_TOKEN, &update_body)
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/admin/realms/master/users/123"))
            .and(header("Authorization", format!("Bearer {}", TEST_TOKEN)))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).unwrap();
        let result = client
            .delete("/admin/realms/master/users/123", TEST_TOKEN)
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/admin/realms/master/users/999"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).unwrap();
        let result = client
            .delete("/admin/realms/master/users/999", TEST_TOKEN)
            .await;

        assert!(matches!(result, Err(ApiError::NotFound)));
    }

    #[tokio::test]
    async fn test_get_paginated_with_params() {
        let mock_server = MockServer::start().await;

        let expected_users = vec![
            TestUser {
                id: "1".to_string(),
                username: "user1".to_string(),
            },
            TestUser {
                id: "2".to_string(),
                username: "user2".to_string(),
            },
        ];

        Mock::given(method("GET"))
            .and(path("/admin/realms/master/users"))
            .and(query_param("first", "10"))
            .and(query_param("max", "20"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_users))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).unwrap();
        let users: Vec<TestUser> = client
            .get_paginated("/admin/realms/master/users", TEST_TOKEN, Some(10), Some(20))
            .await
            .unwrap();

        assert_eq!(users.len(), 2);
        assert_eq!(users[0].username, "user1");
    }

    #[tokio::test]
    async fn test_get_paginated_without_params() {
        let mock_server = MockServer::start().await;

        let expected_users = vec![TestUser {
            id: "1".to_string(),
            username: "user1".to_string(),
        }];

        Mock::given(method("GET"))
            .and(path("/admin/realms/master/users"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_users))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).unwrap();
        let users: Vec<TestUser> = client
            .get_paginated("/admin/realms/master/users", TEST_TOKEN, None, None)
            .await
            .unwrap();

        assert_eq!(users.len(), 1);
    }

    #[tokio::test]
    async fn test_get_paginated_with_first_only() {
        let mock_server = MockServer::start().await;

        let expected_users: Vec<TestUser> = vec![];

        Mock::given(method("GET"))
            .and(path("/admin/realms/master/users"))
            .and(query_param("first", "100"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&expected_users))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).unwrap();
        let users: Vec<TestUser> = client
            .get_paginated("/admin/realms/master/users", TEST_TOKEN, Some(100), None)
            .await
            .unwrap();

        assert!(users.is_empty());
    }

    #[tokio::test]
    async fn test_server_error() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/admin/realms/master/users"))
            .respond_with(ResponseTemplate::new(500).set_body_json(serde_json::json!({
                "error_description": "Internal server error"
            })))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).unwrap();
        let result: Result<Vec<TestUser>, _> =
            client.get("/admin/realms/master/users", TEST_TOKEN).await;

        match result {
            Err(ApiError::ServerError(msg)) => {
                assert!(msg.contains("Internal server error"));
            }
            _ => panic!("Expected ServerError"),
        }
    }

    #[tokio::test]
    async fn test_bad_request() {
        let mock_server = MockServer::start().await;

        let request_body = CreateUserRequest {
            username: "".to_string(),
        };

        Mock::given(method("POST"))
            .and(path("/admin/realms/master/users"))
            .respond_with(ResponseTemplate::new(400).set_body_json(serde_json::json!({
                "error_description": "Username is required"
            })))
            .mount(&mock_server)
            .await;

        let client = KeycloakClient::new(mock_server.uri()).unwrap();
        let result = client
            .post_no_response("/admin/realms/master/users", TEST_TOKEN, &request_body)
            .await;

        match result {
            Err(ApiError::BadRequest(msg)) => {
                assert!(msg.contains("Username is required"));
            }
            _ => panic!("Expected BadRequest error"),
        }
    }

    #[tokio::test]
    async fn test_base_url_strips_trailing_slash() {
        let client = KeycloakClient::new("http://localhost:8080/").unwrap();
        assert_eq!(client.base_url(), "http://localhost:8080");
    }

    #[tokio::test]
    async fn test_with_custom_client() {
        let custom_client = Client::builder().build().unwrap();

        let client = KeycloakClient::with_client("http://localhost:8080", custom_client);
        assert_eq!(client.base_url(), "http://localhost:8080");
    }
}
