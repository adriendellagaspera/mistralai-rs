pub use crate::prelude::*;

/// Capability for task-augmented tools/call requests (2025-11-25 only).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TasksCallCapability {}

impl TasksCallCapability {
    pub fn builder() -> TasksCallCapabilityBuilder {
        <TasksCallCapabilityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TasksCallCapabilityBuilder {}

impl TasksCallCapabilityBuilder {
    /// Consumes the builder and constructs a [`TasksCallCapability`].
    pub fn build(self) -> Result<TasksCallCapability, BuildError> {
        Ok(TasksCallCapability {})
    }
}
