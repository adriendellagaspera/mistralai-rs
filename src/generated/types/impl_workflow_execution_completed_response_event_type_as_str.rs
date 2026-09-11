impl WorkflowExecutionCompletedResponseEventType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::WorkflowExecutionCompleted => "WORKFLOW_EXECUTION_COMPLETED",
        }
    }
}
