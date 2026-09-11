///Typed error responses for `signal_workflow_execution_v1_workflows_executions__execution_id__signals_post`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum SignalWorkflowExecutionV1WorkflowsExecutionsExecutionIdSignalsPostApiError {
    Status422(HTTPValidationError),
}
