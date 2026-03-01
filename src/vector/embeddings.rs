use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::api::error::ApiError;

#[derive(Clone)]
pub struct EmbeddingService {
    client: Arc<Client>,
    model: String,
    dimension: u32,
    api_key: Option<String>,
    provider: EmbeddingProvider,
}

#[derive(Clone, Debug)]
pub enum EmbeddingProvider {
    OpenAI,
    Local,
}

#[derive(Debug, Serialize)]
struct OpenAIEmbeddingRequest {
    model: String,
    input: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct OpenAIEmbeddingResponse {
    data: Vec<OpenAIEmbeddingData>,
}

#[derive(Debug, Deserialize)]
struct OpenAIEmbeddingData {
    embedding: Vec<f32>,
}

impl EmbeddingService {
    pub fn new(model: &str, dimension: u32, api_key: Option<String>) -> Result<Self, ApiError> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .build()
            .map_err(ApiError::HttpError)?;

        let provider = if api_key.is_some() {
            EmbeddingProvider::OpenAI
        } else {
            EmbeddingProvider::Local
        };

        Ok(Self {
            client: Arc::new(client),
            model: model.to_string(),
            dimension,
            api_key,
            provider,
        })
    }

    pub fn dimension(&self) -> u32 {
        self.dimension
    }

    pub async fn embed(&self, texts: Vec<String>) -> Result<Vec<Vec<f32>>, ApiError> {
        match self.provider {
            EmbeddingProvider::OpenAI => self.embed_openai(texts).await,
            EmbeddingProvider::Local => self.embed_local(texts).await,
        }
    }

    pub async fn embed_single(&self, text: &str) -> Result<Vec<f32>, ApiError> {
        let results = self.embed(vec![text.to_string()]).await?;
        results
            .into_iter()
            .next()
            .ok_or_else(|| ApiError::ServerError("No embedding returned".to_string()))
    }

    async fn embed_openai(&self, texts: Vec<String>) -> Result<Vec<Vec<f32>>, ApiError> {
        let api_key = self
            .api_key
            .as_ref()
            .ok_or_else(|| ApiError::BadRequest("OpenAI API key required".to_string()))?;

        let request = OpenAIEmbeddingRequest {
            model: self.model.clone(),
            input: texts,
        };

        let response = self
            .client
            .post("https://api.openai.com/v1/embeddings")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(ApiError::HttpError)?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(ApiError::ServerError(format!(
                "OpenAI API error {}: {}",
                status, body
            )));
        }

        let result: OpenAIEmbeddingResponse = response.json().await.map_err(ApiError::HttpError)?;

        Ok(result.data.into_iter().map(|d| d.embedding).collect())
    }

    async fn embed_local(&self, texts: Vec<String>) -> Result<Vec<Vec<f32>>, ApiError> {
        let embeddings: Vec<Vec<f32>> = texts
            .iter()
            .map(|text| self.simple_hash_embedding(text))
            .collect();

        Ok(embeddings)
    }

    fn simple_hash_embedding(&self, text: &str) -> Vec<f32> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut embedding = vec![0.0f32; self.dimension as usize];
        let words: Vec<&str> = text.split_whitespace().collect();

        for (i, word) in words.iter().enumerate() {
            let mut hasher = DefaultHasher::new();
            word.to_lowercase().hash(&mut hasher);
            let hash = hasher.finish();

            for j in 0..8 {
                let idx = ((hash >> (j * 8)) as usize + i) % self.dimension as usize;
                let val = ((hash >> (j * 4)) & 0xFF) as f32 / 255.0 - 0.5;
                embedding[idx] += val;
            }
        }

        let magnitude: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        if magnitude > 0.0 {
            for val in &mut embedding {
                *val /= magnitude;
            }
        }

        embedding
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embedding_service_creation() {
        let service = EmbeddingService::new("all-MiniLM-L6-v2", 384, None);
        assert!(service.is_ok());
    }

    #[test]
    fn test_local_embedding() {
        let service = EmbeddingService::new("local", 384, None).unwrap();
        let embedding = service.simple_hash_embedding("hello world");
        assert_eq!(embedding.len(), 384);

        let magnitude: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        assert!((magnitude - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_embedding_consistency() {
        let service = EmbeddingService::new("local", 384, None).unwrap();
        let emb1 = service.simple_hash_embedding("test query");
        let emb2 = service.simple_hash_embedding("test query");
        assert_eq!(emb1, emb2);
    }
}
