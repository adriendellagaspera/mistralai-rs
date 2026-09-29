pub use crate::prelude::*;

/// Query parameters for logs
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkflowsDeploymentsLogsQueryRequest {
    /// Filter logs by worker name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_name: Option<String>,
    /// Filter logs by workflow name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_name: Option<String>,
    /// Only return logs at or after this timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<DateTime<FixedOffset>>,
    /// Only return logs before this timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<DateTime<FixedOffset>>,
    /// First-page sort order: 'asc' (oldest first) or 'desc'. Ignored when `cursor` is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<LogsDeploymentsRequestOrder>,
    /// Pagination cursor from a previous response's `next_cursor`; carries the window and order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Maximum number of logs to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl WorkflowsDeploymentsLogsQueryRequest {
    pub fn builder() -> WorkflowsDeploymentsLogsQueryRequestBuilder {
        <WorkflowsDeploymentsLogsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowsDeploymentsLogsQueryRequestBuilder {
    worker_name: Option<String>,
    workflow_name: Option<String>,
    after: Option<DateTime<FixedOffset>>,
    before: Option<DateTime<FixedOffset>>,
    order: Option<LogsDeploymentsRequestOrder>,
    cursor: Option<String>,
    limit: Option<i64>,
}

impl WorkflowsDeploymentsLogsQueryRequestBuilder {
    pub fn worker_name(mut self, value: impl Into<String>) -> Self {
        self.worker_name = Some(value.into());
        self
    }

    pub fn workflow_name(mut self, value: impl Into<String>) -> Self {
        self.workflow_name = Some(value.into());
        self
    }

    pub fn after(mut self, value: DateTime<FixedOffset>) -> Self {
        self.after = Some(value);
        self
    }

    pub fn before(mut self, value: DateTime<FixedOffset>) -> Self {
        self.before = Some(value);
        self
    }

    pub fn order(mut self, value: LogsDeploymentsRequestOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowsDeploymentsLogsQueryRequest`].
    pub fn build(self) -> Result<WorkflowsDeploymentsLogsQueryRequest, BuildError> {
        Ok(WorkflowsDeploymentsLogsQueryRequest {
            worker_name: self.worker_name,
            workflow_name: self.workflow_name,
            after: self.after,
            before: self.before,
            order: self.order,
            cursor: self.cursor,
            limit: self.limit,
        })
    }
}
