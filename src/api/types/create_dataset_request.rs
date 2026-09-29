pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateDatasetRequest {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
}

impl CreateDatasetRequest {
    pub fn builder() -> CreateDatasetRequestBuilder {
        <CreateDatasetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateDatasetRequestBuilder {
    name: Option<String>,
    description: Option<String>,
}

impl CreateDatasetRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateDatasetRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateDatasetRequestBuilder::name)
    /// - [`description`](CreateDatasetRequestBuilder::description)
    pub fn build(self) -> Result<CreateDatasetRequest, BuildError> {
        Ok(CreateDatasetRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
        })
    }
}
