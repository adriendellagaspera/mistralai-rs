///Typed error responses for `get_run_v1_workflows_runs__run_id__get`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum GetRunV1WorkflowsRunsRunIdGetApiError {
    Status422(HTTPValidationError),
}
