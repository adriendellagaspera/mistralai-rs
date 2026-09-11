///Specific version of the agent to use when restarting. If not provided, uses the current version.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ConversationRestartRequestBaseAgentVersion {
    String(String),
    Integer(i64),
}
