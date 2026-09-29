pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateLibraryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl UpdateLibraryRequest {
    pub fn builder() -> UpdateLibraryRequestBuilder {
        <UpdateLibraryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateLibraryRequestBuilder {
    description: Option<String>,
    name: Option<String>,
}

impl UpdateLibraryRequestBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateLibraryRequest`].
    pub fn build(self) -> Result<UpdateLibraryRequest, BuildError> {
        Ok(UpdateLibraryRequest {
            description: self.description,
            name: self.name,
        })
    }
}
