pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum WorkflowType {
    #[serde(rename = "code")]
    Code,
}
impl fmt::Display for WorkflowType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Code => "code",
        };
        write!(f, "{}", s)
    }
}
