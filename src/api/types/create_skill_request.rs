pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateSkillRequest {
    /// Stable object name.
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub definition: SkillDefinition,
    /// Notes for this version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// Registry sharing scope.
    #[serde(rename = "sharingScope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharing_scope: Option<RegistrySharingScope>,
    /// Aliases pointing to this version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,
}

impl CreateSkillRequest {
    pub fn builder() -> CreateSkillRequestBuilder {
        <CreateSkillRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateSkillRequestBuilder {
    name: Option<String>,
    definition: Option<SkillDefinition>,
    notes: Option<String>,
    sharing_scope: Option<RegistrySharingScope>,
    aliases: Option<Vec<String>>,
}

impl CreateSkillRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
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

    pub fn sharing_scope(mut self, value: RegistrySharingScope) -> Self {
        self.sharing_scope = Some(value);
        self
    }

    pub fn aliases(mut self, value: Vec<String>) -> Self {
        self.aliases = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateSkillRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateSkillRequestBuilder::name)
    /// - [`definition`](CreateSkillRequestBuilder::definition)
    pub fn build(self) -> Result<CreateSkillRequest, BuildError> {
        Ok(CreateSkillRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            definition: self
                .definition
                .ok_or_else(|| BuildError::missing_field("definition"))?,
            notes: self.notes,
            sharing_scope: self.sharing_scope,
            aliases: self.aliases,
        })
    }
}
