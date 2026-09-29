pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MessageOutputEntryObject {
    #[serde(rename = "entry")]
    Entry,
}
impl fmt::Display for MessageOutputEntryObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Entry => "entry",
        };
        write!(f, "{}", s)
    }
}
