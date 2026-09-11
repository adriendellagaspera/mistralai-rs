#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum RealtimeTranscriptionInputAudioEndType {
    #[default]
    #[serde(rename = "input_audio.end")]
    InputAudioEnd,
}
