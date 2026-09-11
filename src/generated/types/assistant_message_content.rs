#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum AssistantMessageContent {
    String(String),
    ContentChunkArray(ContentChunkArray),
}
