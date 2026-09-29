pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeploymentWorkerSpecInput {
    #[serde(default)]
    pub github_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entrypoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_dir: Option<String>,
}

impl DeploymentWorkerSpecInput {
    pub fn builder() -> DeploymentWorkerSpecInputBuilder {
        <DeploymentWorkerSpecInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeploymentWorkerSpecInputBuilder {
    github_url: Option<String>,
    revision: Option<String>,
    entrypoint: Option<String>,
    working_dir: Option<String>,
}

impl DeploymentWorkerSpecInputBuilder {
    pub fn github_url(mut self, value: impl Into<String>) -> Self {
        self.github_url = Some(value.into());
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

    /// Consumes the builder and constructs a [`DeploymentWorkerSpecInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`github_url`](DeploymentWorkerSpecInputBuilder::github_url)
    pub fn build(self) -> Result<DeploymentWorkerSpecInput, BuildError> {
        Ok(DeploymentWorkerSpecInput {
            github_url: self
                .github_url
                .ok_or_else(|| BuildError::missing_field("github_url"))?,
            revision: self.revision,
            entrypoint: self.entrypoint,
            working_dir: self.working_dir,
        })
    }
}
