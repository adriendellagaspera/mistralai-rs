pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeploymentWorkerSpecInput {
    /// Backend-specific configuration for the Mistral Cloud backend.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_spec: Option<DeploymentMistralCloudBackendSpec>,
    #[serde(default)]
    pub github_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
}

impl DeploymentWorkerSpecInput {
    pub fn builder() -> DeploymentWorkerSpecInputBuilder {
        <DeploymentWorkerSpecInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeploymentWorkerSpecInputBuilder {
    backend_spec: Option<DeploymentMistralCloudBackendSpec>,
    github_url: Option<String>,
    revision: Option<String>,
}

impl DeploymentWorkerSpecInputBuilder {
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

    /// Consumes the builder and constructs a [`DeploymentWorkerSpecInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`github_url`](DeploymentWorkerSpecInputBuilder::github_url)
    pub fn build(self) -> Result<DeploymentWorkerSpecInput, BuildError> {
        Ok(DeploymentWorkerSpecInput {
            backend_spec: self.backend_spec,
            github_url: self
                .github_url
                .ok_or_else(|| BuildError::missing_field("github_url"))?,
            revision: self.revision,
        })
    }
}
