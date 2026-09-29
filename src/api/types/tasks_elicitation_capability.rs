pub use crate::prelude::*;

/// Capability for tasks elicitation operations.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TasksElicitationCapability {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create: Option<TasksCreateElicitationCapability>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
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
            extra: Default::default(),
        })
    }
}
