pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ArchiveModelResponseObject {
    #[serde(rename = "model")]
    Model,
}
impl fmt::Display for ArchiveModelResponseObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Model => "model",
        };
        write!(f, "{}", s)
    }
}
