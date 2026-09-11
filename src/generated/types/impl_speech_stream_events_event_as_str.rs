impl SpeechStreamEventsEvent {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SpeechAudioDelta => "speech.audio.delta",
            Self::SpeechAudioDone => "speech.audio.done",
        }
    }
}
