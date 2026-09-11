///Typed error responses for `reset_workflow_v1_workflows_executions__execution_id__reset_post`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum ResetWorkflowV1WorkflowsExecutionsExecutionIdResetPostApiError {
    Status422(HTTPValidationError),
}
