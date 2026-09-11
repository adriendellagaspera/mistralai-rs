#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SpeechStreamAudioDelta {
    pub audio_data: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<SpeechStreamAudioDeltaType>,
}
