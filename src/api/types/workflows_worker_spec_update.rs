pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkflowsWorkerSpecUpdate {
    /// Backend-specific configuration for the Mistral Cloud backend.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_spec: Option<DeploymentMistralCloudBackendSpec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub github_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
}

impl WorkflowsWorkerSpecUpdate {
    pub fn builder() -> WorkflowsWorkerSpecUpdateBuilder {
        <WorkflowsWorkerSpecUpdateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowsWorkerSpecUpdateBuilder {
    backend_spec: Option<DeploymentMistralCloudBackendSpec>,
    github_url: Option<String>,
    revision: Option<String>,
}

impl WorkflowsWorkerSpecUpdateBuilder {
    pub fn backend_spec(mut self, value: DeploymentMistralCloudBackendSpec) -> Self {
        self.backend_spec = Some(value);
        self
    }

    pub fn github_url(mut self, value: impl Into<String>) -> Self {
        self.github_url = Some(value.into());
        self
    }

    pub fn revision(mut self, value: impl Into<String>) -> Self {
        self.revision = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WorkflowsWorkerSpecUpdate`].
    pub fn build(self) -> Result<WorkflowsWorkerSpecUpdate, BuildError> {
        Ok(WorkflowsWorkerSpecUpdate {
            backend_spec: self.backend_spec,
            github_url: self.github_url,
            revision: self.revision,
        })
    }
}
