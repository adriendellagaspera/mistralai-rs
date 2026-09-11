///Typed error responses for `get_stream_events_v1_workflows_events_stream_get`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum GetStreamEventsV1WorkflowsEventsStreamGetApiError {
    Status422(HTTPValidationError),
}
