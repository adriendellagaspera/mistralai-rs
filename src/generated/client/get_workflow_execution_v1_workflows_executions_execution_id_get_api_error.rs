///Typed error responses for `get_workflow_execution_v1_workflows_executions__execution_id__get`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum GetWorkflowExecutionV1WorkflowsExecutionsExecutionIdGetApiError {
    Status422(HTTPValidationError),
}
