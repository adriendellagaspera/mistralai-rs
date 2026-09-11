///Event type discriminator.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum WorkflowExecutionStartedResponseEventType {
    #[default]
    #[serde(rename = "WORKFLOW_EXECUTION_STARTED")]
    WorkflowExecutionStarted,
}
