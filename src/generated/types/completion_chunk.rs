#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CompletionChunk {
    pub choices: Vec<CompletionResponseStreamChoice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<i64>,
    pub id: String,
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<UsageInfo>,
}
