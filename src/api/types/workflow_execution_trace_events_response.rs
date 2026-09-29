pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WorkflowExecutionTraceEventsResponse {
    /// The name of the deployment that ran this execution
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_name: Option<String>,
    /// The end time of the workflow execution, if available
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub end_time: Option<DateTime<FixedOffset>>,
    /// The events of the workflow execution
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<WorkflowExecutionTraceEventsResponseEventsItem>>,
    /// The ID of the workflow execution
    #[serde(default)]
    pub execution_id: String,
    /// The parent execution ID of the workflow execution
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_execution_id: Option<String>,
    /// The result of the workflow execution, if available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    /// The root execution ID of the workflow execution
    #[serde(default)]
    pub root_execution_id: String,
    /// The unique run identifier (database UUID)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    /// The start time of the workflow execution
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub start_time: DateTime<FixedOffset>,
    /// The status of the workflow execution
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<WorkflowExecutionStatus>,
    /// The total duration of the trace in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_duration_ms: Option<i64>,
    /// The ID of the user who triggered the execution
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// The ID of the workflow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_id: Option<String>,
    /// The name of the workflow
    #[serde(default)]
    pub workflow_name: String,
}

impl WorkflowExecutionTraceEventsResponse {
    pub fn builder() -> WorkflowExecutionTraceEventsResponseBuilder {
        <WorkflowExecutionTraceEventsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowExecutionTraceEventsResponseBuilder {
    deployment_name: Option<String>,
    end_time: Option<DateTime<FixedOffset>>,
    events: Option<Vec<WorkflowExecutionTraceEventsResponseEventsItem>>,
    execution_id: Option<String>,
    parent_execution_id: Option<String>,
    result: Option<serde_json::Value>,
    root_execution_id: Option<String>,
    run_id: Option<String>,
    start_time: Option<DateTime<FixedOffset>>,
    status: Option<WorkflowExecutionStatus>,
    total_duration_ms: Option<i64>,
    user_id: Option<String>,
    workflow_id: Option<String>,
    workflow_name: Option<String>,
}

impl WorkflowExecutionTraceEventsResponseBuilder {
    pub fn deployment_name(mut self, value: impl Into<String>) -> Self {
        self.deployment_name = Some(value.into());
        self
    }

    pub fn end_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.end_time = Some(value);
        self
    }

    pub fn events(mut self, value: Vec<WorkflowExecutionTraceEventsResponseEventsItem>) -> Self {
        self.events = Some(value);
        self
    }

    pub fn execution_id(mut self, value: impl Into<String>) -> Self {
        self.execution_id = Some(value.into());
        self
    }

    pub fn parent_execution_id(mut self, value: impl Into<String>) -> Self {
        self.parent_execution_id = Some(value.into());
        self
    }

    pub fn result(mut self, value: serde_json::Value) -> Self {
        self.result = Some(value);
        self
    }

    pub fn root_execution_id(mut self, value: impl Into<String>) -> Self {
        self.root_execution_id = Some(value.into());
        self
    }

    pub fn run_id(mut self, value: impl Into<String>) -> Self {
        self.run_id = Some(value.into());
        self
    }

    pub fn start_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.start_time = Some(value);
        self
    }

    pub fn status(mut self, value: WorkflowExecutionStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn total_duration_ms(mut self, value: i64) -> Self {
        self.total_duration_ms = Some(value);
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn workflow_id(mut self, value: impl Into<String>) -> Self {
        self.workflow_id = Some(value.into());
        self
    }

    pub fn workflow_name(mut self, value: impl Into<String>) -> Self {
        self.workflow_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WorkflowExecutionTraceEventsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`execution_id`](WorkflowExecutionTraceEventsResponseBuilder::execution_id)
    /// - [`root_execution_id`](WorkflowExecutionTraceEventsResponseBuilder::root_execution_id)
    /// - [`start_time`](WorkflowExecutionTraceEventsResponseBuilder::start_time)
    /// - [`workflow_name`](WorkflowExecutionTraceEventsResponseBuilder::workflow_name)
    pub fn build(self) -> Result<WorkflowExecutionTraceEventsResponse, BuildError> {
        Ok(WorkflowExecutionTraceEventsResponse {
            deployment_name: self.deployment_name,
            end_time: self.end_time,
            events: self.events,
            execution_id: self
                .execution_id
                .ok_or_else(|| BuildError::missing_field("execution_id"))?,
            parent_execution_id: self.parent_execution_id,
            result: self.result,
            root_execution_id: self
                .root_execution_id
                .ok_or_else(|| BuildError::missing_field("root_execution_id"))?,
            run_id: self.run_id,
            start_time: self
                .start_time
                .ok_or_else(|| BuildError::missing_field("start_time"))?,
            status: self.status,
            total_duration_ms: self.total_duration_ms,
            user_id: self.user_id,
            workflow_id: self.workflow_id,
            workflow_name: self
                .workflow_name
                .ok_or_else(|| BuildError::missing_field("workflow_name"))?,
        })
    }
}
