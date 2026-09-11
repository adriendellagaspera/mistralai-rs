impl ToolType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Rag => "rag",
            Self::Image => "image",
            Self::Code => "code",
            Self::Event => "event",
        }
    }
}
