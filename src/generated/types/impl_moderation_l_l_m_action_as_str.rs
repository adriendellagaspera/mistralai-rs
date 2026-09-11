impl ModerationLLMAction {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Block => "block",
        }
    }
}
