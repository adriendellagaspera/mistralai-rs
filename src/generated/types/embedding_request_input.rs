///Text to embed.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum EmbeddingRequestInput {
    String(String),
    EmbeddingRequestInputStringArray(EmbeddingRequestInputStringArray),
}
