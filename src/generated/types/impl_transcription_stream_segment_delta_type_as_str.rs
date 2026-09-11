impl TranscriptionStreamSegmentDeltaType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::TranscriptionSegment => "transcription.segment",
        }
    }
}
