pub use crate::prelude::*;

/// Capability for task-augmented elicitation operations (2025-11-25 only).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TasksElicitationCapability {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create: Option<TasksCreateElicitationCapability>,
}

impl TasksElicitationCapability {
    pub fn builder() -> TasksElicitationCapabilityBuilder {
        <TasksElicitationCapabilityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TasksElicitationCapabilityBuilder {
    create: Option<TasksCreateElicitationCapability>,
}

impl TasksElicitationCapabilityBuilder {
    pub fn create(mut self, value: TasksCreateElicitationCapability) -> Self {
        self.create = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TasksElicitationCapability`].
    pub fn build(self) -> Result<TasksElicitationCapability, BuildError> {
        Ok(TasksElicitationCapability {
            create: self.create,
        })
    }
}
