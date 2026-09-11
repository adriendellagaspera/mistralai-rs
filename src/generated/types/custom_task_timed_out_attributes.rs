///Attributes for custom task timed out events.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CustomTaskTimedOutAttributes {
    ///Unique identifier for the custom task within the workflow.
    pub custom_task_id: String,
    ///The type/category of the custom task (e.g., 'llm_call', 'api_request').
    pub custom_task_type: String,
    ///The type of timeout that occurred.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub timeout_type: Option<Option<String>>,
}
