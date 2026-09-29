pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RegisterDeploymentRequestDeployment {
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<RegisterDeploymentRequestDeploymentStatus>,
    pub deployment: RegisterDeploymentRequestDeploymentDeployment,
}

impl RegisterDeploymentRequestDeployment {
    pub fn builder() -> RegisterDeploymentRequestDeploymentBuilder {
        <RegisterDeploymentRequestDeploymentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RegisterDeploymentRequestDeploymentBuilder {
    name: Option<String>,
    status: Option<RegisterDeploymentRequestDeploymentStatus>,
    deployment: Option<RegisterDeploymentRequestDeploymentDeployment>,
}

impl RegisterDeploymentRequestDeploymentBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn status(mut self, value: RegisterDeploymentRequestDeploymentStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn deployment(mut self, value: RegisterDeploymentRequestDeploymentDeployment) -> Self {
        self.deployment = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RegisterDeploymentRequestDeployment`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](RegisterDeploymentRequestDeploymentBuilder::name)
    /// - [`deployment`](RegisterDeploymentRequestDeploymentBuilder::deployment)
    pub fn build(self) -> Result<RegisterDeploymentRequestDeployment, BuildError> {
        Ok(RegisterDeploymentRequestDeployment {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            status: self.status,
            deployment: self
                .deployment
                .ok_or_else(|| BuildError::missing_field("deployment"))?,
        })
    }
}
