///Typed error responses for `batch_terminate_workflow_executions_v1_workflows_executions_terminate_post`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum BatchTerminateWorkflowExecutionsV1WorkflowsExecutionsTerminatePostApiError {
    Status422(HTTPValidationError),
}
