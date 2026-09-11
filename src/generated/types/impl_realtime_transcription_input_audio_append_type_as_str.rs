impl RealtimeTranscriptionInputAudioAppendType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::InputAudioAppend => "input_audio.append",
        }
    }
}
