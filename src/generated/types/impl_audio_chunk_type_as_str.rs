impl AudioChunkType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::InputAudio => "input_audio",
        }
    }
}
