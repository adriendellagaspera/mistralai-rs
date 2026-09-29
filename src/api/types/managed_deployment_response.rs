pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ManagedDeploymentResponse {
    #[serde(default)]
    pub service_id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub spec: DeploymentWorkerSpecResponse,
    #[serde(default)]
    pub resources: DeploymentResourceConfig,
    #[serde(default)]
    pub status: DeploymentObservedState,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollout_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployed_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployed_at: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_hardened: Option<bool>,
}

impl ManagedDeploymentResponse {
    pub fn builder() -> ManagedDeploymentResponseBuilder {
        <ManagedDeploymentResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ManagedDeploymentResponseBuilder {
    service_id: Option<String>,
    name: Option<String>,
    spec: Option<DeploymentWorkerSpecResponse>,
    resources: Option<DeploymentResourceConfig>,
    status: Option<DeploymentObservedState>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
    stopped: Option<bool>,
    rollout_status: Option<String>,
    created_by: Option<String>,
    updated_by: Option<String>,
    deployed_by: Option<String>,
    deployed_at: Option<DateTime<FixedOffset>>,
    is_hardened: Option<bool>,
}

impl ManagedDeploymentResponseBuilder {
    pub fn service_id(mut self, value: impl Into<String>) -> Self {
        self.service_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn spec(mut self, value: DeploymentWorkerSpecResponse) -> Self {
        self.spec = Some(value);
        self
    }

    pub fn resources(mut self, value: DeploymentResourceConfig) -> Self {
        self.resources = Some(value);
        self
    }

    pub fn status(mut self, value: DeploymentObservedState) -> Self {
        self.status = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn stopped(mut self, value: bool) -> Self {
        self.stopped = Some(value);
        self
    }

    pub fn rollout_status(mut self, value: impl Into<String>) -> Self {
        self.rollout_status = Some(value.into());
        self
    }

    pub fn created_by(mut self, value: impl Into<String>) -> Self {
        self.created_by = Some(value.into());
        self
    }

    pub fn updated_by(mut self, value: impl Into<String>) -> Self {
        self.updated_by = Some(value.into());
        self
    }

    pub fn deployed_by(mut self, value: impl Into<String>) -> Self {
        self.deployed_by = Some(value.into());
        self
    }

    pub fn deployed_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.deployed_at = Some(value);
        self
    }

    pub fn is_hardened(mut self, value: bool) -> Self {
        self.is_hardened = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ManagedDeploymentResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`service_id`](ManagedDeploymentResponseBuilder::service_id)
    /// - [`name`](ManagedDeploymentResponseBuilder::name)
    /// - [`spec`](ManagedDeploymentResponseBuilder::spec)
    /// - [`resources`](ManagedDeploymentResponseBuilder::resources)
    /// - [`status`](ManagedDeploymentResponseBuilder::status)
    /// - [`created_at`](ManagedDeploymentResponseBuilder::created_at)
    /// - [`updated_at`](ManagedDeploymentResponseBuilder::updated_at)
    pub fn build(self) -> Result<ManagedDeploymentResponse, BuildError> {
        Ok(ManagedDeploymentResponse {
            service_id: self
                .service_id
                .ok_or_else(|| BuildError::missing_field("service_id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            spec: self.spec.ok_or_else(|| BuildError::missing_field("spec"))?,
            resources: self
                .resources
                .ok_or_else(|| BuildError::missing_field("resources"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            stopped: self.stopped,
            rollout_status: self.rollout_status,
            created_by: self.created_by,
            updated_by: self.updated_by,
            deployed_by: self.deployed_by,
            deployed_at: self.deployed_at,
            is_hardened: self.is_hardened,
        })
    }
}
