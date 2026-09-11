///Typed error responses for `update_workflow_execution_v1_workflows_executions__execution_id__updates_post`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum UpdateWorkflowExecutionV1WorkflowsExecutionsExecutionIdUpdatesPostApiError {
    Status422(HTTPValidationError),
}
