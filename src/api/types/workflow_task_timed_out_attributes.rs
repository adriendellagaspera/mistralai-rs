pub use crate::prelude::*;

/// Attributes for workflow task timed out events.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkflowTaskTimedOutAttributes {
    /// Unique identifier for the task within the workflow execution.
    #[serde(default)]
    pub task_id: String,
    /// The type of timeout that occurred (e.g., 'START_TO_CLOSE', 'SCHEDULE_TO_START').
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_type: Option<String>,
}

impl WorkflowTaskTimedOutAttributes {
    pub fn builder() -> WorkflowTaskTimedOutAttributesBuilder {
        <WorkflowTaskTimedOutAttributesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowTaskTimedOutAttributesBuilder {
    task_id: Option<String>,
    timeout_type: Option<String>,
}

impl WorkflowTaskTimedOutAttributesBuilder {
    pub fn task_id(mut self, value: impl Into<String>) -> Self {
        self.task_id = Some(value.into());
        self
    }

    pub fn timeout_type(mut self, value: impl Into<String>) -> Self {
        self.timeout_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WorkflowTaskTimedOutAttributes`].
    /// This method will fail if any of the following fields are not set:
    /// - [`task_id`](WorkflowTaskTimedOutAttributesBuilder::task_id)
    pub fn build(self) -> Result<WorkflowTaskTimedOutAttributes, BuildError> {
        Ok(WorkflowTaskTimedOutAttributes {
            task_id: self
                .task_id
                .ok_or_else(|| BuildError::missing_field("task_id"))?,
            timeout_type: self.timeout_type,
        })
    }
}
