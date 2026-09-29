pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolCall {
    pub function: FunctionCall,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ToolTypes>,
}

impl ToolCall {
    pub fn builder() -> ToolCallBuilder {
        <ToolCallBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ToolCallBuilder {
    function: Option<FunctionCall>,
    id: Option<String>,
    index: Option<i64>,
    r#type: Option<ToolTypes>,
}

impl ToolCallBuilder {
    pub fn function(mut self, value: FunctionCall) -> Self {
        self.function = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn index(mut self, value: i64) -> Self {
        self.index = Some(value);
        self
    }

    pub fn r#type(mut self, value: ToolTypes) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ToolCall`].
    /// This method will fail if any of the following fields are not set:
    /// - [`function`](ToolCallBuilder::function)
    pub fn build(self) -> Result<ToolCall, BuildError> {
        Ok(ToolCall {
            function: self
                .function
                .ok_or_else(|| BuildError::missing_field("function"))?,
            id: self.id,
            index: self.index,
            r#type: self.r#type,
        })
    }
}
