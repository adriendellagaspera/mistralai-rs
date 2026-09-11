impl ToolChoiceEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::None => "none",
            Self::Any => "any",
            Self::Required => "required",
        }
    }
}
