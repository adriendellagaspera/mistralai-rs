pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AgentObject {
    #[serde(rename = "agent")]
    Agent,
}
impl fmt::Display for AgentObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Agent => "agent",
        };
        write!(f, "{}", s)
    }
}
