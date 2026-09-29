pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DeploymentLocation {
    /// K8s cluster name, if applicable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k8s_cluster: Option<String>,
    /// K8s namespace, if applicable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k8s_namespace: Option<String>,
    /// Where the deployment runs: 'local', 'k8s', or 'managed'
    pub location_type: LocationType,
}

impl DeploymentLocation {
    pub fn builder() -> DeploymentLocationBuilder {
        <DeploymentLocationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeploymentLocationBuilder {
    k8s_cluster: Option<String>,
    k8s_namespace: Option<String>,
    location_type: Option<LocationType>,
}

impl DeploymentLocationBuilder {
    pub fn k8s_cluster(mut self, value: impl Into<String>) -> Self {
        self.k8s_cluster = Some(value.into());
        self
    }

    pub fn k8s_namespace(mut self, value: impl Into<String>) -> Self {
        self.k8s_namespace = Some(value.into());
        self
    }

    pub fn location_type(mut self, value: LocationType) -> Self {
        self.location_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeploymentLocation`].
    /// This method will fail if any of the following fields are not set:
    /// - [`location_type`](DeploymentLocationBuilder::location_type)
    pub fn build(self) -> Result<DeploymentLocation, BuildError> {
        Ok(DeploymentLocation {
            k8s_cluster: self.k8s_cluster,
            k8s_namespace: self.k8s_namespace,
            location_type: self
                .location_type
                .ok_or_else(|| BuildError::missing_field("location_type"))?,
        })
    }
}
