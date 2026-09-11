///Typed error responses for `cancel_workflow_execution_v1_workflows_executions__execution_id__cancel_post`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum CancelWorkflowExecutionV1WorkflowsExecutionsExecutionIdCancelPostApiError {
    Status422(HTTPValidationError),
}
