pub use crate::prelude::*;

/// Query parameters for get_workflow_execution_v1_workflows_executions__execution_id__get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetWorkflowExecutionV1WorkflowsExecutionsExecutionIdGetQueryRequest {
    /// Include the execution's search keys (metadata) in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_search_keys: Option<bool>,
}

impl GetWorkflowExecutionV1WorkflowsExecutionsExecutionIdGetQueryRequest {
    pub fn builder() -> GetWorkflowExecutionV1WorkflowsExecutionsExecutionIdGetQueryRequestBuilder {
        <GetWorkflowExecutionV1WorkflowsExecutionsExecutionIdGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetWorkflowExecutionV1WorkflowsExecutionsExecutionIdGetQueryRequestBuilder {
    include_search_keys: Option<bool>,
}

impl GetWorkflowExecutionV1WorkflowsExecutionsExecutionIdGetQueryRequestBuilder {
    pub fn include_search_keys(mut self, value: bool) -> Self {
        self.include_search_keys = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetWorkflowExecutionV1WorkflowsExecutionsExecutionIdGetQueryRequest`].
    pub fn build(
        self,
    ) -> Result<GetWorkflowExecutionV1WorkflowsExecutionsExecutionIdGetQueryRequest, BuildError>
    {
        Ok(
            GetWorkflowExecutionV1WorkflowsExecutionsExecutionIdGetQueryRequest {
                include_search_keys: self.include_search_keys,
            },
        )
    }
}
