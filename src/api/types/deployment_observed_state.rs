pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeploymentObservedState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_replicas: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ready_replicas: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_seen: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployed_revision: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_state: Option<DeploymentBuildState>,
}

impl DeploymentObservedState {
    pub fn builder() -> DeploymentObservedStateBuilder {
        <DeploymentObservedStateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeploymentObservedStateBuilder {
    phase: Option<String>,
    available_replicas: Option<i64>,
    ready_replicas: Option<i64>,
    endpoint: Option<String>,
    message: Option<String>,
    last_seen: Option<DateTime<FixedOffset>>,
    deployed_revision: Option<String>,
    generation: Option<i64>,
    build_state: Option<DeploymentBuildState>,
}

impl DeploymentObservedStateBuilder {
    pub fn phase(mut self, value: impl Into<String>) -> Self {
        self.phase = Some(value.into());
        self
    }

    pub fn available_replicas(mut self, value: i64) -> Self {
        self.available_replicas = Some(value);
        self
    }

    pub fn ready_replicas(mut self, value: i64) -> Self {
        self.ready_replicas = Some(value);
        self
    }

    pub fn endpoint(mut self, value: impl Into<String>) -> Self {
        self.endpoint = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn last_seen(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_seen = Some(value);
        self
    }

    pub fn deployed_revision(mut self, value: impl Into<String>) -> Self {
        self.deployed_revision = Some(value.into());
        self
    }

    pub fn generation(mut self, value: i64) -> Self {
        self.generation = Some(value);
        self
    }

    pub fn build_state(mut self, value: DeploymentBuildState) -> Self {
        self.build_state = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeploymentObservedState`].
    pub fn build(self) -> Result<DeploymentObservedState, BuildError> {
        Ok(DeploymentObservedState {
            phase: self.phase,
            available_replicas: self.available_replicas,
            ready_replicas: self.ready_replicas,
            endpoint: self.endpoint,
            message: self.message,
            last_seen: self.last_seen,
            deployed_revision: self.deployed_revision,
            generation: self.generation,
            build_state: self.build_state,
        })
    }
}
