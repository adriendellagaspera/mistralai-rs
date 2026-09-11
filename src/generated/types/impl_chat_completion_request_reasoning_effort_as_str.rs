impl ChatCompletionRequestReasoningEffort {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::High => "high",
            Self::None => "none",
        }
    }
}
