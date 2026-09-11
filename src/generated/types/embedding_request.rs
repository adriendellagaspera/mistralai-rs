#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmbeddingRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_format: Option<EncodingFormat>,
    ///Text to embed.
    pub input: EmbeddingRequestInput,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub metadata: Option<Option<EmbeddingRequestMetadata>>,
    ///ID of the model to use.
    pub model: String,
    ///The dimension of the output embeddings when feature available. If not provided, a default output dimension will be used.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub output_dimension: Option<Option<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_dtype: Option<EmbeddingDtype>,
}
