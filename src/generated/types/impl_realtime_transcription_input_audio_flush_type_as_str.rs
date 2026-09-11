impl RealtimeTranscriptionInputAudioFlushType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::InputAudioFlush => "input_audio.flush",
        }
    }
}
