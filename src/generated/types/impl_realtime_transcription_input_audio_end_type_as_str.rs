impl RealtimeTranscriptionInputAudioEndType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::InputAudioEnd => "input_audio.end",
        }
    }
}
