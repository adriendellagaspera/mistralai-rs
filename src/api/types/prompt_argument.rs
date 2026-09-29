pub use crate::prelude::*;

/// An argument for a prompt template.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PromptArgument {
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl PromptArgument {
    pub fn builder() -> PromptArgumentBuilder {
        <PromptArgumentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PromptArgumentBuilder {
    name: Option<String>,
    description: Option<String>,
    required: Option<bool>,
}

impl PromptArgumentBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn required(mut self, value: bool) -> Self {
        self.required = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PromptArgument`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PromptArgumentBuilder::name)
    pub fn build(self) -> Result<PromptArgument, BuildError> {
        Ok(PromptArgument {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            description: self.description,
            required: self.required,
            extra: Default::default(),
        })
    }
}
