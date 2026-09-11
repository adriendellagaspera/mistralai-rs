///Event type discriminator.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum WorkflowExecutionContinuedAsNewResponseEventType {
    #[default]
    #[serde(rename = "WORKFLOW_EXECUTION_CONTINUED_AS_NEW")]
    WorkflowExecutionContinuedAsNew,
}
