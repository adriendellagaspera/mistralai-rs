///Typed error responses for `stream_v1_workflows_executions__execution_id__stream_get`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum StreamV1WorkflowsExecutionsExecutionIdStreamGetApiError {
    Status422(HTTPValidationError),
}
