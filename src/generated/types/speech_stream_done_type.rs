#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum SpeechStreamDoneType {
    #[default]
    #[serde(rename = "speech.audio.done")]
    SpeechAudioDone,
}
