#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct RealtimeTranscriptionSessionUpdatePayload {
    ///Set before sending audio. Audio format updates are rejected after audio starts.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub audio_format: Option<Option<AudioFormat>>,
    ///Set before sending audio. Streaming delay updates are rejected after audio starts.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub target_streaming_delay_ms: Option<Option<i64>>,
}
