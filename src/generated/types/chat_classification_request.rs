#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChatClassificationRequest {
    pub input: ChatClassificationRequestInputs,
    pub model: String,
}
