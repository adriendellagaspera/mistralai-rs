pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeploymentWorkerSpecResponse {
    /// Archive the worker builds from, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_id: Option<String>,
    /// Backend-specific configuration. 'mistral_cloud' is Mistral Cloud; 'kubernetes' is a deployment that already runs on Kubernetes.
    pub backend_spec: DeploymentWorkerSpecResponseBackendSpec,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit: Option<GitCommitMetadata>,
    /// GitHub repository the worker builds from. Empty for a worker built from an archive.
    #[serde(default)]
    pub github_url: String,
    /// Repository the worker builds from, on GitHub or the git server, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_roles: Option<Vec<String>>,
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
    archive_id: Option<String>,
    backend_spec: Option<DeploymentWorkerSpecResponseBackendSpec>,
    commit: Option<GitCommitMetadata>,
    github_url: Option<String>,
    repository_url: Option<String>,
    requested_roles: Option<Vec<String>>,
    restarted_at: Option<String>,
    revision: Option<String>,
    r#type: Option<String>,
}

impl DeploymentWorkerSpecResponseBuilder {
    pub fn archive_id(mut self, value: impl Into<String>) -> Self {
        self.archive_id = Some(value.into());
        self
    }

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

    pub fn repository_url(mut self, value: impl Into<String>) -> Self {
        self.repository_url = Some(value.into());
        self
    }

    pub fn requested_roles(mut self, value: Vec<String>) -> Self {
        self.requested_roles = Some(value);
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
            archive_id: self.archive_id,
            backend_spec: self
                .backend_spec
                .ok_or_else(|| BuildError::missing_field("backend_spec"))?,
            commit: self.commit,
            github_url: self
                .github_url
                .ok_or_else(|| BuildError::missing_field("github_url"))?,
            repository_url: self.repository_url,
            requested_roles: self.requested_roles,
            restarted_at: self.restarted_at,
            revision: self.revision,
            r#type: self.r#type,
        })
    }
}
