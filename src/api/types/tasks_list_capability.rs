pub use crate::prelude::*;

/// Capability for tasks listing operations (2025-11-25 only).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TasksListCapability {}

impl TasksListCapability {
    pub fn builder() -> TasksListCapabilityBuilder {
        <TasksListCapabilityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TasksListCapabilityBuilder {}

impl TasksListCapabilityBuilder {
    /// Consumes the builder and constructs a [`TasksListCapability`].
    pub fn build(self) -> Result<TasksListCapability, BuildError> {
        Ok(TasksListCapability {})
    }
}
