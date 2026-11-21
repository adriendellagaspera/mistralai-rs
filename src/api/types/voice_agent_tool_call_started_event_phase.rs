pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum VoiceAgentToolCallStartedEventPhase {
    #[serde(rename = "started")]
    Started,
}
impl fmt::Display for VoiceAgentToolCallStartedEventPhase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Started => "started",
        };
        write!(f, "{}", s)
    }
}
