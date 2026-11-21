pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeploymentWorkerSpecInput {
    /// Mistral Cloud archive to build the worker from instead of a repository. The archive is the whole build context: 'dockerfile_path' is relative to its root, and 'build_directory' must be unset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_id: Option<String>,
    /// Backend-specific configuration for the Mistral Cloud backend.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_spec: Option<DeploymentMistralCloudBackendSpec>,
    /// Deprecated alias of 'repository_url' for a GitHub repository. Set only 'repository_url'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub github_url: Option<String>,
    /// Repository to build the worker from, set instead of 'archive_id': a GitHub repository (`https://github.com/<owner>/<repo>`) or a repository on the Mistral git server (`https://api.globalaegis.net/v1/git/<workspace>/<repo>.git`). On the git server you must be able to read the repository; the worker builds from an archive of 'revision' and 'build_directory'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_url: Option<String>,
    /// Workspace roles, by name, that the deployment's service account holds beyond `workflow_executor`. Granted only if the deploying user holds them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_roles: Option<Vec<String>>,
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
    archive_id: Option<String>,
    backend_spec: Option<DeploymentMistralCloudBackendSpec>,
    github_url: Option<String>,
    repository_url: Option<String>,
    requested_roles: Option<Vec<String>>,
    revision: Option<String>,
}

impl DeploymentWorkerSpecInputBuilder {
    pub fn archive_id(mut self, value: impl Into<String>) -> Self {
        self.archive_id = Some(value.into());
        self
    }

    pub fn backend_spec(mut self, value: DeploymentMistralCloudBackendSpec) -> Self {
        self.backend_spec = Some(value);
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

    pub fn revision(mut self, value: impl Into<String>) -> Self {
        self.revision = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeploymentWorkerSpecInput`].
    pub fn build(self) -> Result<DeploymentWorkerSpecInput, BuildError> {
        Ok(DeploymentWorkerSpecInput {
            archive_id: self.archive_id,
            backend_spec: self.backend_spec,
            github_url: self.github_url,
            repository_url: self.repository_url,
            requested_roles: self.requested_roles,
            revision: self.revision,
        })
    }
}
