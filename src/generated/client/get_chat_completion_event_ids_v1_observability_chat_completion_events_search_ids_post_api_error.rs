///Typed error responses for `get_chat_completion_event_ids_v1_observability_chat_completion_events_search_ids_post`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum GetChatCompletionEventIdsV1ObservabilityChatCompletionEventsSearchIdsPostApiError {
    Status400(ObservabilityError),
    Status404(ObservabilityError),
    Status408(ObservabilityError),
    Status409(ObservabilityError),
    Status422(ObservabilityError),
}
