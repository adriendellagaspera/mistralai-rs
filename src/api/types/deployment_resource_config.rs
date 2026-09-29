pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeploymentResourceConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_limit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_request: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_limit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_request: Option<String>,
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
    cpu_limit: Option<String>,
    cpu_request: Option<String>,
    memory_limit: Option<String>,
    memory_request: Option<String>,
    replicas: Option<i64>,
}

impl DeploymentResourceConfigBuilder {
    pub fn cpu_limit(mut self, value: impl Into<String>) -> Self {
        self.cpu_limit = Some(value.into());
        self
    }

    pub fn cpu_request(mut self, value: impl Into<String>) -> Self {
        self.cpu_request = Some(value.into());
        self
    }

    pub fn memory_limit(mut self, value: impl Into<String>) -> Self {
        self.memory_limit = Some(value.into());
        self
    }

    pub fn memory_request(mut self, value: impl Into<String>) -> Self {
        self.memory_request = Some(value.into());
        self
    }

    pub fn replicas(mut self, value: i64) -> Self {
        self.replicas = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeploymentResourceConfig`].
    pub fn build(self) -> Result<DeploymentResourceConfig, BuildError> {
        Ok(DeploymentResourceConfig {
            cpu_limit: self.cpu_limit,
            cpu_request: self.cpu_request,
            memory_limit: self.memory_limit,
            memory_request: self.memory_request,
            replicas: self.replicas,
        })
    }
}
