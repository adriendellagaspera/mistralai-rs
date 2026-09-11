///Attributes for workflow execution completed events.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorkflowExecutionCompletedAttributesResponse {
    pub result: JSONPayloadResponse,
    ///Unique identifier for the task within the workflow execution.
    pub task_id: String,
}
