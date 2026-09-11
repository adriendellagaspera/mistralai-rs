impl ToolExecutionEntryType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ToolExecution => "tool.execution",
        }
    }
}
