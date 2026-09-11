impl ToolExecutionStartedEventType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ToolExecutionStarted => "tool.execution.started",
        }
    }
}
