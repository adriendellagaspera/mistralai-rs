#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChatCompletionEventPreview {
    pub correlation_id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub event_id: String,
    pub extra_fields: ChatCompletionEventPreviewExtraFields,
    pub nb_input_tokens: i64,
    pub nb_output_tokens: i64,
}
