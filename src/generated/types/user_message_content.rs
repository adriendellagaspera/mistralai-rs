#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum UserMessageContent {
    String(String),
    ContentChunkArrayInline4AA6E9FB71A69E74(ContentChunkArrayInline4AA6E9FB71A69E74),
}
