pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateVersionPromptsRequest {
    /// Aliases pointing to this version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,
    #[serde(default)]
    pub definition: PromptDefinition,
    /// Notes for this version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl CreateVersionPromptsRequest {
    pub fn builder() -> CreateVersionPromptsRequestBuilder {
        <CreateVersionPromptsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateVersionPromptsRequestBuilder {
    aliases: Option<Vec<String>>,
    definition: Option<PromptDefinition>,
    notes: Option<String>,
}

impl CreateVersionPromptsRequestBuilder {
    pub fn aliases(mut self, value: Vec<String>) -> Self {
        self.aliases = Some(value);
        self
    }

    pub fn definition(mut self, value: PromptDefinition) -> Self {
        self.definition = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateVersionPromptsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`definition`](CreateVersionPromptsRequestBuilder::definition)
    pub fn build(self) -> Result<CreateVersionPromptsRequest, BuildError> {
        Ok(CreateVersionPromptsRequest {
            aliases: self.aliases,
            definition: self
                .definition
                .ok_or_else(|| BuildError::missing_field("definition"))?,
            notes: self.notes,
        })
    }
}
