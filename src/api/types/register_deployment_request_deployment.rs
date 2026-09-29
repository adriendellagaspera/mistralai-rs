pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RegisterDeploymentRequestDeployment {
    pub deployment: RegisterDeploymentRequestDeploymentDeployment,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<RegisterDeploymentRequestDeploymentStatus>,
}

impl RegisterDeploymentRequestDeployment {
    pub fn builder() -> RegisterDeploymentRequestDeploymentBuilder {
        <RegisterDeploymentRequestDeploymentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RegisterDeploymentRequestDeploymentBuilder {
    deployment: Option<RegisterDeploymentRequestDeploymentDeployment>,
    name: Option<String>,
    status: Option<RegisterDeploymentRequestDeploymentStatus>,
}

impl RegisterDeploymentRequestDeploymentBuilder {
    pub fn deployment(mut self, value: RegisterDeploymentRequestDeploymentDeployment) -> Self {
        self.deployment = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn status(mut self, value: RegisterDeploymentRequestDeploymentStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RegisterDeploymentRequestDeployment`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deployment`](RegisterDeploymentRequestDeploymentBuilder::deployment)
    /// - [`name`](RegisterDeploymentRequestDeploymentBuilder::name)
    pub fn build(self) -> Result<RegisterDeploymentRequestDeployment, BuildError> {
        Ok(RegisterDeploymentRequestDeployment {
            deployment: self
                .deployment
                .ok_or_else(|| BuildError::missing_field("deployment"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            status: self.status,
        })
    }
}
