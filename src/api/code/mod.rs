use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

use crate::api::error::ApiError;
use crate::vector::{MilvusClient, EmbeddingService, IndexStats};

pub struct CodeSearchService {
    milvus: MilvusClient,
    embeddings: EmbeddingService,
    collection: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CodeSearchParams {
    pub realm: String,
    pub query: String,
    #[serde(default = "default_limit")]
    pub limit: u32,
    pub language: Option<String>,
    pub file_path: Option<String>,
    pub chunk_type: Option<String>,
}

fn default_limit() -> u32 { 10 }

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CodeGetFileParams {
    pub realm: String,
    pub file_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CodeGetStatsParams {
    pub realm: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeSearchResponse {
    pub results: Vec<CodeSearchHit>,
    pub total: usize,
    pub query: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeSearchHit {
    pub id: String,
    pub score: f32,
    pub text: String,
    pub file_path: String,
    pub language: String,
    pub chunk_type: String,
    pub name: Option<String>,
    pub start_line: u32,
    pub end_line: u32,
    pub git_tag: String,
}

impl CodeSearchService {
    pub fn new(milvus: MilvusClient, embeddings: EmbeddingService, collection: String) -> Self {
        Self { milvus, embeddings, collection }
    }

    pub async fn ensure_collection(&self) -> Result<(), ApiError> {
        if !self.milvus.collection_exists(&self.collection).await? {
            self.milvus.create_collection(&self.collection).await?;
        }
        Ok(())
    }

    pub async fn search(&self, params: &CodeSearchParams) -> Result<CodeSearchResponse, ApiError> {
        let query_vector = self.embeddings.embed_single(&params.query).await?;
        
        let mut filters = Vec::new();
        if let Some(lang) = &params.language {
            filters.push(format!("language == \"{}\"", lang));
        }
        if let Some(path) = &params.file_path {
            filters.push(format!("file_path like \"%{}%\"", path));
        }
        if let Some(ct) = &params.chunk_type {
            filters.push(format!("chunk_type == \"{}\"", ct));
        }
        
        let filter = if filters.is_empty() {
            None
        } else {
            Some(filters.join(" and "))
        };

        let output_fields = vec![
            "text", "file_path", "language", "chunk_type", 
            "name", "start_line", "end_line", "git_tag"
        ];
        
        let results = self.milvus.search(
            &self.collection,
            query_vector,
            params.limit,
            filter,
            output_fields,
        ).await?;

        let hits: Vec<CodeSearchHit> = results.into_iter()
            .map(|r| CodeSearchHit {
                id: r.id,
                score: r.score,
                text: r.text,
                file_path: r.metadata.get("file_path")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                language: r.metadata.get("language")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                chunk_type: r.metadata.get("chunk_type")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                name: r.metadata.get("name")
                    .and_then(|v| v.as_str())
                    .map(String::from),
                start_line: r.metadata.get("start_line")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0) as u32,
                end_line: r.metadata.get("end_line")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0) as u32,
                git_tag: r.metadata.get("git_tag")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
            })
            .collect();

        let total = hits.len();
        
        Ok(CodeSearchResponse {
            results: hits,
            total,
            query: params.query.clone(),
        })
    }

    pub async fn get_stats(&self) -> Result<IndexStats, ApiError> {
        self.milvus.get_stats(&self.collection).await
    }
}

pub async fn code_search(
    service: &CodeSearchService,
    params: &CodeSearchParams,
) -> Result<CodeSearchResponse, ApiError> {
    if params.query.trim().is_empty() {
        return Err(ApiError::BadRequest("query is required".to_string()));
    }
    service.search(params).await
}

pub async fn code_get_stats(
    service: &CodeSearchService,
    _params: &CodeGetStatsParams,
) -> Result<IndexStats, ApiError> {
    service.get_stats().await
}
