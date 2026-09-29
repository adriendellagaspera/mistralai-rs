pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ConversationResponseObject {
    #[serde(rename = "conversation.response")]
    ConversationResponse,
}
impl fmt::Display for ConversationResponseObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::ConversationResponse => "conversation.response",
        };
        write!(f, "{}", s)
    }
}
