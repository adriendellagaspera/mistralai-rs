#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmbeddingResponse {
    pub data: Vec<EmbeddingResponseData>,
    pub id: String,
    pub model: String,
    pub object: String,
    pub usage: UsageInfo,
}
