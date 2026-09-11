#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChatModerationRequest {
    ///Chat to classify
    pub input: ChatModerationRequestInput,
    pub model: String,
}
