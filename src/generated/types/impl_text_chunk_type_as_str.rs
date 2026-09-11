impl TextChunkType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Text => "text",
        }
    }
}
