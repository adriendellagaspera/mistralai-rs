#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct RealtimeTranscriptionInputAudioFlush {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<RealtimeTranscriptionInputAudioFlushType>,
}
