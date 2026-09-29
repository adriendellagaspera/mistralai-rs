pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ToolExecutionEntryObject {
    #[serde(rename = "entry")]
    Entry,
}
impl fmt::Display for ToolExecutionEntryObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Entry => "entry",
        };
        write!(f, "{}", s)
    }
}
