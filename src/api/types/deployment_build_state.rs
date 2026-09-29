pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeploymentBuildState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_sha: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<DateTime<FixedOffset>>,
}

impl DeploymentBuildState {
    pub fn builder() -> DeploymentBuildStateBuilder {
        <DeploymentBuildStateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeploymentBuildStateBuilder {
    commit_sha: Option<String>,
    finished_at: Option<DateTime<FixedOffset>>,
    image: Option<String>,
    message: Option<String>,
    phase: Option<String>,
    started_at: Option<DateTime<FixedOffset>>,
}

impl DeploymentBuildStateBuilder {
    pub fn commit_sha(mut self, value: impl Into<String>) -> Self {
        self.commit_sha = Some(value.into());
        self
    }

    pub fn finished_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.finished_at = Some(value);
        self
    }

    pub fn image(mut self, value: impl Into<String>) -> Self {
        self.image = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn phase(mut self, value: impl Into<String>) -> Self {
        self.phase = Some(value.into());
        self
    }

    pub fn started_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.started_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeploymentBuildState`].
    pub fn build(self) -> Result<DeploymentBuildState, BuildError> {
        Ok(DeploymentBuildState {
            commit_sha: self.commit_sha,
            finished_at: self.finished_at,
            image: self.image,
            message: self.message,
            phase: self.phase,
            started_at: self.started_at,
        })
    }
}
