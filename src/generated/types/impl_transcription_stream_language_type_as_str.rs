impl TranscriptionStreamLanguageType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::TranscriptionLanguage => "transcription.language",
        }
    }
}
