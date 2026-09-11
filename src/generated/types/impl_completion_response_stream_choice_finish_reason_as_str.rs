impl CompletionResponseStreamChoiceFinishReason {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Stop => "stop",
            Self::Length => "length",
            Self::Error => "error",
            Self::ToolCalls => "tool_calls",
            Self::NullValue => "null",
        }
    }
}
