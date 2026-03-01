use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

use crate::api::error::ApiError;
use crate::vector::{MilvusClient, EmbeddingService, IndexStats};

pub struct DocsSearchService {
    milvus: MilvusClient,
    embeddings: EmbeddingService,
    collection: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DocsSearchParams {
    pub realm: String,
    pub query: String,
    #[serde(default = "default_limit")]
    pub limit: u32,
    pub doc_type: Option<String>,
}

fn default_limit() -> u32 { 10 }

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DocsListSectionsParams {
    pub realm: String,
    pub doc_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DocsGetStatsParams {
    pub realm: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocsSearchResponse {
    pub results: Vec<DocSearchHit>,
    pub total: usize,
    pub query: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocSearchHit {
    pub id: String,
    pub score: f32,
    pub text: String,
    pub source_url: String,
    pub doc_type: String,
    pub section_path: Vec<String>,
    pub heading: String,
}

impl DocsSearchService {
    pub fn new(milvus: MilvusClient, embeddings: EmbeddingService, collection: String) -> Self {
        Self { milvus, embeddings, collection }
    }

    pub async fn ensure_collection(&self) -> Result<(), ApiError> {
        if !self.milvus.collection_exists(&self.collection).await? {
            self.milvus.create_collection(&self.collection).await?;
        }
        Ok(())
    }

    pub async fn search(&self, params: &DocsSearchParams) -> Result<DocsSearchResponse, ApiError> {
        let query_vector = self.embeddings.embed_single(&params.query).await?;
        
        let filter = params.doc_type.as_ref().map(|dt| {
            format!("doc_type == \"{}\"", dt)
        });

        let output_fields = vec!["text", "source_url", "doc_type", "section_path", "heading"];
        
        let results = self.milvus.search(
            &self.collection,
            query_vector,
            params.limit,
            filter,
            output_fields,
        ).await?;

        let hits: Vec<DocSearchHit> = results.into_iter()
            .map(|r| DocSearchHit {
                id: r.id,
                score: r.score,
                text: r.text,
                source_url: r.metadata.get("source_url")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                doc_type: r.metadata.get("doc_type")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                section_path: r.metadata.get("section_path")
                    .and_then(|v| v.as_array())
                    .map(|arr| arr.iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect())
                    .unwrap_or_default(),
                heading: r.metadata.get("heading")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
            })
            .collect();

        let total = hits.len();
        
        Ok(DocsSearchResponse {
            results: hits,
            total,
            query: params.query.clone(),
        })
    }

    pub async fn get_stats(&self) -> Result<IndexStats, ApiError> {
        self.milvus.get_stats(&self.collection).await
    }
}

pub async fn docs_search(
    service: &DocsSearchService,
    params: &DocsSearchParams,
) -> Result<DocsSearchResponse, ApiError> {
    if params.query.trim().is_empty() {
        return Err(ApiError::BadRequest("query is required".to_string()));
    }
    service.search(params).await
}

pub async fn docs_get_stats(
    service: &DocsSearchService,
    _params: &DocsGetStatsParams,
) -> Result<IndexStats, ApiError> {
    service.get_stats().await
}
