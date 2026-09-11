impl ToolFileChunkType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ToolFile => "tool_file",
        }
    }
}
