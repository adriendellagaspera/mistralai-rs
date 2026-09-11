#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum TranscriptionStreamEventTypes {
    #[default]
    #[serde(rename = "transcription.language")]
    TranscriptionLanguage,
    #[serde(rename = "transcription.segment")]
    TranscriptionSegment,
    #[serde(rename = "transcription.text.delta")]
    TranscriptionTextDelta,
    #[serde(rename = "transcription.done")]
    TranscriptionDone,
}
