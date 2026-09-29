pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreatePromptRequest {
    /// Aliases pointing to this version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,
    #[serde(default)]
    pub definition: PromptDefinition,
    /// Display description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Stable object name.
    #[serde(default)]
    pub name: String,
    /// Notes for this version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// Registry sharing scope.
    #[serde(rename = "sharingScope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharing_scope: Option<RegistrySharingScope>,
    /// Display title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl CreatePromptRequest {
    pub fn builder() -> CreatePromptRequestBuilder {
        <CreatePromptRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePromptRequestBuilder {
    aliases: Option<Vec<String>>,
    definition: Option<PromptDefinition>,
    description: Option<String>,
    name: Option<String>,
    notes: Option<String>,
    sharing_scope: Option<RegistrySharingScope>,
    title: Option<String>,
}

impl CreatePromptRequestBuilder {
    pub fn aliases(mut self, value: Vec<String>) -> Self {
        self.aliases = Some(value);
        self
    }

    pub fn definition(mut self, value: PromptDefinition) -> Self {
        self.definition = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
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

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreatePromptRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`definition`](CreatePromptRequestBuilder::definition)
    /// - [`name`](CreatePromptRequestBuilder::name)
    pub fn build(self) -> Result<CreatePromptRequest, BuildError> {
        Ok(CreatePromptRequest {
            aliases: self.aliases,
            definition: self
                .definition
                .ok_or_else(|| BuildError::missing_field("definition"))?,
            description: self.description,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            notes: self.notes,
            sharing_scope: self.sharing_scope,
            title: self.title,
        })
    }
}
