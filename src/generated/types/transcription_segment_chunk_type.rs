#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum TranscriptionSegmentChunkType {
    #[default]
    #[serde(rename = "transcription_segment")]
    TranscriptionSegment,
}
