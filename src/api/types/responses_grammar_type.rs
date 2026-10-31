pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ResponsesGrammarType {
    #[serde(rename = "grammar")]
    Grammar,
}
impl fmt::Display for ResponsesGrammarType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Grammar => "grammar",
        };
        write!(f, "{}", s)
    }
}
