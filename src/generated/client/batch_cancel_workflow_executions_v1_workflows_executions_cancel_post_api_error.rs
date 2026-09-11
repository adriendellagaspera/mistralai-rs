///Typed error responses for `batch_cancel_workflow_executions_v1_workflows_executions_cancel_post`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum BatchCancelWorkflowExecutionsV1WorkflowsExecutionsCancelPostApiError {
    Status422(HTTPValidationError),
}
