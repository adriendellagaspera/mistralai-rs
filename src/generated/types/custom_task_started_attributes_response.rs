///Attributes for custom task started events.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CustomTaskStartedAttributesResponse {
    ///Unique identifier for the custom task within the workflow.
    pub custom_task_id: String,
    ///The type/category of the custom task (e.g., 'llm_call', 'api_request').
    pub custom_task_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<JSONPayloadResponse>,
}
