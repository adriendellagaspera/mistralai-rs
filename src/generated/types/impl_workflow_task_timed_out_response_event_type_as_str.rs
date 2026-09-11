impl WorkflowTaskTimedOutResponseEventType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::WorkflowTaskTimedOut => "WORKFLOW_TASK_TIMED_OUT",
        }
    }
}
