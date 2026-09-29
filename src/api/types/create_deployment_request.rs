pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateDeploymentRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hardened: Option<bool>,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<DeploymentResourceConfig>,
    #[serde(default)]
    pub spec: DeploymentWorkerSpecInput,
}

impl CreateDeploymentRequest {
    pub fn builder() -> CreateDeploymentRequestBuilder {
        <CreateDeploymentRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateDeploymentRequestBuilder {
    hardened: Option<bool>,
    name: Option<String>,
    resources: Option<DeploymentResourceConfig>,
    spec: Option<DeploymentWorkerSpecInput>,
}

impl CreateDeploymentRequestBuilder {
    pub fn hardened(mut self, value: bool) -> Self {
        self.hardened = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn resources(mut self, value: DeploymentResourceConfig) -> Self {
        self.resources = Some(value);
        self
    }

    pub fn spec(mut self, value: DeploymentWorkerSpecInput) -> Self {
        self.spec = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateDeploymentRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateDeploymentRequestBuilder::name)
    /// - [`spec`](CreateDeploymentRequestBuilder::spec)
    pub fn build(self) -> Result<CreateDeploymentRequest, BuildError> {
        Ok(CreateDeploymentRequest {
            hardened: self.hardened,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            resources: self.resources,
            spec: self.spec.ok_or_else(|| BuildError::missing_field("spec"))?,
        })
    }
}
