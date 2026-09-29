pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateDatasetRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl UpdateDatasetRequest {
    pub fn builder() -> UpdateDatasetRequestBuilder {
        <UpdateDatasetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateDatasetRequestBuilder {
    description: Option<String>,
    name: Option<String>,
}

impl UpdateDatasetRequestBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateDatasetRequest`].
    pub fn build(self) -> Result<UpdateDatasetRequest, BuildError> {
        Ok(UpdateDatasetRequest {
            description: self.description,
            name: self.name,
        })
    }
}
