pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateVersionMetadataPromptsRequest {
    /// Aliases pointing to this version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<AliasList>,
    /// Notes for this version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl UpdateVersionMetadataPromptsRequest {
    pub fn builder() -> UpdateVersionMetadataPromptsRequestBuilder {
        <UpdateVersionMetadataPromptsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateVersionMetadataPromptsRequestBuilder {
    aliases: Option<AliasList>,
    notes: Option<String>,
}

impl UpdateVersionMetadataPromptsRequestBuilder {
    pub fn aliases(mut self, value: AliasList) -> Self {
        self.aliases = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateVersionMetadataPromptsRequest`].
    pub fn build(self) -> Result<UpdateVersionMetadataPromptsRequest, BuildError> {
        Ok(UpdateVersionMetadataPromptsRequest {
            aliases: self.aliases,
            notes: self.notes,
        })
    }
}
