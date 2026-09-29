pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateLibraryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl UpdateLibraryRequest {
    pub fn builder() -> UpdateLibraryRequestBuilder {
        <UpdateLibraryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateLibraryRequestBuilder {
    name: Option<String>,
    description: Option<String>,
}

impl UpdateLibraryRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateLibraryRequest`].
    pub fn build(self) -> Result<UpdateLibraryRequest, BuildError> {
        Ok(UpdateLibraryRequest {
            name: self.name,
            description: self.description,
        })
    }
}
