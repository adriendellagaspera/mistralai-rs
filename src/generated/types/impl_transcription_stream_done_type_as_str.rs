impl TranscriptionStreamDoneType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::TranscriptionDone => "transcription.done",
        }
    }
}
