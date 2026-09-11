#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum TranscriptionStreamSegmentDeltaType {
    #[default]
    #[serde(rename = "transcription.segment")]
    TranscriptionSegment,
}
