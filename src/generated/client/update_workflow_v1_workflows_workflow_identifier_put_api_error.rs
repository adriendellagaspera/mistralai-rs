///Typed error responses for `update_workflow_v1_workflows__workflow_identifier__put`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum UpdateWorkflowV1WorkflowsWorkflowIdentifierPutApiError {
    Status422(HTTPValidationError),
}
