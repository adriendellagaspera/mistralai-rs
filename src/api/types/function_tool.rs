pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FunctionTool {
    #[serde(default)]
    pub function: Function,
}

impl FunctionTool {
    pub fn builder() -> FunctionToolBuilder {
        <FunctionToolBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FunctionToolBuilder {
    function: Option<Function>,
}

impl FunctionToolBuilder {
    pub fn function(mut self, value: Function) -> Self {
        self.function = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FunctionTool`].
    /// This method will fail if any of the following fields are not set:
    /// - [`function`](FunctionToolBuilder::function)
    pub fn build(self) -> Result<FunctionTool, BuildError> {
        Ok(FunctionTool {
            function: self
                .function
                .ok_or_else(|| BuildError::missing_field("function"))?,
        })
    }
}
