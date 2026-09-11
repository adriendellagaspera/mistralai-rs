///Typed error responses for `get_similar_chat_completion_events_v1_observability_chat_completion_events__event_id__similar_events_get`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum GetSimilarChatCompletionEventsV1ObservabilityChatCompletionEventsEventIdSimilarEventsGetApiError
{
    Status400(ObservabilityError),
    Status404(ObservabilityError),
    Status408(ObservabilityError),
    Status409(ObservabilityError),
    Status422(ObservabilityError),
}
