pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreatePromptRequest {
    /// Stable object name.
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub definition: PromptDefinition,
    /// Display title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Display description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
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

impl CreatePromptRequest {
    pub fn builder() -> CreatePromptRequestBuilder {
        <CreatePromptRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePromptRequestBuilder {
    name: Option<String>,
    definition: Option<PromptDefinition>,
    title: Option<String>,
    description: Option<String>,
    notes: Option<String>,
    sharing_scope: Option<RegistrySharingScope>,
    aliases: Option<Vec<String>>,
}

impl CreatePromptRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn definition(mut self, value: PromptDefinition) -> Self {
        self.definition = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
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

    /// Consumes the builder and constructs a [`CreatePromptRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreatePromptRequestBuilder::name)
    /// - [`definition`](CreatePromptRequestBuilder::definition)
    pub fn build(self) -> Result<CreatePromptRequest, BuildError> {
        Ok(CreatePromptRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            definition: self
                .definition
                .ok_or_else(|| BuildError::missing_field("definition"))?,
            title: self.title,
            description: self.description,
            notes: self.notes,
            sharing_scope: self.sharing_scope,
            aliases: self.aliases,
        })
    }
}
