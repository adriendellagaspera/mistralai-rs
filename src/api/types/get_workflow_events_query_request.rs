pub use crate::prelude::*;

/// Query parameters for get_workflow_events
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetWorkflowEventsQueryRequest {
    /// Execution ID of the root workflow that initiated this execution chain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_workflow_exec_id: Option<String>,
    /// Execution ID of the workflow that emitted this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_exec_id: Option<String>,
    /// Run ID of the workflow that emitted this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_run_id: Option<String>,
    /// Maximum number of events to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Cursor for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetWorkflowEventsQueryRequest {
    pub fn builder() -> GetWorkflowEventsQueryRequestBuilder {
        <GetWorkflowEventsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetWorkflowEventsQueryRequestBuilder {
    root_workflow_exec_id: Option<String>,
    workflow_exec_id: Option<String>,
    workflow_run_id: Option<String>,
    limit: Option<i64>,
    cursor: Option<String>,
}

impl GetWorkflowEventsQueryRequestBuilder {
    pub fn root_workflow_exec_id(mut self, value: impl Into<String>) -> Self {
        self.root_workflow_exec_id = Some(value.into());
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

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetWorkflowEventsQueryRequest`].
    pub fn build(self) -> Result<GetWorkflowEventsQueryRequest, BuildError> {
        Ok(GetWorkflowEventsQueryRequest {
            root_workflow_exec_id: self.root_workflow_exec_id,
            workflow_exec_id: self.workflow_exec_id,
            workflow_run_id: self.workflow_run_id,
            limit: self.limit,
            cursor: self.cursor,
        })
    }
}
