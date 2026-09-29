pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeploymentWorkerSpecResponse {
    #[serde(default)]
    pub github_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entrypoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_dir: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restarted_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_sha: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit: Option<GitCommitMetadata>,
}

impl DeploymentWorkerSpecResponse {
    pub fn builder() -> DeploymentWorkerSpecResponseBuilder {
        <DeploymentWorkerSpecResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeploymentWorkerSpecResponseBuilder {
    github_url: Option<String>,
    r#type: Option<String>,
    revision: Option<String>,
    entrypoint: Option<String>,
    working_dir: Option<String>,
    restarted_at: Option<String>,
    commit_sha: Option<String>,
    commit: Option<GitCommitMetadata>,
}

impl DeploymentWorkerSpecResponseBuilder {
    pub fn github_url(mut self, value: impl Into<String>) -> Self {
        self.github_url = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn revision(mut self, value: impl Into<String>) -> Self {
        self.revision = Some(value.into());
        self
    }

    pub fn entrypoint(mut self, value: impl Into<String>) -> Self {
        self.entrypoint = Some(value.into());
        self
    }

    pub fn working_dir(mut self, value: impl Into<String>) -> Self {
        self.working_dir = Some(value.into());
        self
    }

    pub fn restarted_at(mut self, value: impl Into<String>) -> Self {
        self.restarted_at = Some(value.into());
        self
    }

    pub fn commit_sha(mut self, value: impl Into<String>) -> Self {
        self.commit_sha = Some(value.into());
        self
    }

    pub fn commit(mut self, value: GitCommitMetadata) -> Self {
        self.commit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeploymentWorkerSpecResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`github_url`](DeploymentWorkerSpecResponseBuilder::github_url)
    pub fn build(self) -> Result<DeploymentWorkerSpecResponse, BuildError> {
        Ok(DeploymentWorkerSpecResponse {
            github_url: self
                .github_url
                .ok_or_else(|| BuildError::missing_field("github_url"))?,
            r#type: self.r#type,
            revision: self.revision,
            entrypoint: self.entrypoint,
            working_dir: self.working_dir,
            restarted_at: self.restarted_at,
            commit_sha: self.commit_sha,
            commit: self.commit,
        })
    }
}
