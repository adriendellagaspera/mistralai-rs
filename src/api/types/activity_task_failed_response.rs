pub use crate::prelude::*;

/// Emitted when an activity task fails after exhausting all retry attempts.
///
/// This is a terminal event indicating the activity could not complete successfully.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ActivityTaskFailedResponse {
    /// Event-specific attributes.
    #[serde(default)]
    pub attributes: ActivityTaskFailedAttributes,
    /// Run ID of the execution this run continued from. Non-null for continue-as-new runs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continued_run_id: Option<String>,
    /// Unique identifier for this event instance.
    #[serde(default)]
    pub event_id: String,
    /// Unix timestamp in nanoseconds when the event was created.
    #[serde(default)]
    pub event_timestamp: i64,
    /// Run ID of the first execution in this workflow chain. Equals workflow_run_id on fresh starts and resets (chain anchor resets on reset); differs on CAN and Retry runs where it stays anchored to the original first run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_execution_run_id: Option<String>,
    /// Execution ID of the parent workflow that initiated this execution. If this is a root workflow, this field is not set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_workflow_exec_id: Option<String>,
    /// Execution ID of the root workflow that initiated this execution chain.
    #[serde(default)]
    pub root_workflow_exec_id: String,
    /// Temporal schedule ID that triggered this execution, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_id: Option<String>,
    /// Execution ID of the workflow that emitted this event.
    #[serde(default)]
    pub workflow_exec_id: String,
    /// The registered name of the workflow that emitted this event.
    #[serde(default)]
    pub workflow_name: String,
    /// Run ID of the workflow execution. Changes on continue-as-new while workflow_exec_id stays the same.
    #[serde(default)]
    pub workflow_run_id: String,
}

impl ActivityTaskFailedResponse {
    pub fn builder() -> ActivityTaskFailedResponseBuilder {
        <ActivityTaskFailedResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ActivityTaskFailedResponseBuilder {
    attributes: Option<ActivityTaskFailedAttributes>,
    continued_run_id: Option<String>,
    event_id: Option<String>,
    event_timestamp: Option<i64>,
    first_execution_run_id: Option<String>,
    parent_workflow_exec_id: Option<String>,
    root_workflow_exec_id: Option<String>,
    schedule_id: Option<String>,
    workflow_exec_id: Option<String>,
    workflow_name: Option<String>,
    workflow_run_id: Option<String>,
}

impl ActivityTaskFailedResponseBuilder {
    pub fn attributes(mut self, value: ActivityTaskFailedAttributes) -> Self {
        self.attributes = Some(value);
        self
    }

    pub fn continued_run_id(mut self, value: impl Into<String>) -> Self {
        self.continued_run_id = Some(value.into());
        self
    }

    pub fn event_id(mut self, value: impl Into<String>) -> Self {
        self.event_id = Some(value.into());
        self
    }

    pub fn event_timestamp(mut self, value: i64) -> Self {
        self.event_timestamp = Some(value);
        self
    }

    pub fn first_execution_run_id(mut self, value: impl Into<String>) -> Self {
        self.first_execution_run_id = Some(value.into());
        self
    }

    pub fn parent_workflow_exec_id(mut self, value: impl Into<String>) -> Self {
        self.parent_workflow_exec_id = Some(value.into());
        self
    }

    pub fn root_workflow_exec_id(mut self, value: impl Into<String>) -> Self {
        self.root_workflow_exec_id = Some(value.into());
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

    pub fn workflow_name(mut self, value: impl Into<String>) -> Self {
        self.workflow_name = Some(value.into());
        self
    }

    pub fn workflow_run_id(mut self, value: impl Into<String>) -> Self {
        self.workflow_run_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ActivityTaskFailedResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attributes`](ActivityTaskFailedResponseBuilder::attributes)
    /// - [`event_id`](ActivityTaskFailedResponseBuilder::event_id)
    /// - [`event_timestamp`](ActivityTaskFailedResponseBuilder::event_timestamp)
    /// - [`root_workflow_exec_id`](ActivityTaskFailedResponseBuilder::root_workflow_exec_id)
    /// - [`workflow_exec_id`](ActivityTaskFailedResponseBuilder::workflow_exec_id)
    /// - [`workflow_name`](ActivityTaskFailedResponseBuilder::workflow_name)
    /// - [`workflow_run_id`](ActivityTaskFailedResponseBuilder::workflow_run_id)
    pub fn build(self) -> Result<ActivityTaskFailedResponse, BuildError> {
        Ok(ActivityTaskFailedResponse {
            attributes: self
                .attributes
                .ok_or_else(|| BuildError::missing_field("attributes"))?,
            continued_run_id: self.continued_run_id,
            event_id: self
                .event_id
                .ok_or_else(|| BuildError::missing_field("event_id"))?,
            event_timestamp: self
                .event_timestamp
                .ok_or_else(|| BuildError::missing_field("event_timestamp"))?,
            first_execution_run_id: self.first_execution_run_id,
            parent_workflow_exec_id: self.parent_workflow_exec_id,
            root_workflow_exec_id: self
                .root_workflow_exec_id
                .ok_or_else(|| BuildError::missing_field("root_workflow_exec_id"))?,
            schedule_id: self.schedule_id,
            workflow_exec_id: self
                .workflow_exec_id
                .ok_or_else(|| BuildError::missing_field("workflow_exec_id"))?,
            workflow_name: self
                .workflow_name
                .ok_or_else(|| BuildError::missing_field("workflow_name"))?,
            workflow_run_id: self
                .workflow_run_id
                .ok_or_else(|| BuildError::missing_field("workflow_run_id"))?,
        })
    }
}
