impl ToolReferenceChunkType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ToolReference => "tool_reference",
        }
    }
}
