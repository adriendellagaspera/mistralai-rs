pub use crate::prelude::*;

/// Attributes for workflow execution failed events.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkflowExecutionFailedAttributes {
    /// Workflow retry attempt number. 1 on first run and CAN; >1 on workflow-level retries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempt: Option<i64>,
    /// Details about the failure that caused the workflow to fail.
    #[serde(default)]
    pub failure: Failure,
    /// Unique identifier for the task within the workflow execution.
    #[serde(default)]
    pub task_id: String,
}

impl WorkflowExecutionFailedAttributes {
    pub fn builder() -> WorkflowExecutionFailedAttributesBuilder {
        <WorkflowExecutionFailedAttributesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowExecutionFailedAttributesBuilder {
    attempt: Option<i64>,
    failure: Option<Failure>,
    task_id: Option<String>,
}

impl WorkflowExecutionFailedAttributesBuilder {
    pub fn attempt(mut self, value: i64) -> Self {
        self.attempt = Some(value);
        self
    }

    pub fn failure(mut self, value: Failure) -> Self {
        self.failure = Some(value);
        self
    }

    pub fn task_id(mut self, value: impl Into<String>) -> Self {
        self.task_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WorkflowExecutionFailedAttributes`].
    /// This method will fail if any of the following fields are not set:
    /// - [`failure`](WorkflowExecutionFailedAttributesBuilder::failure)
    /// - [`task_id`](WorkflowExecutionFailedAttributesBuilder::task_id)
    pub fn build(self) -> Result<WorkflowExecutionFailedAttributes, BuildError> {
        Ok(WorkflowExecutionFailedAttributes {
            attempt: self.attempt,
            failure: self
                .failure
                .ok_or_else(|| BuildError::missing_field("failure"))?,
            task_id: self
                .task_id
                .ok_or_else(|| BuildError::missing_field("task_id"))?,
        })
    }
}
