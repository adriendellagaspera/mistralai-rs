pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TasksCreateElicitationCapability(pub HashMap<String, serde_json::Value>);
