pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateModelRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl UpdateModelRequest {
    pub fn builder() -> UpdateModelRequestBuilder {
        <UpdateModelRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateModelRequestBuilder {
    description: Option<String>,
    name: Option<String>,
}

impl UpdateModelRequestBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateModelRequest`].
    pub fn build(self) -> Result<UpdateModelRequest, BuildError> {
        Ok(UpdateModelRequest {
            description: self.description,
            name: self.name,
        })
    }
}
