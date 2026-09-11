///Event type discriminator.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum WorkflowTaskTimedOutResponseEventType {
    #[default]
    #[serde(rename = "WORKFLOW_TASK_TIMED_OUT")]
    WorkflowTaskTimedOut,
}
