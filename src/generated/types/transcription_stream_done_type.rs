#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum TranscriptionStreamDoneType {
    #[default]
    #[serde(rename = "transcription.done")]
    TranscriptionDone,
}
