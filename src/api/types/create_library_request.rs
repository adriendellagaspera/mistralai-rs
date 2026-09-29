pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateLibraryRequest {
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The size of the chunks (in characters) to split document text into. Must be between 256 and 32768.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chunk_size: Option<i64>,
    /// Determines who owns the created library. 'User' creates a private library accessible only to its owner. 'Workspace' creates a library shared with the workspace. Defaults to 'Workspace' for API key sessions. Only API keys with the 'Private and shared connectors' connector access scope can create private, user-owned libraries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_type: Option<CreateLibraryRequestOwnerType>,
}

impl CreateLibraryRequest {
    pub fn builder() -> CreateLibraryRequestBuilder {
        <CreateLibraryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateLibraryRequestBuilder {
    name: Option<String>,
    description: Option<String>,
    chunk_size: Option<i64>,
    owner_type: Option<CreateLibraryRequestOwnerType>,
}

impl CreateLibraryRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn chunk_size(mut self, value: i64) -> Self {
        self.chunk_size = Some(value);
        self
    }

    pub fn owner_type(mut self, value: CreateLibraryRequestOwnerType) -> Self {
        self.owner_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateLibraryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateLibraryRequestBuilder::name)
    pub fn build(self) -> Result<CreateLibraryRequest, BuildError> {
        Ok(CreateLibraryRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            description: self.description,
            chunk_size: self.chunk_size,
            owner_type: self.owner_type,
        })
    }
}
