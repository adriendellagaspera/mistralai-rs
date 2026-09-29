pub use crate::prelude::*;

/// Capability for tasks tools operations.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TasksToolsCapability {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call: Option<TasksCallCapability>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl TasksToolsCapability {
    pub fn builder() -> TasksToolsCapabilityBuilder {
        <TasksToolsCapabilityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TasksToolsCapabilityBuilder {
    call: Option<TasksCallCapability>,
}

impl TasksToolsCapabilityBuilder {
    pub fn call(mut self, value: TasksCallCapability) -> Self {
        self.call = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TasksToolsCapability`].
    pub fn build(self) -> Result<TasksToolsCapability, BuildError> {
        Ok(TasksToolsCapability {
            call: self.call,
            extra: Default::default(),
        })
    }
}
