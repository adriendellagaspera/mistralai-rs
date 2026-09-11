#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum MessageInputEntryContent {
    String(String),
    MessageInputContentChunks(MessageInputContentChunks),
}
