pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ResponsesReasoningItemType {
    #[serde(rename = "reasoning")]
    Reasoning,
}
impl fmt::Display for ResponsesReasoningItemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Reasoning => "reasoning",
        };
        write!(f, "{}", s)
    }
}
