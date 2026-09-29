pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeploymentWorkerSpecResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit: Option<GitCommitMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_sha: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entrypoint: Option<String>,
    #[serde(default)]
    pub github_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restarted_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_dir: Option<String>,
}

impl DeploymentWorkerSpecResponse {
    pub fn builder() -> DeploymentWorkerSpecResponseBuilder {
        <DeploymentWorkerSpecResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeploymentWorkerSpecResponseBuilder {
    commit: Option<GitCommitMetadata>,
    commit_sha: Option<String>,
    entrypoint: Option<String>,
    github_url: Option<String>,
    restarted_at: Option<String>,
    revision: Option<String>,
    r#type: Option<String>,
    working_dir: Option<String>,
}

impl DeploymentWorkerSpecResponseBuilder {
    pub fn commit(mut self, value: GitCommitMetadata) -> Self {
        self.commit = Some(value);
        self
    }

    pub fn commit_sha(mut self, value: impl Into<String>) -> Self {
        self.commit_sha = Some(value.into());
        self
    }

    pub fn entrypoint(mut self, value: impl Into<String>) -> Self {
        self.entrypoint = Some(value.into());
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

    pub fn working_dir(mut self, value: impl Into<String>) -> Self {
        self.working_dir = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeploymentWorkerSpecResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`github_url`](DeploymentWorkerSpecResponseBuilder::github_url)
    pub fn build(self) -> Result<DeploymentWorkerSpecResponse, BuildError> {
        Ok(DeploymentWorkerSpecResponse {
            commit: self.commit,
            commit_sha: self.commit_sha,
            entrypoint: self.entrypoint,
            github_url: self
                .github_url
                .ok_or_else(|| BuildError::missing_field("github_url"))?,
            restarted_at: self.restarted_at,
            revision: self.revision,
            r#type: self.r#type,
            working_dir: self.working_dir,
        })
    }
}
