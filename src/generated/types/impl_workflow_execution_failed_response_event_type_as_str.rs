impl WorkflowExecutionFailedResponseEventType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::WorkflowExecutionFailed => "WORKFLOW_EXECUTION_FAILED",
        }
    }
}
