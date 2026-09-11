///Typed error responses for `query_workflow_execution_v1_workflows_executions__execution_id__queries_post`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum QueryWorkflowExecutionV1WorkflowsExecutionsExecutionIdQueriesPostApiError {
    Status422(HTTPValidationError),
}
