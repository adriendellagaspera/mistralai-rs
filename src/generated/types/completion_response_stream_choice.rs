#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CompletionResponseStreamChoice {
    pub delta: DeltaMessage,
    pub finish_reason: Option<CompletionResponseStreamChoiceFinishReason>,
    pub index: i64,
}
