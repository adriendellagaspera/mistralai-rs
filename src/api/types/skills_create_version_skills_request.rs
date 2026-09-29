pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SkillsCreateVersionSkillsRequest {
    /// Aliases pointing to this version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,
    #[serde(default)]
    pub definition: SkillDefinition,
    /// Notes for this version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl SkillsCreateVersionSkillsRequest {
    pub fn builder() -> SkillsCreateVersionSkillsRequestBuilder {
        <SkillsCreateVersionSkillsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SkillsCreateVersionSkillsRequestBuilder {
    aliases: Option<Vec<String>>,
    definition: Option<SkillDefinition>,
    notes: Option<String>,
}

impl SkillsCreateVersionSkillsRequestBuilder {
    pub fn aliases(mut self, value: Vec<String>) -> Self {
        self.aliases = Some(value);
        self
    }

    pub fn definition(mut self, value: SkillDefinition) -> Self {
        self.definition = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SkillsCreateVersionSkillsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`definition`](SkillsCreateVersionSkillsRequestBuilder::definition)
    pub fn build(self) -> Result<SkillsCreateVersionSkillsRequest, BuildError> {
        Ok(SkillsCreateVersionSkillsRequest {
            aliases: self.aliases,
            definition: self
                .definition
                .ok_or_else(|| BuildError::missing_field("definition"))?,
            notes: self.notes,
        })
    }
}
