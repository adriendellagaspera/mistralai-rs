pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PromptsUpdateVersionMetadataPromptsRequest {
    /// Notes for this version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// Aliases pointing to this version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<AliasList>,
}

impl PromptsUpdateVersionMetadataPromptsRequest {
    pub fn builder() -> PromptsUpdateVersionMetadataPromptsRequestBuilder {
        <PromptsUpdateVersionMetadataPromptsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PromptsUpdateVersionMetadataPromptsRequestBuilder {
    notes: Option<String>,
    aliases: Option<AliasList>,
}

impl PromptsUpdateVersionMetadataPromptsRequestBuilder {
    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn aliases(mut self, value: AliasList) -> Self {
        self.aliases = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PromptsUpdateVersionMetadataPromptsRequest`].
    pub fn build(self) -> Result<PromptsUpdateVersionMetadataPromptsRequest, BuildError> {
        Ok(PromptsUpdateVersionMetadataPromptsRequest {
            notes: self.notes,
            aliases: self.aliases,
        })
    }
}
