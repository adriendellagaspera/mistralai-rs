pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeploymentResourceConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i64>,
}

impl DeploymentResourceConfig {
    pub fn builder() -> DeploymentResourceConfigBuilder {
        <DeploymentResourceConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeploymentResourceConfigBuilder {
    replicas: Option<i64>,
}

impl DeploymentResourceConfigBuilder {
    pub fn replicas(mut self, value: i64) -> Self {
        self.replicas = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeploymentResourceConfig`].
    pub fn build(self) -> Result<DeploymentResourceConfig, BuildError> {
        Ok(DeploymentResourceConfig {
            replicas: self.replicas,
        })
    }
}
