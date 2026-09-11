///Typed error responses for `list_runs_v1_workflows_runs_get`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum ListRunsV1WorkflowsRunsGetApiError {
    Status422(HTTPValidationError),
}
