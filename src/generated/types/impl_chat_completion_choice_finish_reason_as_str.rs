impl ChatCompletionChoiceFinishReason {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Stop => "stop",
            Self::Length => "length",
            Self::ModelLength => "model_length",
            Self::Error => "error",
            Self::ToolCalls => "tool_calls",
        }
    }
}
