///Attributes for workflow execution continued-as-new events.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorkflowExecutionContinuedAsNewAttributesResponse {
    pub input: JSONPayloadResponse,
    ///The run ID of the new workflow execution that continues this workflow.
    pub new_execution_run_id: String,
    ///Unique identifier for the task within the workflow execution.
    pub task_id: String,
    ///The registered name of the continued workflow.
    pub workflow_name: String,
}
