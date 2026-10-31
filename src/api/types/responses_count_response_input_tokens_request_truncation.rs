pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CountResponseInputTokensRequestTruncation {
    #[serde(rename = "disabled")]
    Disabled,
}
impl fmt::Display for CountResponseInputTokensRequestTruncation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Disabled => "disabled",
        };
        write!(f, "{}", s)
    }
}
