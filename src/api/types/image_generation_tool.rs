pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ImageGenerationTool {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_configuration: Option<ToolConfiguration>,
}

impl ImageGenerationTool {
    pub fn builder() -> ImageGenerationToolBuilder {
        <ImageGenerationToolBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ImageGenerationToolBuilder {
    tool_configuration: Option<ToolConfiguration>,
}

impl ImageGenerationToolBuilder {
    pub fn tool_configuration(mut self, value: ToolConfiguration) -> Self {
        self.tool_configuration = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ImageGenerationTool`].
    pub fn build(self) -> Result<ImageGenerationTool, BuildError> {
        Ok(ImageGenerationTool {
            tool_configuration: self.tool_configuration,
        })
    }
}
