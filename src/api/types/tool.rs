pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Tool {
    #[serde(default)]
    pub function: Function,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ToolTypes>,
}

impl Tool {
    pub fn builder() -> ToolBuilder {
        <ToolBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ToolBuilder {
    function: Option<Function>,
    r#type: Option<ToolTypes>,
}

impl ToolBuilder {
    pub fn function(mut self, value: Function) -> Self {
        self.function = Some(value);
        self
    }

    pub fn r#type(mut self, value: ToolTypes) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Tool`].
    /// This method will fail if any of the following fields are not set:
    /// - [`function`](ToolBuilder::function)
    pub fn build(self) -> Result<Tool, BuildError> {
        Ok(Tool {
            function: self
                .function
                .ok_or_else(|| BuildError::missing_field("function"))?,
            r#type: self.r#type,
        })
    }
}
