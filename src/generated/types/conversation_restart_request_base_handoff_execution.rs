#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ConversationRestartRequestBaseHandoffExecution {
    #[serde(rename = "client")]
    Client,
    #[default]
    #[serde(rename = "server")]
    Server,
}
