pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ToolExecutionInfo(pub HashMap<String, serde_json::Value>);
