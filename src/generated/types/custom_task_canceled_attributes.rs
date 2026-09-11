///Attributes for custom task canceled events.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CustomTaskCanceledAttributes {
    ///Unique identifier for the custom task within the workflow.
    pub custom_task_id: String,
    ///The type/category of the custom task (e.g., 'llm_call', 'api_request').
    pub custom_task_type: String,
    ///Optional reason provided for the cancellation.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub reason: Option<Option<String>>,
}
