pub use crate::prelude::*;

/// Query parameters for get_workflow_registration_v1_workflows_registrations__workflow_registration_id__get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetWorkflowRegistrationV1WorkflowsRegistrationsWorkflowRegistrationIdGetQueryRequest {
    /// Whether to include the workflow definition
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_workflow: Option<bool>,
    /// Whether to include shared workflow versions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_shared: Option<bool>,
}

impl GetWorkflowRegistrationV1WorkflowsRegistrationsWorkflowRegistrationIdGetQueryRequest {
    pub fn builder(
    ) -> GetWorkflowRegistrationV1WorkflowsRegistrationsWorkflowRegistrationIdGetQueryRequestBuilder
    {
        <GetWorkflowRegistrationV1WorkflowsRegistrationsWorkflowRegistrationIdGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetWorkflowRegistrationV1WorkflowsRegistrationsWorkflowRegistrationIdGetQueryRequestBuilder
{
    with_workflow: Option<bool>,
    include_shared: Option<bool>,
}

impl GetWorkflowRegistrationV1WorkflowsRegistrationsWorkflowRegistrationIdGetQueryRequestBuilder {
    pub fn with_workflow(mut self, value: bool) -> Self {
        self.with_workflow = Some(value);
        self
    }

    pub fn include_shared(mut self, value: bool) -> Self {
        self.include_shared = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetWorkflowRegistrationV1WorkflowsRegistrationsWorkflowRegistrationIdGetQueryRequest`].
    pub fn build(
        self,
    ) -> Result<
        GetWorkflowRegistrationV1WorkflowsRegistrationsWorkflowRegistrationIdGetQueryRequest,
        BuildError,
    > {
        Ok(
            GetWorkflowRegistrationV1WorkflowsRegistrationsWorkflowRegistrationIdGetQueryRequest {
                with_workflow: self.with_workflow,
                include_shared: self.include_shared,
            },
        )
    }
}
