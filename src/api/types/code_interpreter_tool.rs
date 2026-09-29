pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CodeInterpreterTool {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_configuration: Option<ToolConfiguration>,
}

impl CodeInterpreterTool {
    pub fn builder() -> CodeInterpreterToolBuilder {
        <CodeInterpreterToolBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CodeInterpreterToolBuilder {
    tool_configuration: Option<ToolConfiguration>,
}

impl CodeInterpreterToolBuilder {
    pub fn tool_configuration(mut self, value: ToolConfiguration) -> Self {
        self.tool_configuration = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CodeInterpreterTool`].
    pub fn build(self) -> Result<CodeInterpreterTool, BuildError> {
        Ok(CodeInterpreterTool {
            tool_configuration: self.tool_configuration,
        })
    }
}
