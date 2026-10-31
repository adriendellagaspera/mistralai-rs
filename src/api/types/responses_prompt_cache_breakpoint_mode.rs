pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ResponsesPromptCacheBreakpointMode {
    #[serde(rename = "explicit")]
    Explicit,
}
impl fmt::Display for ResponsesPromptCacheBreakpointMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Explicit => "explicit",
        };
        write!(f, "{}", s)
    }
}
