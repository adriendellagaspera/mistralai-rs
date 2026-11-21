pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Tool {
    #[serde(default)]
    pub function: Function,
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
}

impl ToolBuilder {
    pub fn function(mut self, value: Function) -> Self {
        self.function = Some(value);
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
        })
    }
}
