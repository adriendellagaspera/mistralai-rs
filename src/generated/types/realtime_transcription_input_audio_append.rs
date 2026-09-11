#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RealtimeTranscriptionInputAudioAppend {
    ///Base64-encoded raw PCM bytes matching the current audio_format. Max decoded size: 262144 bytes.
    pub audio: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<RealtimeTranscriptionInputAudioAppendType>,
}
