///Typed error responses for `unarchive_workflow_v1_workflows__workflow_identifier__unarchive_put`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum UnarchiveWorkflowV1WorkflowsWorkflowIdentifierUnarchivePutApiError {
    Status422(HTTPValidationError),
}
