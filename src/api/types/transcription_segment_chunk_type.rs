pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TranscriptionSegmentChunkType {
    #[serde(rename = "transcription_segment")]
    TranscriptionSegment,
}
impl fmt::Display for TranscriptionSegmentChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::TranscriptionSegment => "transcription_segment",
        };
        write!(f, "{}", s)
    }
}
