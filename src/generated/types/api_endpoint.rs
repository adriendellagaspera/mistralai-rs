#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ApiEndpoint {
    #[default]
    #[serde(rename = "/v1/chat/completions")]
    V1ChatCompletions,
    #[serde(rename = "/v1/embeddings")]
    V1Embeddings,
    #[serde(rename = "/v1/fim/completions")]
    V1FimCompletions,
    #[serde(rename = "/v1/moderations")]
    V1Moderations,
    #[serde(rename = "/v1/chat/moderations")]
    V1ChatModerations,
    #[serde(rename = "/v1/ocr")]
    V1Ocr,
    #[serde(rename = "/v1/classifications")]
    V1Classifications,
    #[serde(rename = "/v1/chat/classifications")]
    V1ChatClassifications,
    #[serde(rename = "/v1/conversations")]
    V1Conversations,
    #[serde(rename = "/v1/audio/transcriptions")]
    V1AudioTranscriptions,
}
