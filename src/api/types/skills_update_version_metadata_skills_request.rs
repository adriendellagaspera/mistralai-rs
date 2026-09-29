pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SkillsUpdateVersionMetadataSkillsRequest {
    /// Notes for this version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// Aliases pointing to this version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<AliasList>,
}

impl SkillsUpdateVersionMetadataSkillsRequest {
    pub fn builder() -> SkillsUpdateVersionMetadataSkillsRequestBuilder {
        <SkillsUpdateVersionMetadataSkillsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SkillsUpdateVersionMetadataSkillsRequestBuilder {
    notes: Option<String>,
    aliases: Option<AliasList>,
}

impl SkillsUpdateVersionMetadataSkillsRequestBuilder {
    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn aliases(mut self, value: AliasList) -> Self {
        self.aliases = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SkillsUpdateVersionMetadataSkillsRequest`].
    pub fn build(self) -> Result<SkillsUpdateVersionMetadataSkillsRequest, BuildError> {
        Ok(SkillsUpdateVersionMetadataSkillsRequest {
            notes: self.notes,
            aliases: self.aliases,
        })
    }
}
