#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChatCompletionEvents {
    pub completion_events: FeedResultChatCompletionEventPreview,
}
