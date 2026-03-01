use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;

use super::types::{IndexStats, SearchResult};
use crate::api::error::ApiError;

#[derive(Clone)]
pub struct MilvusClient {
    client: Arc<Client>,
    base_url: String,
    dimension: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct MilvusResponse<T> {
    code: i32,
    data: Option<T>,
    message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SearchHit {
    id: String,
    distance: f32,
    entity: Value,
}

#[derive(Debug, Serialize, Deserialize)]
struct CollectionStats {
    #[serde(rename = "rowCount")]
    row_count: u64,
}

impl MilvusClient {
    pub fn new(host: &str, port: u16, dimension: u32) -> Result<Self, ApiError> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(ApiError::HttpError)?;

        Ok(Self {
            client: Arc::new(client),
            base_url: format!("http://{}:{}/v2/vectordb", host, port),
            dimension,
        })
    }

    pub async fn create_collection(&self, collection_name: &str) -> Result<(), ApiError> {
        let url = format!("{}/collections/create", self.base_url);

        let payload = json!({
            "collectionName": collection_name,
            "dimension": self.dimension,
            "metricType": "COSINE",
            "primaryFieldName": "id",
            "vectorFieldName": "vector",
            "idType": "VarChar",
            "autoId": false,
            "params": {
                "max_length": 512
            }
        });

        let response = self
            .client
            .post(&url)
            .json(&payload)
            .send()
            .await
            .map_err(ApiError::HttpError)?;

        let result: MilvusResponse<Value> = response.json().await.map_err(ApiError::HttpError)?;

        if result.code != 0
            && !result
                .message
                .as_ref()
                .map(|m| m.contains("already exists"))
                .unwrap_or(false)
        {
            return Err(ApiError::ServerError(
                result
                    .message
                    .unwrap_or_else(|| "Unknown Milvus error".to_string()),
            ));
        }

        Ok(())
    }

    pub async fn collection_exists(&self, collection_name: &str) -> Result<bool, ApiError> {
        let url = format!("{}/collections/has", self.base_url);

        let payload = json!({
            "collectionName": collection_name
        });

        let response = self
            .client
            .post(&url)
            .json(&payload)
            .send()
            .await
            .map_err(ApiError::HttpError)?;

        let result: MilvusResponse<Value> = response.json().await.map_err(ApiError::HttpError)?;

        if result.code != 0 {
            return Err(ApiError::ServerError(
                result
                    .message
                    .unwrap_or_else(|| "Unknown Milvus error".to_string()),
            ));
        }

        Ok(result
            .data
            .and_then(|d| d.get("has").and_then(|v| v.as_bool()))
            .unwrap_or(false))
    }

    pub async fn insert(
        &self,
        collection_name: &str,
        ids: Vec<String>,
        vectors: Vec<Vec<f32>>,
        metadata: Vec<Value>,
    ) -> Result<u64, ApiError> {
        let url = format!("{}/entities/insert", self.base_url);

        let data: Vec<Value> = ids
            .into_iter()
            .zip(vectors.into_iter())
            .zip(metadata.into_iter())
            .map(|((id, vector), meta)| {
                let mut obj = meta.as_object().cloned().unwrap_or_default();
                obj.insert("id".to_string(), json!(id));
                obj.insert("vector".to_string(), json!(vector));
                json!(obj)
            })
            .collect();

        let payload = json!({
            "collectionName": collection_name,
            "data": data
        });

        let response = self
            .client
            .post(&url)
            .json(&payload)
            .send()
            .await
            .map_err(ApiError::HttpError)?;

        let result: MilvusResponse<Value> = response.json().await.map_err(ApiError::HttpError)?;

        if result.code != 0 {
            return Err(ApiError::ServerError(
                result
                    .message
                    .unwrap_or_else(|| "Unknown Milvus error".to_string()),
            ));
        }

        Ok(result
            .data
            .and_then(|d| d.get("insertCount").and_then(|v| v.as_u64()))
            .unwrap_or(0))
    }

    pub async fn search(
        &self,
        collection_name: &str,
        vector: Vec<f32>,
        limit: u32,
        filter: Option<String>,
        output_fields: Vec<&str>,
    ) -> Result<Vec<SearchResult>, ApiError> {
        let url = format!("{}/entities/search", self.base_url);

        let mut payload = json!({
            "collectionName": collection_name,
            "data": [vector],
            "limit": limit,
            "outputFields": output_fields,
            "params": {
                "nprobe": 10
            }
        });

        if let Some(f) = filter {
            payload["filter"] = json!(f);
        }

        let response = self
            .client
            .post(&url)
            .json(&payload)
            .send()
            .await
            .map_err(ApiError::HttpError)?;

        let result: MilvusResponse<Vec<Vec<SearchHit>>> =
            response.json().await.map_err(ApiError::HttpError)?;

        if result.code != 0 {
            return Err(ApiError::ServerError(
                result
                    .message
                    .unwrap_or_else(|| "Unknown Milvus error".to_string()),
            ));
        }

        let hits = result
            .data
            .and_then(|d| d.into_iter().next())
            .unwrap_or_default();

        let results = hits
            .into_iter()
            .map(|hit| {
                let text = hit
                    .entity
                    .get("text")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();

                SearchResult {
                    id: hit.id,
                    score: 1.0 - hit.distance,
                    text,
                    metadata: hit.entity,
                }
            })
            .collect();

        Ok(results)
    }

    pub async fn get_stats(&self, collection_name: &str) -> Result<IndexStats, ApiError> {
        let url = format!("{}/collections/get_stats", self.base_url);

        let payload = json!({
            "collectionName": collection_name
        });

        let response = self
            .client
            .post(&url)
            .json(&payload)
            .send()
            .await
            .map_err(ApiError::HttpError)?;

        let result: MilvusResponse<CollectionStats> =
            response.json().await.map_err(ApiError::HttpError)?;

        if result.code != 0 {
            return Err(ApiError::ServerError(
                result
                    .message
                    .unwrap_or_else(|| "Unknown Milvus error".to_string()),
            ));
        }

        Ok(IndexStats {
            collection: collection_name.to_string(),
            total_vectors: result.data.map(|d| d.row_count).unwrap_or(0),
            indexed_at: None,
        })
    }

    pub async fn delete_collection(&self, collection_name: &str) -> Result<(), ApiError> {
        let url = format!("{}/collections/drop", self.base_url);

        let payload = json!({
            "collectionName": collection_name
        });

        let response = self
            .client
            .post(&url)
            .json(&payload)
            .send()
            .await
            .map_err(ApiError::HttpError)?;

        let result: MilvusResponse<Value> = response.json().await.map_err(ApiError::HttpError)?;

        if result.code != 0 {
            return Err(ApiError::ServerError(
                result
                    .message
                    .unwrap_or_else(|| "Unknown Milvus error".to_string()),
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_milvus_client_creation() {
        let client = MilvusClient::new("localhost", 19530, 384);
        assert!(client.is_ok());
    }
}
