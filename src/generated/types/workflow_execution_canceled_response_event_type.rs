///Event type discriminator.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum WorkflowExecutionCanceledResponseEventType {
    #[default]
    #[serde(rename = "WORKFLOW_EXECUTION_CANCELED")]
    WorkflowExecutionCanceled,
}
