impl FunctionCallEntryType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::FunctionCall => "function.call",
        }
    }
}
