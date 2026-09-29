pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TasksCreateMessageCapability(pub HashMap<String, serde_json::Value>);
