///Typed error responses for `get_workflow_events_v1_workflows_events_list_get`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum GetWorkflowEventsV1WorkflowsEventsListGetApiError {
    Status422(HTTPValidationError),
}
