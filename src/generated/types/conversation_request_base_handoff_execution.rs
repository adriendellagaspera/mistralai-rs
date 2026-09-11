#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ConversationRequestBaseHandoffExecution {
    #[default]
    #[serde(rename = "client")]
    Client,
    #[serde(rename = "server")]
    Server,
}
