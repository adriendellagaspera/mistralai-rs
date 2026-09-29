pub use crate::prelude::*;

/// Attributes for workflow execution failed events.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkflowExecutionFailedAttributes {
    /// Unique identifier for the task within the workflow execution.
    #[serde(default)]
    pub task_id: String,
    /// Details about the failure that caused the workflow to fail.
    #[serde(default)]
    pub failure: Failure,
    /// Workflow retry attempt number. 1 on first run and CAN; >1 on workflow-level retries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempt: Option<i64>,
}

impl WorkflowExecutionFailedAttributes {
    pub fn builder() -> WorkflowExecutionFailedAttributesBuilder {
        <WorkflowExecutionFailedAttributesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowExecutionFailedAttributesBuilder {
    task_id: Option<String>,
    failure: Option<Failure>,
    attempt: Option<i64>,
}

impl WorkflowExecutionFailedAttributesBuilder {
    pub fn task_id(mut self, value: impl Into<String>) -> Self {
        self.task_id = Some(value.into());
        self
    }

    pub fn failure(mut self, value: Failure) -> Self {
        self.failure = Some(value);
        self
    }

    pub fn attempt(mut self, value: i64) -> Self {
        self.attempt = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowExecutionFailedAttributes`].
    /// This method will fail if any of the following fields are not set:
    /// - [`task_id`](WorkflowExecutionFailedAttributesBuilder::task_id)
    /// - [`failure`](WorkflowExecutionFailedAttributesBuilder::failure)
    pub fn build(self) -> Result<WorkflowExecutionFailedAttributes, BuildError> {
        Ok(WorkflowExecutionFailedAttributes {
            task_id: self
                .task_id
                .ok_or_else(|| BuildError::missing_field("task_id"))?,
            failure: self
                .failure
                .ok_or_else(|| BuildError::missing_field("failure"))?,
            attempt: self.attempt,
        })
    }
}
