#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SystemMessageContent {
    String(String),
    SystemMessageContentChunksArray(SystemMessageContentChunksArray),
}
