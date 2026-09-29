pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WorkflowRegistrationListResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    /// A list of workflow registrations
    #[serde(default)]
    pub workflow_registrations: Vec<WorkflowRegistration>,
    /// Deprecated: use workflow_registrations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_versions: Option<Vec<WorkflowRegistration>>,
}

impl WorkflowRegistrationListResponse {
    pub fn builder() -> WorkflowRegistrationListResponseBuilder {
        <WorkflowRegistrationListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowRegistrationListResponseBuilder {
    next_cursor: Option<String>,
    workflow_registrations: Option<Vec<WorkflowRegistration>>,
    workflow_versions: Option<Vec<WorkflowRegistration>>,
}

impl WorkflowRegistrationListResponseBuilder {
    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    pub fn workflow_registrations(mut self, value: Vec<WorkflowRegistration>) -> Self {
        self.workflow_registrations = Some(value);
        self
    }

    pub fn workflow_versions(mut self, value: Vec<WorkflowRegistration>) -> Self {
        self.workflow_versions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowRegistrationListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`workflow_registrations`](WorkflowRegistrationListResponseBuilder::workflow_registrations)
    pub fn build(self) -> Result<WorkflowRegistrationListResponse, BuildError> {
        Ok(WorkflowRegistrationListResponse {
            next_cursor: self.next_cursor,
            workflow_registrations: self
                .workflow_registrations
                .ok_or_else(|| BuildError::missing_field("workflow_registrations"))?,
            workflow_versions: self.workflow_versions,
        })
    }
}
