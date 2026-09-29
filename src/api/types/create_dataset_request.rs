pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateDatasetRequest {
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub name: String,
}

impl CreateDatasetRequest {
    pub fn builder() -> CreateDatasetRequestBuilder {
        <CreateDatasetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateDatasetRequestBuilder {
    description: Option<String>,
    name: Option<String>,
}

impl CreateDatasetRequestBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateDatasetRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`description`](CreateDatasetRequestBuilder::description)
    /// - [`name`](CreateDatasetRequestBuilder::name)
    pub fn build(self) -> Result<CreateDatasetRequest, BuildError> {
        Ok(CreateDatasetRequest {
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
