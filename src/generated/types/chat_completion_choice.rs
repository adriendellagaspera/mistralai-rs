#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChatCompletionChoice {
    pub finish_reason: ChatCompletionChoiceFinishReason,
    pub index: i64,
    pub message: AssistantMessage,
}
