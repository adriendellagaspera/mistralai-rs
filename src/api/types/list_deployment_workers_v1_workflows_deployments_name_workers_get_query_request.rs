pub use crate::prelude::*;

/// Query parameters for list_deployment_workers_v1_workflows_deployments__name__workers_get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListDeploymentWorkersV1WorkflowsDeploymentsNameWorkersGetQueryRequest {
    /// Filter by worker activity. active=only active, inactive=only inactive, None=no filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_status: Option<
        ListDeploymentWorkersV1WorkflowsDeploymentsNameWorkersGetDeploymentsRequestWorkerStatus,
    >,
    /// Maximum number of workers to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Cursor from a previous response's `next_cursor`. Resend `worker_status` unchanged alongside it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl ListDeploymentWorkersV1WorkflowsDeploymentsNameWorkersGetQueryRequest {
    pub fn builder() -> ListDeploymentWorkersV1WorkflowsDeploymentsNameWorkersGetQueryRequestBuilder
    {
        <ListDeploymentWorkersV1WorkflowsDeploymentsNameWorkersGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListDeploymentWorkersV1WorkflowsDeploymentsNameWorkersGetQueryRequestBuilder {
    worker_status: Option<
        ListDeploymentWorkersV1WorkflowsDeploymentsNameWorkersGetDeploymentsRequestWorkerStatus,
    >,
    limit: Option<i64>,
    cursor: Option<String>,
}

impl ListDeploymentWorkersV1WorkflowsDeploymentsNameWorkersGetQueryRequestBuilder {
    pub fn worker_status(
        mut self,
        value: ListDeploymentWorkersV1WorkflowsDeploymentsNameWorkersGetDeploymentsRequestWorkerStatus,
    ) -> Self {
        self.worker_status = Some(value);
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

    /// Consumes the builder and constructs a [`ListDeploymentWorkersV1WorkflowsDeploymentsNameWorkersGetQueryRequest`].
    pub fn build(
        self,
    ) -> Result<ListDeploymentWorkersV1WorkflowsDeploymentsNameWorkersGetQueryRequest, BuildError>
    {
        Ok(
            ListDeploymentWorkersV1WorkflowsDeploymentsNameWorkersGetQueryRequest {
                worker_status: self.worker_status,
                limit: self.limit,
                cursor: self.cursor,
            },
        )
    }
}
