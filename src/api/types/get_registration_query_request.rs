pub use crate::prelude::*;

/// Query parameters for get_registration
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetRegistrationQueryRequest {
    /// Whether to include the workflow definition
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_workflow: Option<bool>,
    /// Whether to include shared workflow versions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_shared: Option<bool>,
}

impl GetRegistrationQueryRequest {
    pub fn builder() -> GetRegistrationQueryRequestBuilder {
        <GetRegistrationQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetRegistrationQueryRequestBuilder {
    with_workflow: Option<bool>,
    include_shared: Option<bool>,
}

impl GetRegistrationQueryRequestBuilder {
    pub fn with_workflow(mut self, value: bool) -> Self {
        self.with_workflow = Some(value);
        self
    }

    pub fn include_shared(mut self, value: bool) -> Self {
        self.include_shared = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetRegistrationQueryRequest`].
    pub fn build(self) -> Result<GetRegistrationQueryRequest, BuildError> {
        Ok(GetRegistrationQueryRequest {
            with_workflow: self.with_workflow,
            include_shared: self.include_shared,
        })
    }
}
