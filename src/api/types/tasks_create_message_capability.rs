pub use crate::prelude::*;

/// Capability for task-augmented sampling/createMessage requests (2025-11-25 only).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TasksCreateMessageCapability {}

impl TasksCreateMessageCapability {
    pub fn builder() -> TasksCreateMessageCapabilityBuilder {
        <TasksCreateMessageCapabilityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TasksCreateMessageCapabilityBuilder {}

impl TasksCreateMessageCapabilityBuilder {
    /// Consumes the builder and constructs a [`TasksCreateMessageCapability`].
    pub fn build(self) -> Result<TasksCreateMessageCapability, BuildError> {
        Ok(TasksCreateMessageCapability {})
    }
}
