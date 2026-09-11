///Event type discriminator.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum WorkflowTaskFailedResponseEventType {
    #[default]
    #[serde(rename = "WORKFLOW_TASK_FAILED")]
    WorkflowTaskFailed,
}
