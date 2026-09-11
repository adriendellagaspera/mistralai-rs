impl ToolExecutionDoneEventType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ToolExecutionDone => "tool.execution.done",
        }
    }
}
