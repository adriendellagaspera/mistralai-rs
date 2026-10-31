pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CountResponseInputTokensResponseObject {
    #[serde(rename = "response.input_tokens")]
    ResponseInputTokens,
}
impl fmt::Display for CountResponseInputTokensResponseObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::ResponseInputTokens => "response.input_tokens",
        };
        write!(f, "{}", s)
    }
}
