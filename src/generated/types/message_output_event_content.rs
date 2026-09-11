#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum MessageOutputEventContent {
    String(String),
    OutputContentChunks(OutputContentChunks),
}
