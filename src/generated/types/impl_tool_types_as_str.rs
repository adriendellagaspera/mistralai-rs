impl ToolTypes {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Function => "function",
        }
    }
}
