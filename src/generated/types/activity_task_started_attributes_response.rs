///Attributes for activity task started events.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ActivityTaskStartedAttributesResponse {
    ///The registered name of the activity being executed.
    pub activity_name: String,
    pub input: JSONPayloadResponse,
    ///Unique identifier for the activity task within the workflow.
    pub task_id: String,
}
