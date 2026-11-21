pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ScorerKind {
    #[serde(rename = "llm_judge")]
    LlmJudge,
}
impl fmt::Display for ScorerKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::LlmJudge => "llm_judge",
        };
        write!(f, "{}", s)
    }
}
