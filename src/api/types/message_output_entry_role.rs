pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MessageOutputEntryRole {
    #[serde(rename = "assistant")]
    Assistant,
}
impl fmt::Display for MessageOutputEntryRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Assistant => "assistant",
        };
        write!(f, "{}", s)
    }
}
