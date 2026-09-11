#[derive(Debug, Clone)]
pub enum ExecuteWorkflowV1WorkflowsWorkflowIdentifierExecutePostResponse {
    WorkflowExecutionResponse(WorkflowExecutionResponse),
    WorkflowExecutionSyncResponse(WorkflowExecutionSyncResponse),
}
