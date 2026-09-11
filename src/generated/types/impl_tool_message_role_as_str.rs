impl ToolMessageRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Tool => "tool",
        }
    }
}
