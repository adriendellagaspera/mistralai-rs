impl SpeechStreamDoneType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SpeechAudioDone => "speech.audio.done",
        }
    }
}
