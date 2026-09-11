///Typed error responses for `get_workflow_execution_trace_otel`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum GetWorkflowExecutionTraceOtelApiError {
    Status422(HTTPValidationError),
}
