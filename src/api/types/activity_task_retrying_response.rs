pub use crate::prelude::*;

/// Emitted when an activity task fails and will be retried.
///
/// Contains information about the failed attempt and the error that occurred.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ActivityTaskRetryingResponse {
    /// Unique identifier for this event instance.
    #[serde(default)]
    pub event_id: String,
    /// Unix timestamp in nanoseconds when the event was created.
    #[serde(default)]
    pub event_timestamp: i64,
    /// Execution ID of the root workflow that initiated this execution chain.
    #[serde(default)]
    pub root_workflow_exec_id: String,
    /// Execution ID of the parent workflow that initiated this execution. If this is a root workflow, this field is not set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_workflow_exec_id: Option<String>,
    /// Run ID of the execution this run continued from. Non-null for continue-as-new runs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continued_run_id: Option<String>,
    /// Run ID of the first execution in this workflow chain. Equals workflow_run_id on fresh starts and resets (chain anchor resets on reset); differs on CAN and Retry runs where it stays anchored to the original first run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_execution_run_id: Option<String>,
    /// Temporal schedule ID that triggered this execution, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_id: Option<String>,
    /// Execution ID of the workflow that emitted this event.
    #[serde(default)]
    pub workflow_exec_id: String,
    /// Run ID of the workflow execution. Changes on continue-as-new while workflow_exec_id stays the same.
    #[serde(default)]
    pub workflow_run_id: String,
    /// The registered name of the workflow that emitted this event.
    #[serde(default)]
    pub workflow_name: String,
    /// Event-specific attributes.
    #[serde(default)]
    pub attributes: ActivityTaskRetryingAttributes,
}

impl ActivityTaskRetryingResponse {
    pub fn builder() -> ActivityTaskRetryingResponseBuilder {
        <ActivityTaskRetryingResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ActivityTaskRetryingResponseBuilder {
    event_id: Option<String>,
    event_timestamp: Option<i64>,
    root_workflow_exec_id: Option<String>,
    parent_workflow_exec_id: Option<String>,
    continued_run_id: Option<String>,
    first_execution_run_id: Option<String>,
    schedule_id: Option<String>,
    workflow_exec_id: Option<String>,
    workflow_run_id: Option<String>,
    workflow_name: Option<String>,
    attributes: Option<ActivityTaskRetryingAttributes>,
}

impl ActivityTaskRetryingResponseBuilder {
    pub fn event_id(mut self, value: impl Into<String>) -> Self {
        self.event_id = Some(value.into());
        self
    }

    pub fn event_timestamp(mut self, value: i64) -> Self {
        self.event_timestamp = Some(value);
        self
    }

    pub fn root_workflow_exec_id(mut self, value: impl Into<String>) -> Self {
        self.root_workflow_exec_id = Some(value.into());
        self
    }

    pub fn parent_workflow_exec_id(mut self, value: impl Into<String>) -> Self {
        self.parent_workflow_exec_id = Some(value.into());
        self
    }

    pub fn continued_run_id(mut self, value: impl Into<String>) -> Self {
        self.continued_run_id = Some(value.into());
        self
    }

    pub fn first_execution_run_id(mut self, value: impl Into<String>) -> Self {
        self.first_execution_run_id = Some(value.into());
        self
    }

    pub fn schedule_id(mut self, value: impl Into<String>) -> Self {
        self.schedule_id = Some(value.into());
        self
    }

    pub fn workflow_exec_id(mut self, value: impl Into<String>) -> Self {
        self.workflow_exec_id = Some(value.into());
        self
    }

    pub fn workflow_run_id(mut self, value: impl Into<String>) -> Self {
        self.workflow_run_id = Some(value.into());
        self
    }

    pub fn workflow_name(mut self, value: impl Into<String>) -> Self {
        self.workflow_name = Some(value.into());
        self
    }

    pub fn attributes(mut self, value: ActivityTaskRetryingAttributes) -> Self {
        self.attributes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ActivityTaskRetryingResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`event_id`](ActivityTaskRetryingResponseBuilder::event_id)
    /// - [`event_timestamp`](ActivityTaskRetryingResponseBuilder::event_timestamp)
    /// - [`root_workflow_exec_id`](ActivityTaskRetryingResponseBuilder::root_workflow_exec_id)
    /// - [`workflow_exec_id`](ActivityTaskRetryingResponseBuilder::workflow_exec_id)
    /// - [`workflow_run_id`](ActivityTaskRetryingResponseBuilder::workflow_run_id)
    /// - [`workflow_name`](ActivityTaskRetryingResponseBuilder::workflow_name)
    /// - [`attributes`](ActivityTaskRetryingResponseBuilder::attributes)
    pub fn build(self) -> Result<ActivityTaskRetryingResponse, BuildError> {
        Ok(ActivityTaskRetryingResponse {
            event_id: self
                .event_id
                .ok_or_else(|| BuildError::missing_field("event_id"))?,
            event_timestamp: self
                .event_timestamp
                .ok_or_else(|| BuildError::missing_field("event_timestamp"))?,
            root_workflow_exec_id: self
                .root_workflow_exec_id
                .ok_or_else(|| BuildError::missing_field("root_workflow_exec_id"))?,
            parent_workflow_exec_id: self.parent_workflow_exec_id,
            continued_run_id: self.continued_run_id,
            first_execution_run_id: self.first_execution_run_id,
            schedule_id: self.schedule_id,
            workflow_exec_id: self
                .workflow_exec_id
                .ok_or_else(|| BuildError::missing_field("workflow_exec_id"))?,
            workflow_run_id: self
                .workflow_run_id
                .ok_or_else(|| BuildError::missing_field("workflow_run_id"))?,
            workflow_name: self
                .workflow_name
                .ok_or_else(|| BuildError::missing_field("workflow_name"))?,
            attributes: self
                .attributes
                .ok_or_else(|| BuildError::missing_field("attributes"))?,
        })
    }
}
