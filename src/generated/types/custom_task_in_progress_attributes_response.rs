///Attributes for custom task in-progress events with streaming updates.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CustomTaskInProgressAttributesResponse {
    ///Unique identifier for the custom task within the workflow.
    pub custom_task_id: String,
    ///The type/category of the custom task (e.g., 'llm_call', 'api_request').
    pub custom_task_type: String,
    ///The current state or incremental update for the task.
    pub payload: CustomTaskInProgressAttributesResponsePayload,
}
