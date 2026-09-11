///Typed error responses for `get_workflow_execution_history_v1_workflows_executions__execution_id__history_get`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum GetWorkflowExecutionHistoryV1WorkflowsExecutionsExecutionIdHistoryGetApiError {
    Status422(HTTPValidationError),
}
