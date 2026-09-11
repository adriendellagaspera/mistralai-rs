#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ResponseDoneEventType {
    #[default]
    #[serde(rename = "conversation.response.done")]
    ConversationResponseDone,
}
