#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ModelConversationObject {
    #[default]
    #[serde(rename = "conversation")]
    Conversation,
}
