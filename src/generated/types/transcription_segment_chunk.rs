#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TranscriptionSegmentChunk {
    pub end: f64,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub score: Option<Option<f64>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub speaker_id: Option<Option<String>>,
    pub start: f64,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<TranscriptionSegmentChunkType>,
}
