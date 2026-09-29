pub use crate::prelude::*;

/// Attributes for workflow task failed events.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkflowTaskFailedAttributes {
    /// Details about the failure that caused the task to fail.
    #[serde(default)]
    pub failure: Failure,
    /// Unique identifier for the task within the workflow execution.
    #[serde(default)]
    pub task_id: String,
}

impl WorkflowTaskFailedAttributes {
    pub fn builder() -> WorkflowTaskFailedAttributesBuilder {
        <WorkflowTaskFailedAttributesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowTaskFailedAttributesBuilder {
    failure: Option<Failure>,
    task_id: Option<String>,
}

impl WorkflowTaskFailedAttributesBuilder {
    pub fn failure(mut self, value: Failure) -> Self {
        self.failure = Some(value);
        self
    }

    pub fn task_id(mut self, value: impl Into<String>) -> Self {
        self.task_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WorkflowTaskFailedAttributes`].
    /// This method will fail if any of the following fields are not set:
    /// - [`failure`](WorkflowTaskFailedAttributesBuilder::failure)
    /// - [`task_id`](WorkflowTaskFailedAttributesBuilder::task_id)
    pub fn build(self) -> Result<WorkflowTaskFailedAttributes, BuildError> {
        Ok(WorkflowTaskFailedAttributes {
            failure: self
                .failure
                .ok_or_else(|| BuildError::missing_field("failure"))?,
            task_id: self
                .task_id
                .ok_or_else(|| BuildError::missing_field("task_id"))?,
        })
    }
}
