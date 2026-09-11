#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChatCompletionEvent {
    pub chat_transcription_events: Vec<ChatTranscriptionEvent>,
    pub correlation_id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub enabled_tools: Vec<ChatCompletionEventEnabledToolsItem>,
    pub event_id: String,
    pub extra_fields: ChatCompletionEventExtraFields,
    pub nb_input_tokens: i64,
    pub nb_messages: i64,
    pub nb_output_tokens: i64,
    pub request_messages: Vec<ChatCompletionEventRequestMessagesItem>,
    pub response_messages: Vec<ChatCompletionEventResponseMessagesItem>,
}
