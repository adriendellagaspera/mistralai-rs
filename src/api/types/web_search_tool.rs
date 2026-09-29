pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WebSearchTool {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_configuration: Option<ToolConfiguration>,
}

impl WebSearchTool {
    pub fn builder() -> WebSearchToolBuilder {
        <WebSearchToolBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebSearchToolBuilder {
    tool_configuration: Option<ToolConfiguration>,
}

impl WebSearchToolBuilder {
    pub fn tool_configuration(mut self, value: ToolConfiguration) -> Self {
        self.tool_configuration = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WebSearchTool`].
    pub fn build(self) -> Result<WebSearchTool, BuildError> {
        Ok(WebSearchTool {
            tool_configuration: self.tool_configuration,
        })
    }
}
