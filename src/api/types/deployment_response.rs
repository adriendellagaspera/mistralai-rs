pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeploymentResponse {
    /// Number of workers currently live within the liveness cutoff
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_worker_count: Option<i64>,
    /// When the deployment was first registered
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// Unique identifier of the deployment
    #[serde(default)]
    pub id: String,
    /// Whether at least one worker is currently live
    #[serde(default)]
    pub is_active: bool,
    /// Whether the deployment has at least one authorized credential
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_hardened: Option<bool>,
    /// Where the deployment is running
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<DeploymentLocation>,
    /// Distinct location types reported by the deployment's workers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<LocationType>>,
    /// Live managed service state for managed deployments; null for self-hosted deployments or when managed services are unavailable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed: Option<ManagedDeploymentResponse>,
    /// Deployment name
    #[serde(default)]
    pub name: String,
    /// When the deployment was last updated
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    /// Number of workers registered to the deployment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_count: Option<i64>,
}

impl DeploymentResponse {
    pub fn builder() -> DeploymentResponseBuilder {
        <DeploymentResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeploymentResponseBuilder {
    active_worker_count: Option<i64>,
    created_at: Option<DateTime<FixedOffset>>,
    id: Option<String>,
    is_active: Option<bool>,
    is_hardened: Option<bool>,
    location: Option<DeploymentLocation>,
    locations: Option<Vec<LocationType>>,
    managed: Option<ManagedDeploymentResponse>,
    name: Option<String>,
    updated_at: Option<DateTime<FixedOffset>>,
    worker_count: Option<i64>,
}

impl DeploymentResponseBuilder {
    pub fn active_worker_count(mut self, value: i64) -> Self {
        self.active_worker_count = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(value);
        self
    }

    pub fn is_hardened(mut self, value: bool) -> Self {
        self.is_hardened = Some(value);
        self
    }

    pub fn location(mut self, value: DeploymentLocation) -> Self {
        self.location = Some(value);
        self
    }

    pub fn locations(mut self, value: Vec<LocationType>) -> Self {
        self.locations = Some(value);
        self
    }

    pub fn managed(mut self, value: ManagedDeploymentResponse) -> Self {
        self.managed = Some(value);
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

    pub fn worker_count(mut self, value: i64) -> Self {
        self.worker_count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeploymentResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](DeploymentResponseBuilder::created_at)
    /// - [`id`](DeploymentResponseBuilder::id)
    /// - [`is_active`](DeploymentResponseBuilder::is_active)
    /// - [`name`](DeploymentResponseBuilder::name)
    /// - [`updated_at`](DeploymentResponseBuilder::updated_at)
    pub fn build(self) -> Result<DeploymentResponse, BuildError> {
        Ok(DeploymentResponse {
            active_worker_count: self.active_worker_count,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            is_active: self
                .is_active
                .ok_or_else(|| BuildError::missing_field("is_active"))?,
            is_hardened: self.is_hardened,
            location: self.location,
            locations: self.locations,
            managed: self.managed,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            worker_count: self.worker_count,
        })
    }
}
