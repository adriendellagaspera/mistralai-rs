pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum VoiceAgentFunctionToolType {
    #[serde(rename = "function")]
    Function,
}
impl fmt::Display for VoiceAgentFunctionToolType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Function => "function",
        };
        write!(f, "{}", s)
    }
}
