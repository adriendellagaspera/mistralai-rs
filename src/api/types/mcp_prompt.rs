pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct McpPrompt {
    #[serde(rename = "_meta")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Vec<PromptArgument>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icons: Option<Vec<McpServerIcon>>,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl McpPrompt {
    pub fn builder() -> McpPromptBuilder {
        <McpPromptBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct McpPromptBuilder {
    meta: Option<HashMap<String, serde_json::Value>>,
    arguments: Option<Vec<PromptArgument>>,
    description: Option<String>,
    icons: Option<Vec<McpServerIcon>>,
    name: Option<String>,
    title: Option<String>,
}

impl McpPromptBuilder {
    pub fn meta(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.meta = Some(value);
        self
    }

    pub fn arguments(mut self, value: Vec<PromptArgument>) -> Self {
        self.arguments = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn icons(mut self, value: Vec<McpServerIcon>) -> Self {
        self.icons = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`McpPrompt`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](McpPromptBuilder::name)
    pub fn build(self) -> Result<McpPrompt, BuildError> {
        Ok(McpPrompt {
            meta: self.meta,
            arguments: self.arguments,
            description: self.description,
            icons: self.icons,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            title: self.title,
            extra: Default::default(),
        })
    }
}
