pub use crate::prelude::*;

/// Available options to the prompt_mode argument on the chat completion endpoint.
/// Values represent high-level intent. Assignment to actual SPs is handled internally.
/// System prompt may include knowledge cutoff date, model capabilities, tone to use, safety guidelines, etc.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MistralPromptMode {
    #[serde(rename = "reasoning")]
    Reasoning,
}
impl fmt::Display for MistralPromptMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Reasoning => "reasoning",
        };
        write!(f, "{}", s)
    }
}
