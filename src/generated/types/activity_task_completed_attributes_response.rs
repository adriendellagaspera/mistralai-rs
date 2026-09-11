///Attributes for activity task completed events.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ActivityTaskCompletedAttributesResponse {
    ///The registered name of the activity being executed.
    pub activity_name: String,
    pub result: JSONPayloadResponse,
    ///Unique identifier for the activity task within the workflow.
    pub task_id: String,
}
