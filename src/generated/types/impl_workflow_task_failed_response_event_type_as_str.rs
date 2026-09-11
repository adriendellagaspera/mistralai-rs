impl WorkflowTaskFailedResponseEventType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::WorkflowTaskFailed => "WORKFLOW_TASK_FAILED",
        }
    }
}
