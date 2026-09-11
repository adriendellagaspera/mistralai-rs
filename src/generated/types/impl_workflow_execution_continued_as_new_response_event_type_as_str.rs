impl WorkflowExecutionContinuedAsNewResponseEventType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::WorkflowExecutionContinuedAsNew => "WORKFLOW_EXECUTION_CONTINUED_AS_NEW",
        }
    }
}
