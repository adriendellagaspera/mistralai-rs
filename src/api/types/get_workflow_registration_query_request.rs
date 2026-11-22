pub use crate::prelude::*;

/// Query parameters for get_workflow_registration
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetWorkflowRegistrationQueryRequest {
    /// Whether to include the workflow definition
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_workflow: Option<bool>,
    /// Whether to include shared workflow versions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_shared: Option<bool>,
}

impl GetWorkflowRegistrationQueryRequest {
    pub fn builder() -> GetWorkflowRegistrationQueryRequestBuilder {
        <GetWorkflowRegistrationQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetWorkflowRegistrationQueryRequestBuilder {
    with_workflow: Option<bool>,
    include_shared: Option<bool>,
}

impl GetWorkflowRegistrationQueryRequestBuilder {
    pub fn with_workflow(mut self, value: bool) -> Self {
        self.with_workflow = Some(value);
        self
    }

    pub fn include_shared(mut self, value: bool) -> Self {
        self.include_shared = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetWorkflowRegistrationQueryRequest`].
    pub fn build(self) -> Result<GetWorkflowRegistrationQueryRequest, BuildError> {
        Ok(GetWorkflowRegistrationQueryRequest {
            with_workflow: self.with_workflow,
            include_shared: self.include_shared,
        })
    }
}
