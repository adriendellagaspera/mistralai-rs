///Typed error responses for `get_run_history_v1_workflows_runs__run_id__history_get`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum GetRunHistoryV1WorkflowsRunsRunIdHistoryGetApiError {
    Status422(HTTPValidationError),
}
