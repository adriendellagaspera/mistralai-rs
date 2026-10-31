pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeploymentWorkerResponse {
    /// When the worker first registered
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// Whether this worker's liveness key is currently alive
    #[serde(default)]
    pub is_active: bool,
    /// When the worker last registered
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub last_heartbeat: DateTime<FixedOffset>,
    /// Where the worker is running; null if the worker did not report a location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<DeploymentLocation>,
    /// Worker name
    #[serde(default)]
    pub name: String,
    /// Deprecated alias of last_heartbeat
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl DeploymentWorkerResponse {
    pub fn builder() -> DeploymentWorkerResponseBuilder {
        <DeploymentWorkerResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeploymentWorkerResponseBuilder {
    created_at: Option<DateTime<FixedOffset>>,
    is_active: Option<bool>,
    last_heartbeat: Option<DateTime<FixedOffset>>,
    location: Option<DeploymentLocation>,
    name: Option<String>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl DeploymentWorkerResponseBuilder {
    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(value);
        self
    }

    pub fn last_heartbeat(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_heartbeat = Some(value);
        self
    }

    pub fn location(mut self, value: DeploymentLocation) -> Self {
        self.location = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeploymentWorkerResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](DeploymentWorkerResponseBuilder::created_at)
    /// - [`is_active`](DeploymentWorkerResponseBuilder::is_active)
    /// - [`last_heartbeat`](DeploymentWorkerResponseBuilder::last_heartbeat)
    /// - [`name`](DeploymentWorkerResponseBuilder::name)
    /// - [`updated_at`](DeploymentWorkerResponseBuilder::updated_at)
    pub fn build(self) -> Result<DeploymentWorkerResponse, BuildError> {
        Ok(DeploymentWorkerResponse {
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            is_active: self
                .is_active
                .ok_or_else(|| BuildError::missing_field("is_active"))?,
            last_heartbeat: self
                .last_heartbeat
                .ok_or_else(|| BuildError::missing_field("last_heartbeat"))?,
            location: self.location,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
