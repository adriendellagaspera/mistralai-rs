///Event type discriminator.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum WorkflowExecutionCompletedResponseEventType {
    #[default]
    #[serde(rename = "WORKFLOW_EXECUTION_COMPLETED")]
    WorkflowExecutionCompleted,
}
