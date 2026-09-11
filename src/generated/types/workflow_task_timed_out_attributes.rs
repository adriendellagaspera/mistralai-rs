///Attributes for workflow task timed out events.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorkflowTaskTimedOutAttributes {
    ///Unique identifier for the task within the workflow execution.
    pub task_id: String,
    ///The type of timeout that occurred (e.g., 'START_TO_CLOSE', 'SCHEDULE_TO_START').
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub timeout_type: Option<Option<String>>,
}
