pub use crate::prelude::*;

/// ToolChoice is either a ToolChoiceEnum or a ToolChoice
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ToolChoice {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ToolTypes>,
    #[serde(default)]
    pub function: FunctionName,
}

impl ToolChoice {
    pub fn builder() -> ToolChoiceBuilder {
        <ToolChoiceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ToolChoiceBuilder {
    r#type: Option<ToolTypes>,
    function: Option<FunctionName>,
}

impl ToolChoiceBuilder {
    pub fn r#type(mut self, value: ToolTypes) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn function(mut self, value: FunctionName) -> Self {
        self.function = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ToolChoice`].
    /// This method will fail if any of the following fields are not set:
    /// - [`function`](ToolChoiceBuilder::function)
    pub fn build(self) -> Result<ToolChoice, BuildError> {
        Ok(ToolChoice {
            r#type: self.r#type,
            function: self
                .function
                .ok_or_else(|| BuildError::missing_field("function"))?,
        })
    }
}
