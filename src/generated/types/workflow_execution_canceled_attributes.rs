///Attributes for workflow execution canceled events.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorkflowExecutionCanceledAttributes {
    ///Optional reason provided for the cancellation.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub reason: Option<Option<String>>,
    ///Unique identifier for the task within the workflow execution.
    pub task_id: String,
}
