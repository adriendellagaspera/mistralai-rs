impl ToolExecutionDeltaEventType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ToolExecutionDelta => "tool.execution.delta",
        }
    }
}
