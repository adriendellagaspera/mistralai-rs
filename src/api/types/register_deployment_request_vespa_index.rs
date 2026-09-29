pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RegisterDeploymentRequestVespaIndex {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub fields: Vec<RegisterDeploymentRequestVespaField>,
    #[serde(default)]
    pub sd: String,
}

impl RegisterDeploymentRequestVespaIndex {
    pub fn builder() -> RegisterDeploymentRequestVespaIndexBuilder {
        <RegisterDeploymentRequestVespaIndexBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RegisterDeploymentRequestVespaIndexBuilder {
    name: Option<String>,
    fields: Option<Vec<RegisterDeploymentRequestVespaField>>,
    sd: Option<String>,
}

impl RegisterDeploymentRequestVespaIndexBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn fields(mut self, value: Vec<RegisterDeploymentRequestVespaField>) -> Self {
        self.fields = Some(value);
        self
    }

    pub fn sd(mut self, value: impl Into<String>) -> Self {
        self.sd = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RegisterDeploymentRequestVespaIndex`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](RegisterDeploymentRequestVespaIndexBuilder::name)
    /// - [`fields`](RegisterDeploymentRequestVespaIndexBuilder::fields)
    /// - [`sd`](RegisterDeploymentRequestVespaIndexBuilder::sd)
    pub fn build(self) -> Result<RegisterDeploymentRequestVespaIndex, BuildError> {
        Ok(RegisterDeploymentRequestVespaIndex {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            fields: self
                .fields
                .ok_or_else(|| BuildError::missing_field("fields"))?,
            sd: self.sd.ok_or_else(|| BuildError::missing_field("sd"))?,
        })
    }
}
