pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateVersionMetadataSkillsRequest {
    /// Aliases pointing to this version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<AliasList>,
    /// Notes for this version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl UpdateVersionMetadataSkillsRequest {
    pub fn builder() -> UpdateVersionMetadataSkillsRequestBuilder {
        <UpdateVersionMetadataSkillsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateVersionMetadataSkillsRequestBuilder {
    aliases: Option<AliasList>,
    notes: Option<String>,
}

impl UpdateVersionMetadataSkillsRequestBuilder {
    pub fn aliases(mut self, value: AliasList) -> Self {
        self.aliases = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateVersionMetadataSkillsRequest`].
    pub fn build(self) -> Result<UpdateVersionMetadataSkillsRequest, BuildError> {
        Ok(UpdateVersionMetadataSkillsRequest {
            aliases: self.aliases,
            notes: self.notes,
        })
    }
}
