impl FunctionResultEntryType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::FunctionResult => "function.result",
        }
    }
}
