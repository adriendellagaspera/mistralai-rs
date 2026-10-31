pub use crate::prelude::*;

/// Describes an argument that a prompt can accept.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PromptArgument {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl PromptArgument {
    pub fn builder() -> PromptArgumentBuilder {
        <PromptArgumentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PromptArgumentBuilder {
    description: Option<String>,
    name: Option<String>,
    required: Option<bool>,
    title: Option<String>,
}

impl PromptArgumentBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn required(mut self, value: bool) -> Self {
        self.required = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PromptArgument`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PromptArgumentBuilder::name)
    pub fn build(self) -> Result<PromptArgument, BuildError> {
        Ok(PromptArgument {
            description: self.description,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            required: self.required,
            title: self.title,
        })
    }
}
