impl ThinkChunkType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Thinking => "thinking",
        }
    }
}
