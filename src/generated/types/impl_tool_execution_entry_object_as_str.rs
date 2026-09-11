impl ToolExecutionEntryObject {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Entry => "entry",
        }
    }
}
