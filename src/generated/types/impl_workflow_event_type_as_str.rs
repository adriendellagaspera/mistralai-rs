impl WorkflowEventType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::WorkflowExecutionStarted => "WORKFLOW_EXECUTION_STARTED",
            Self::WorkflowExecutionCompleted => "WORKFLOW_EXECUTION_COMPLETED",
            Self::WorkflowExecutionFailed => "WORKFLOW_EXECUTION_FAILED",
            Self::WorkflowExecutionCanceled => "WORKFLOW_EXECUTION_CANCELED",
            Self::WorkflowExecutionContinuedAsNew => "WORKFLOW_EXECUTION_CONTINUED_AS_NEW",
            Self::WorkflowTaskTimedOut => "WORKFLOW_TASK_TIMED_OUT",
            Self::WorkflowTaskFailed => "WORKFLOW_TASK_FAILED",
            Self::CustomTaskStarted => "CUSTOM_TASK_STARTED",
            Self::CustomTaskInProgress => "CUSTOM_TASK_IN_PROGRESS",
            Self::CustomTaskCompleted => "CUSTOM_TASK_COMPLETED",
            Self::CustomTaskFailed => "CUSTOM_TASK_FAILED",
            Self::CustomTaskTimedOut => "CUSTOM_TASK_TIMED_OUT",
            Self::CustomTaskCanceled => "CUSTOM_TASK_CANCELED",
            Self::ActivityTaskStarted => "ACTIVITY_TASK_STARTED",
            Self::ActivityTaskCompleted => "ACTIVITY_TASK_COMPLETED",
            Self::ActivityTaskRetrying => "ACTIVITY_TASK_RETRYING",
            Self::ActivityTaskFailed => "ACTIVITY_TASK_FAILED",
        }
    }
}
