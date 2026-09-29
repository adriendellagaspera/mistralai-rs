pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WorkflowRegistrationGetResponse {
    /// The workflow registration
    #[serde(default)]
    pub workflow_registration: WorkflowRegistrationWithWorkerStatus,
    /// Deprecated: use workflow_registration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_version: Option<WorkflowRegistrationWithWorkerStatus>,
}

impl WorkflowRegistrationGetResponse {
    pub fn builder() -> WorkflowRegistrationGetResponseBuilder {
        <WorkflowRegistrationGetResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowRegistrationGetResponseBuilder {
    workflow_registration: Option<WorkflowRegistrationWithWorkerStatus>,
    workflow_version: Option<WorkflowRegistrationWithWorkerStatus>,
}

impl WorkflowRegistrationGetResponseBuilder {
    pub fn workflow_registration(mut self, value: WorkflowRegistrationWithWorkerStatus) -> Self {
        self.workflow_registration = Some(value);
        self
    }

    pub fn workflow_version(mut self, value: WorkflowRegistrationWithWorkerStatus) -> Self {
        self.workflow_version = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowRegistrationGetResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`workflow_registration`](WorkflowRegistrationGetResponseBuilder::workflow_registration)
    pub fn build(self) -> Result<WorkflowRegistrationGetResponse, BuildError> {
        Ok(WorkflowRegistrationGetResponse {
            workflow_registration: self
                .workflow_registration
                .ok_or_else(|| BuildError::missing_field("workflow_registration"))?,
            workflow_version: self.workflow_version,
        })
    }
}
