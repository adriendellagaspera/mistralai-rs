pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AgentListPageObject {
    #[serde(rename = "list")]
    List,
}
impl fmt::Display for AgentListPageObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::List => "list",
        };
        write!(f, "{}", s)
    }
}
