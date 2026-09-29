pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ConversationMessagesObject {
    #[serde(rename = "conversation.messages")]
    ConversationMessages,
}
impl fmt::Display for ConversationMessagesObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::ConversationMessages => "conversation.messages",
        };
        write!(f, "{}", s)
    }
}
