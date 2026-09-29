pub use crate::prelude::*;

/// Attributes for workflow execution canceled events.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkflowExecutionCanceledAttributes {
    /// Workflow retry attempt number. 1 on first run and CAN; >1 on workflow-level retries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempt: Option<i64>,
    /// Optional reason provided for the cancellation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Unique identifier for the task within the workflow execution.
    #[serde(default)]
    pub task_id: String,
}

impl WorkflowExecutionCanceledAttributes {
    pub fn builder() -> WorkflowExecutionCanceledAttributesBuilder {
        <WorkflowExecutionCanceledAttributesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowExecutionCanceledAttributesBuilder {
    attempt: Option<i64>,
    reason: Option<String>,
    task_id: Option<String>,
}

impl WorkflowExecutionCanceledAttributesBuilder {
    pub fn attempt(mut self, value: i64) -> Self {
        self.attempt = Some(value);
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn task_id(mut self, value: impl Into<String>) -> Self {
        self.task_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WorkflowExecutionCanceledAttributes`].
    /// This method will fail if any of the following fields are not set:
    /// - [`task_id`](WorkflowExecutionCanceledAttributesBuilder::task_id)
    pub fn build(self) -> Result<WorkflowExecutionCanceledAttributes, BuildError> {
        Ok(WorkflowExecutionCanceledAttributes {
            attempt: self.attempt,
            reason: self.reason,
            task_id: self
                .task_id
                .ok_or_else(|| BuildError::missing_field("task_id"))?,
        })
    }
}
