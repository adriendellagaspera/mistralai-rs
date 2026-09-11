impl TranscriptionStreamEventTypes {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::TranscriptionLanguage => "transcription.language",
            Self::TranscriptionSegment => "transcription.segment",
            Self::TranscriptionTextDelta => "transcription.text.delta",
            Self::TranscriptionDone => "transcription.done",
        }
    }
}
