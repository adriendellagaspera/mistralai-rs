pub use crate::prelude::*;

/// Capability for task-augmented elicitation/create requests (2025-11-25 only).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TasksCreateElicitationCapability {}

impl TasksCreateElicitationCapability {
    pub fn builder() -> TasksCreateElicitationCapabilityBuilder {
        <TasksCreateElicitationCapabilityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TasksCreateElicitationCapabilityBuilder {}

impl TasksCreateElicitationCapabilityBuilder {
    /// Consumes the builder and constructs a [`TasksCreateElicitationCapability`].
    pub fn build(self) -> Result<TasksCreateElicitationCapability, BuildError> {
        Ok(TasksCreateElicitationCapability {})
    }
}
