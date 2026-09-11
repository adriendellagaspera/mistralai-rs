///Typed error responses for `execute_workflow_v1_workflows__workflow_identifier__execute_post`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum ExecuteWorkflowV1WorkflowsWorkflowIdentifierExecutePostApiError {
    Status422(HTTPValidationError),
}
