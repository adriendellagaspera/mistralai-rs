impl WorkflowExecutionStartedResponseEventType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::WorkflowExecutionStarted => "WORKFLOW_EXECUTION_STARTED",
        }
    }
}
