///Typed error responses for `get_chat_completion_event_v1_observability_chat_completion_events__event_id__get`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum GetChatCompletionEventV1ObservabilityChatCompletionEventsEventIdGetApiError {
    Status400(ObservabilityError),
    Status404(ObservabilityError),
    Status408(ObservabilityError),
    Status409(ObservabilityError),
    Status422(ObservabilityError),
}
