pub use crate::prelude::*;

/// Query parameters for get_logs
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkflowsDeploymentsGetLogsQueryRequest {
    /// Which stream to read. 'runtime' is the workers' own logs; 'build' reads the image build output and requires a managed deployment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_type: Option<DeploymentLogType>,
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
    pub order: Option<GetLogsDeploymentsRequestOrder>,
    /// Pagination cursor from a previous response's `next_cursor`; carries the window and order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Maximum number of logs to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl WorkflowsDeploymentsGetLogsQueryRequest {
    pub fn builder() -> WorkflowsDeploymentsGetLogsQueryRequestBuilder {
        <WorkflowsDeploymentsGetLogsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowsDeploymentsGetLogsQueryRequestBuilder {
    log_type: Option<DeploymentLogType>,
    worker_name: Option<String>,
    workflow_name: Option<String>,
    after: Option<DateTime<FixedOffset>>,
    before: Option<DateTime<FixedOffset>>,
    order: Option<GetLogsDeploymentsRequestOrder>,
    cursor: Option<String>,
    limit: Option<i64>,
}

impl WorkflowsDeploymentsGetLogsQueryRequestBuilder {
    pub fn log_type(mut self, value: DeploymentLogType) -> Self {
        self.log_type = Some(value);
        self
    }

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

    pub fn order(mut self, value: GetLogsDeploymentsRequestOrder) -> Self {
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

    /// Consumes the builder and constructs a [`WorkflowsDeploymentsGetLogsQueryRequest`].
    pub fn build(self) -> Result<WorkflowsDeploymentsGetLogsQueryRequest, BuildError> {
        Ok(WorkflowsDeploymentsGetLogsQueryRequest {
            log_type: self.log_type,
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
