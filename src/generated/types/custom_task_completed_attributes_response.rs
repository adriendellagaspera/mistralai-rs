///Attributes for custom task completed events.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CustomTaskCompletedAttributesResponse {
    ///Unique identifier for the custom task within the workflow.
    pub custom_task_id: String,
    ///The type/category of the custom task (e.g., 'llm_call', 'api_request').
    pub custom_task_type: String,
    pub payload: JSONPayloadResponse,
}
