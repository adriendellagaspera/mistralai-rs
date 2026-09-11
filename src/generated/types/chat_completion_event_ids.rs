#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChatCompletionEventIds {
    pub completion_event_ids: Vec<String>,
}
