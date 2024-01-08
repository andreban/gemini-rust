use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CountTokensRequest {
    pub contents: Content,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CountTokensResponse {
    pub total_tokens: i32,
}

#[derive(Serialize, Deserialize)]
pub struct GenerateContentRequest {
    pub contents: Vec<Content>,
    pub generation_config: GenerationConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Content {
    pub role: String,
    pub parts: Vec<Part>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerationConfig {
    pub max_output_tokens: i32,
    pub temperature: f32,
    pub top_p: f32,
    pub top_k: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Part {
    Text(String),
    InlineData { mime_type: String, data: String },
    FileData { mime_type: String, file_uri: String },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateContentResponse(pub Vec<ResponseStreamChunk>);

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseStreamChunk {
    pub candidates: Vec<Candidate>,
    pub usage_metadata: Option<UsageMetadata>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Candidate {
    pub content: Content,
    pub citation_metadata: Option<CitationMetadata>,
    pub safety_ratings: Vec<SafetyRating>,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SafetyRating {
    pub category: String,
    pub probability: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Citation {
    start_index: i32,
    end_index: i32,
    uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CitationMetadata {
    pub citations: Vec<Citation>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsageMetadata {
    candidates_token_count: i32,
    prompt_token_count: i32,
    total_token_count: i32,
}
