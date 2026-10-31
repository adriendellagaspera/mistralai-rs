pub use crate::prelude::*;

/// Capability for tasks cancel operations (2025-11-25 only).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TasksCancelCapability {}

impl TasksCancelCapability {
    pub fn builder() -> TasksCancelCapabilityBuilder {
        <TasksCancelCapabilityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TasksCancelCapabilityBuilder {}

impl TasksCancelCapabilityBuilder {
    /// Consumes the builder and constructs a [`TasksCancelCapability`].
    pub fn build(self) -> Result<TasksCancelCapability, BuildError> {
        Ok(TasksCancelCapability {})
    }
}
