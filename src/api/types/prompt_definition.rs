pub use crate::prelude::*;

/// Versioned prompt content.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PromptDefinition {
    /// Prompt template content.
    #[serde(default)]
    pub content: String,
    /// Variables used by the prompt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<Vec<PromptVariable>>,
}

impl PromptDefinition {
    pub fn builder() -> PromptDefinitionBuilder {
        <PromptDefinitionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PromptDefinitionBuilder {
    content: Option<String>,
    variables: Option<Vec<PromptVariable>>,
}

impl PromptDefinitionBuilder {
    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    pub fn variables(mut self, value: Vec<PromptVariable>) -> Self {
        self.variables = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PromptDefinition`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](PromptDefinitionBuilder::content)
    pub fn build(self) -> Result<PromptDefinition, BuildError> {
        Ok(PromptDefinition {
            content: self
                .content
                .ok_or_else(|| BuildError::missing_field("content"))?,
            variables: self.variables,
        })
    }
}
