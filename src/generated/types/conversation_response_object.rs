#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ConversationResponseObject {
    #[default]
    #[serde(rename = "conversation.response")]
    ConversationResponse,
}
