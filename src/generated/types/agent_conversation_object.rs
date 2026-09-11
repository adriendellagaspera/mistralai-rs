#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum AgentConversationObject {
    #[default]
    #[serde(rename = "conversation")]
    Conversation,
}
