impl TranscriptionStreamTextDeltaType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::TranscriptionTextDelta => "transcription.text.delta",
        }
    }
}
