pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WebSearchPremiumTool {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_configuration: Option<ToolConfiguration>,
}

impl WebSearchPremiumTool {
    pub fn builder() -> WebSearchPremiumToolBuilder {
        <WebSearchPremiumToolBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebSearchPremiumToolBuilder {
    tool_configuration: Option<ToolConfiguration>,
}

impl WebSearchPremiumToolBuilder {
    pub fn tool_configuration(mut self, value: ToolConfiguration) -> Self {
        self.tool_configuration = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WebSearchPremiumTool`].
    pub fn build(self) -> Result<WebSearchPremiumTool, BuildError> {
        Ok(WebSearchPremiumTool {
            tool_configuration: self.tool_configuration,
        })
    }
}
