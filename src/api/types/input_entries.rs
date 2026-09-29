pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct InputEntries(pub Vec<InputEntriesItem>);
