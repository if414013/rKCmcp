use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentChunk {
    pub id: String,
    pub text: String,
    pub embedding: Option<Vec<f32>>,
    pub metadata: DocumentMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMetadata {
    pub source_url: String,
    pub doc_type: String,
    pub section_path: Vec<String>,
    pub heading: String,
    pub heading_level: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeChunk {
    pub id: String,
    pub text: String,
    pub embedding: Option<Vec<f32>>,
    pub metadata: CodeMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeMetadata {
    pub file_path: String,
    pub language: String,
    pub chunk_type: String,
    pub name: Option<String>,
    pub parent_name: Option<String>,
    pub start_line: u32,
    pub end_line: u32,
    pub git_tag: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: String,
    pub score: f32,
    pub text: String,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchRequest {
    pub query: String,
    pub limit: Option<u32>,
    pub filter: Option<SearchFilter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchFilter {
    pub doc_type: Option<String>,
    pub language: Option<String>,
    pub file_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexStats {
    pub collection: String,
    pub total_vectors: u64,
    pub indexed_at: Option<String>,
}
