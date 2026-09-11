impl WorkflowExecutionCanceledResponseEventType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::WorkflowExecutionCanceled => "WORKFLOW_EXECUTION_CANCELED",
        }
    }
}
