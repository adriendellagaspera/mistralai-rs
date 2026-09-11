///Typed error responses for `get_chat_completion_field_options_counts_v1_observability_chat_completion_fields__field_name__options_counts_post`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum GetChatCompletionFieldOptionsCountsV1ObservabilityChatCompletionFieldsFieldNameOptionsCountsPostApiError
{
    Status400(ObservabilityError),
    Status404(ObservabilityError),
    Status408(ObservabilityError),
    Status409(ObservabilityError),
    Status422(ObservabilityError),
}
