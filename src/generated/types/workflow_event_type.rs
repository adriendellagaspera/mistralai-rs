#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum WorkflowEventType {
    #[default]
    #[serde(rename = "WORKFLOW_EXECUTION_STARTED")]
    WorkflowExecutionStarted,
    #[serde(rename = "WORKFLOW_EXECUTION_COMPLETED")]
    WorkflowExecutionCompleted,
    #[serde(rename = "WORKFLOW_EXECUTION_FAILED")]
    WorkflowExecutionFailed,
    #[serde(rename = "WORKFLOW_EXECUTION_CANCELED")]
    WorkflowExecutionCanceled,
    #[serde(rename = "WORKFLOW_EXECUTION_CONTINUED_AS_NEW")]
    WorkflowExecutionContinuedAsNew,
    #[serde(rename = "WORKFLOW_TASK_TIMED_OUT")]
    WorkflowTaskTimedOut,
    #[serde(rename = "WORKFLOW_TASK_FAILED")]
    WorkflowTaskFailed,
    #[serde(rename = "CUSTOM_TASK_STARTED")]
    CustomTaskStarted,
    #[serde(rename = "CUSTOM_TASK_IN_PROGRESS")]
    CustomTaskInProgress,
    #[serde(rename = "CUSTOM_TASK_COMPLETED")]
    CustomTaskCompleted,
    #[serde(rename = "CUSTOM_TASK_FAILED")]
    CustomTaskFailed,
    #[serde(rename = "CUSTOM_TASK_TIMED_OUT")]
    CustomTaskTimedOut,
    #[serde(rename = "CUSTOM_TASK_CANCELED")]
    CustomTaskCanceled,
    #[serde(rename = "ACTIVITY_TASK_STARTED")]
    ActivityTaskStarted,
    #[serde(rename = "ACTIVITY_TASK_COMPLETED")]
    ActivityTaskCompleted,
    #[serde(rename = "ACTIVITY_TASK_RETRYING")]
    ActivityTaskRetrying,
    #[serde(rename = "ACTIVITY_TASK_FAILED")]
    ActivityTaskFailed,
}
