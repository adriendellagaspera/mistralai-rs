#[derive(Debug, Clone)]
pub enum ExecuteWorkflowRegistrationV1WorkflowsRegistrationsWorkflowRegistrationIdExecutePostResponse
{
    WorkflowExecutionResponse(WorkflowExecutionResponse),
    WorkflowExecutionSyncResponse(WorkflowExecutionSyncResponse),
}
