impl SpeechStreamAudioDeltaType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SpeechAudioDelta => "speech.audio.delta",
        }
    }
}
