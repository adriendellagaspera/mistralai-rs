#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum SpeechStreamAudioDeltaType {
    #[default]
    #[serde(rename = "speech.audio.delta")]
    SpeechAudioDelta,
}
