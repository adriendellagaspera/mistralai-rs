pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AgentConversationObject {
    #[serde(rename = "conversation")]
    Conversation,
}
impl fmt::Display for AgentConversationObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Conversation => "conversation",
        };
        write!(f, "{}", s)
    }
}
