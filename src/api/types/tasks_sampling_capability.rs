pub use crate::prelude::*;

/// Capability for task-augmented sampling operations (2025-11-25 only).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TasksSamplingCapability {
    #[serde(rename = "createMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_message: Option<TasksCreateMessageCapability>,
}

impl TasksSamplingCapability {
    pub fn builder() -> TasksSamplingCapabilityBuilder {
        <TasksSamplingCapabilityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TasksSamplingCapabilityBuilder {
    create_message: Option<TasksCreateMessageCapability>,
}

impl TasksSamplingCapabilityBuilder {
    pub fn create_message(mut self, value: TasksCreateMessageCapability) -> Self {
        self.create_message = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TasksSamplingCapability`].
    pub fn build(self) -> Result<TasksSamplingCapability, BuildError> {
        Ok(TasksSamplingCapability {
            create_message: self.create_message,
        })
    }
}
