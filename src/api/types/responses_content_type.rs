pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ResponsesContentType {
    #[serde(rename = "reasoning_text")]
    ReasoningText,
}
impl fmt::Display for ResponsesContentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::ReasoningText => "reasoning_text",
        };
        write!(f, "{}", s)
    }
}
