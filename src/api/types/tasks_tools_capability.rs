pub use crate::prelude::*;

/// Capability for task-augmented tool operations (2025-11-25 only).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TasksToolsCapability {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call: Option<TasksCallCapability>,
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
        Ok(TasksToolsCapability { call: self.call })
    }
}
