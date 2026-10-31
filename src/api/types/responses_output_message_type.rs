pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ResponsesOutputMessageType {
    #[serde(rename = "message")]
    Message,
}
impl fmt::Display for ResponsesOutputMessageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Message => "message",
        };
        write!(f, "{}", s)
    }
}
