///Typed error responses for `terminate_workflow_execution_v1_workflows_executions__execution_id__terminate_post`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum TerminateWorkflowExecutionV1WorkflowsExecutionsExecutionIdTerminatePostApiError {
    Status422(HTTPValidationError),
}
