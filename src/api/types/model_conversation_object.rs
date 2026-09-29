pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ModelConversationObject {
    #[serde(rename = "conversation")]
    Conversation,
}
impl fmt::Display for ModelConversationObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Conversation => "conversation",
        };
        write!(f, "{}", s)
    }
}
