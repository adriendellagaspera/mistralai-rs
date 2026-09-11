#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ConversationRequestBaseAgentVersion {
    String(String),
    Integer(i64),
}
