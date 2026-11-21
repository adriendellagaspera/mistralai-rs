pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum VoiceAgentToolCallEndedEventPhase {
    #[serde(rename = "ended")]
    Ended,
}
impl fmt::Display for VoiceAgentToolCallEndedEventPhase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Ended => "ended",
        };
        write!(f, "{}", s)
    }
}
