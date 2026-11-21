pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkflowsWorkerSpecUpdate {
    /// Rebuild the worker from this archive, dropping 'repository_url' and 'revision'. Sending 'repository_url' instead moves it back to a repository.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_id: Option<String>,
    /// Backend-specific configuration for the Mistral Cloud backend.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_spec: Option<DeploymentMistralCloudBackendSpec>,
    /// Deprecated alias of 'repository_url' for a GitHub repository. Set only 'repository_url'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub github_url: Option<String>,
    /// Rebuild the worker from this repository, on GitHub or the git server, dropping 'archive_id'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_url: Option<String>,
    /// The complete workspace-role set after the update, beyond `workflow_executor`. Replaces the current roles; omit it to leave them unchanged, send an empty list to drop all extras. Granted only if the deploying user holds them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_roles: Option<Vec<String>>,
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
    archive_id: Option<String>,
    backend_spec: Option<DeploymentMistralCloudBackendSpec>,
    github_url: Option<String>,
    repository_url: Option<String>,
    requested_roles: Option<Vec<String>>,
    revision: Option<String>,
}

impl WorkflowsWorkerSpecUpdateBuilder {
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

    /// Consumes the builder and constructs a [`WorkflowsWorkerSpecUpdate`].
    pub fn build(self) -> Result<WorkflowsWorkerSpecUpdate, BuildError> {
        Ok(WorkflowsWorkerSpecUpdate {
            archive_id: self.archive_id,
            backend_spec: self.backend_spec,
            github_url: self.github_url,
            repository_url: self.repository_url,
            requested_roles: self.requested_roles,
            revision: self.revision,
        })
    }
}
