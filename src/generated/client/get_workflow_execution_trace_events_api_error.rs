///Typed error responses for `get_workflow_execution_trace_events`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum GetWorkflowExecutionTraceEventsApiError {
    Status422(HTTPValidationError),
}
