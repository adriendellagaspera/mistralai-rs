#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum MessageOutputEntryContent {
    String(String),
    MessageOutputContentChunks(MessageOutputContentChunks),
}
