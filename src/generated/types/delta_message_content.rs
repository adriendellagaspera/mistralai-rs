#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum DeltaMessageContent {
    String(String),
    ContentChunkArrayInline(ContentChunkArrayInline),
}
