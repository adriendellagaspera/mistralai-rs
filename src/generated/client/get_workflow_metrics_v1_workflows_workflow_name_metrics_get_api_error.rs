///Typed error responses for `get_workflow_metrics_v1_workflows__workflow_name__metrics_get`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum GetWorkflowMetricsV1WorkflowsWorkflowNameMetricsGetApiError {
    Status422(HTTPValidationError),
}
