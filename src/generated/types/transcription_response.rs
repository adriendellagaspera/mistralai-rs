#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TranscriptionResponse {
    pub language: Option<String>,
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments: Option<Vec<TranscriptionSegmentChunk>>,
    pub text: String,
    pub usage: UsageInfo,
}
