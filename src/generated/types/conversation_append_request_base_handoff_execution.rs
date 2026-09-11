#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ConversationAppendRequestBaseHandoffExecution {
    #[serde(rename = "client")]
    Client,
    #[default]
    #[serde(rename = "server")]
    Server,
}
