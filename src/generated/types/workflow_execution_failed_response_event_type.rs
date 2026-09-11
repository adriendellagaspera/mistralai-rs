///Event type discriminator.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum WorkflowExecutionFailedResponseEventType {
    #[default]
    #[serde(rename = "WORKFLOW_EXECUTION_FAILED")]
    WorkflowExecutionFailed,
}
