#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum SpeechStreamEventsEvent {
    #[default]
    #[serde(rename = "speech.audio.delta")]
    SpeechAudioDelta,
    #[serde(rename = "speech.audio.done")]
    SpeechAudioDone,
}
