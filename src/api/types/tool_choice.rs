pub use crate::prelude::*;

/// ToolChoice is either a ToolChoiceEnum or a ToolChoice
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ToolChoice {
    #[serde(default)]
    pub function: FunctionName,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ToolTypes>,
}

impl ToolChoice {
    pub fn builder() -> ToolChoiceBuilder {
        <ToolChoiceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ToolChoiceBuilder {
    function: Option<FunctionName>,
    r#type: Option<ToolTypes>,
}

impl ToolChoiceBuilder {
    pub fn function(mut self, value: FunctionName) -> Self {
        self.function = Some(value);
        self
    }

    pub fn r#type(mut self, value: ToolTypes) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ToolChoice`].
    /// This method will fail if any of the following fields are not set:
    /// - [`function`](ToolChoiceBuilder::function)
    pub fn build(self) -> Result<ToolChoice, BuildError> {
        Ok(ToolChoice {
            function: self
                .function
                .ok_or_else(|| BuildError::missing_field("function"))?,
            r#type: self.r#type,
        })
    }
}
