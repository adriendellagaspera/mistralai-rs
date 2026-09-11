#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum RealtimeTranscriptionInputAudioFlushType {
    #[default]
    #[serde(rename = "input_audio.flush")]
    InputAudioFlush,
}
