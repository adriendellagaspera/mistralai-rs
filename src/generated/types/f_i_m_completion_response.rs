#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FIMCompletionResponse {
    pub choices: Vec<ChatCompletionChoice>,
    pub created: i64,
    pub id: String,
    pub model: String,
    pub object: String,
    pub usage: UsageInfo,
}
