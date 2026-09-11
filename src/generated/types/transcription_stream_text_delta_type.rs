#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum TranscriptionStreamTextDeltaType {
    #[default]
    #[serde(rename = "transcription.text.delta")]
    TranscriptionTextDelta,
}
