pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum BatchJobObject {
    #[serde(rename = "batch")]
    Batch,
}
impl fmt::Display for BatchJobObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Batch => "batch",
        };
        write!(f, "{}", s)
    }
}
