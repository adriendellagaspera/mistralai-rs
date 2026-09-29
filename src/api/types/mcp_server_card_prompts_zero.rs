pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum McpServerCardPromptsZero {
    #[serde(rename = "dynamic")]
    Dynamic,
}
impl fmt::Display for McpServerCardPromptsZero {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Dynamic => "dynamic",
        };
        write!(f, "{}", s)
    }
}
