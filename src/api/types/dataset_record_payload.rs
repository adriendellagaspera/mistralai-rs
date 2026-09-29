pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DatasetRecordPayload(pub HashMap<String, serde_json::Value>);
