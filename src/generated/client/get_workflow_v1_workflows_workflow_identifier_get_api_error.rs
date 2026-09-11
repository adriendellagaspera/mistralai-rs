///Typed error responses for `get_workflow_v1_workflows__workflow_identifier__get`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum GetWorkflowV1WorkflowsWorkflowIdentifierGetApiError {
    Status422(HTTPValidationError),
}
