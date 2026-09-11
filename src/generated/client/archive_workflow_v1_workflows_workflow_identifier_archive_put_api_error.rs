///Typed error responses for `archive_workflow_v1_workflows__workflow_identifier__archive_put`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum ArchiveWorkflowV1WorkflowsWorkflowIdentifierArchivePutApiError {
    Status422(HTTPValidationError),
}
