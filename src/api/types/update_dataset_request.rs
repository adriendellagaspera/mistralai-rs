pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateDatasetRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl UpdateDatasetRequest {
    pub fn builder() -> UpdateDatasetRequestBuilder {
        <UpdateDatasetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateDatasetRequestBuilder {
    name: Option<String>,
    description: Option<String>,
}

impl UpdateDatasetRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateDatasetRequest`].
    pub fn build(self) -> Result<UpdateDatasetRequest, BuildError> {
        Ok(UpdateDatasetRequest {
            name: self.name,
            description: self.description,
        })
    }
}
