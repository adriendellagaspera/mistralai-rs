pub use crate::prelude::*;

/// Query parameters for get_workflow_execution
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetWorkflowExecutionQueryRequest {
    /// Include the execution's search keys (metadata) in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_search_keys: Option<bool>,
}

impl GetWorkflowExecutionQueryRequest {
    pub fn builder() -> GetWorkflowExecutionQueryRequestBuilder {
        <GetWorkflowExecutionQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetWorkflowExecutionQueryRequestBuilder {
    include_search_keys: Option<bool>,
}

impl GetWorkflowExecutionQueryRequestBuilder {
    pub fn include_search_keys(mut self, value: bool) -> Self {
        self.include_search_keys = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetWorkflowExecutionQueryRequest`].
    pub fn build(self) -> Result<GetWorkflowExecutionQueryRequest, BuildError> {
        Ok(GetWorkflowExecutionQueryRequest {
            include_search_keys: self.include_search_keys,
        })
    }
}
