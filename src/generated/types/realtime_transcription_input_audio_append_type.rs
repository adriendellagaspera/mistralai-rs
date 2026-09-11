#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum RealtimeTranscriptionInputAudioAppendType {
    #[default]
    #[serde(rename = "input_audio.append")]
    InputAudioAppend,
}
