pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeploymentWorkerSpecResponse {
    /// Backend-specific configuration. 'mistral_cloud' is Mistral Cloud; 'kubernetes' is a deployment that already runs on Kubernetes.
    pub backend_spec: DeploymentWorkerSpecResponseBackendSpec,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit: Option<GitCommitMetadata>,
    #[serde(default)]
    pub github_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restarted_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl DeploymentWorkerSpecResponse {
    pub fn builder() -> DeploymentWorkerSpecResponseBuilder {
        <DeploymentWorkerSpecResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeploymentWorkerSpecResponseBuilder {
    backend_spec: Option<DeploymentWorkerSpecResponseBackendSpec>,
    commit: Option<GitCommitMetadata>,
    github_url: Option<String>,
    restarted_at: Option<String>,
    revision: Option<String>,
    r#type: Option<String>,
}

impl DeploymentWorkerSpecResponseBuilder {
    pub fn backend_spec(mut self, value: DeploymentWorkerSpecResponseBackendSpec) -> Self {
        self.backend_spec = Some(value);
        self
    }

    pub fn commit(mut self, value: GitCommitMetadata) -> Self {
        self.commit = Some(value);
        self
    }

    pub fn github_url(mut self, value: impl Into<String>) -> Self {
        self.github_url = Some(value.into());
        self
    }

    pub fn restarted_at(mut self, value: impl Into<String>) -> Self {
        self.restarted_at = Some(value.into());
        self
    }

    pub fn revision(mut self, value: impl Into<String>) -> Self {
        self.revision = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeploymentWorkerSpecResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`backend_spec`](DeploymentWorkerSpecResponseBuilder::backend_spec)
    /// - [`github_url`](DeploymentWorkerSpecResponseBuilder::github_url)
    pub fn build(self) -> Result<DeploymentWorkerSpecResponse, BuildError> {
        Ok(DeploymentWorkerSpecResponse {
            backend_spec: self
                .backend_spec
                .ok_or_else(|| BuildError::missing_field("backend_spec"))?,
            commit: self.commit,
            github_url: self
                .github_url
                .ok_or_else(|| BuildError::missing_field("github_url"))?,
            restarted_at: self.restarted_at,
            revision: self.revision,
            r#type: self.r#type,
        })
    }
}
