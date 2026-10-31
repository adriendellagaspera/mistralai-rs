pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ResponsesNamespaceTool {
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub tools: Vec<ResponsesNamespaceToolToolsItem>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl ResponsesNamespaceTool {
    pub fn builder() -> ResponsesNamespaceToolBuilder {
        <ResponsesNamespaceToolBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResponsesNamespaceToolBuilder {
    description: Option<String>,
    name: Option<String>,
    tools: Option<Vec<ResponsesNamespaceToolToolsItem>>,
}

impl ResponsesNamespaceToolBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn tools(mut self, value: Vec<ResponsesNamespaceToolToolsItem>) -> Self {
        self.tools = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResponsesNamespaceTool`].
    /// This method will fail if any of the following fields are not set:
    /// - [`description`](ResponsesNamespaceToolBuilder::description)
    /// - [`name`](ResponsesNamespaceToolBuilder::name)
    /// - [`tools`](ResponsesNamespaceToolBuilder::tools)
    pub fn build(self) -> Result<ResponsesNamespaceTool, BuildError> {
        Ok(ResponsesNamespaceTool {
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            tools: self
                .tools
                .ok_or_else(|| BuildError::missing_field("tools"))?,
            extra: Default::default(),
        })
    }
}
