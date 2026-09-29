pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ConversationHistoryObject {
    #[serde(rename = "conversation.history")]
    ConversationHistory,
}
impl fmt::Display for ConversationHistoryObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::ConversationHistory => "conversation.history",
        };
        write!(f, "{}", s)
    }
}
