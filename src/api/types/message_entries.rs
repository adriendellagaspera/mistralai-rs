pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct MessageEntries(pub Vec<MessageEntriesItem>);
