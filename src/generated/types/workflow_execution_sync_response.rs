///Response model for synchronous workflow execution
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorkflowExecutionSyncResponse {
    ///ID of the workflow execution
    pub execution_id: String,
    ///The result of the workflow execution
    pub result: serde_json::Value,
    ///Name of the workflow that was executed
    pub workflow_name: String,
}
