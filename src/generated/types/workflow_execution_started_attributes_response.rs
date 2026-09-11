///Attributes for workflow execution started events.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorkflowExecutionStartedAttributesResponse {
    pub input: JSONPayloadResponse,
    ///Unique identifier for the task within the workflow execution.
    pub task_id: String,
    ///The registered name of the workflow being executed.
    pub workflow_name: String,
}
