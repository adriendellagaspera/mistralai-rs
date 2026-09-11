///Typed error responses for `get_chat_completion_fields_v1_observability_chat_completion_fields_get`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum GetChatCompletionFieldsV1ObservabilityChatCompletionFieldsGetApiError {
    Status400(ObservabilityError),
    Status404(ObservabilityError),
    Status408(ObservabilityError),
    Status409(ObservabilityError),
    Status422(ObservabilityError),
}
