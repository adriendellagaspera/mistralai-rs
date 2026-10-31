pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeploymentResourceConfigUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i64>,
}

impl DeploymentResourceConfigUpdate {
    pub fn builder() -> DeploymentResourceConfigUpdateBuilder {
        <DeploymentResourceConfigUpdateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeploymentResourceConfigUpdateBuilder {
    replicas: Option<i64>,
}

impl DeploymentResourceConfigUpdateBuilder {
    pub fn replicas(mut self, value: i64) -> Self {
        self.replicas = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeploymentResourceConfigUpdate`].
    pub fn build(self) -> Result<DeploymentResourceConfigUpdate, BuildError> {
        Ok(DeploymentResourceConfigUpdate {
            replicas: self.replicas,
        })
    }
}
