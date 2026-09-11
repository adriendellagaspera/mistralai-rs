///Typed error responses for `get_workflow_execution_trace_summary`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum GetWorkflowExecutionTraceSummaryApiError {
    Status422(HTTPValidationError),
}
