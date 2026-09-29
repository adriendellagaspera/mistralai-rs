pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ToolCallConfirmation {
    pub confirmation: ToolCallConfirmationConfirmation,
    #[serde(default)]
    pub tool_call_id: String,
}

impl ToolCallConfirmation {
    pub fn builder() -> ToolCallConfirmationBuilder {
        <ToolCallConfirmationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ToolCallConfirmationBuilder {
    confirmation: Option<ToolCallConfirmationConfirmation>,
    tool_call_id: Option<String>,
}

impl ToolCallConfirmationBuilder {
    pub fn confirmation(mut self, value: ToolCallConfirmationConfirmation) -> Self {
        self.confirmation = Some(value);
        self
    }

    pub fn tool_call_id(mut self, value: impl Into<String>) -> Self {
        self.tool_call_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ToolCallConfirmation`].
    /// This method will fail if any of the following fields are not set:
    /// - [`confirmation`](ToolCallConfirmationBuilder::confirmation)
    /// - [`tool_call_id`](ToolCallConfirmationBuilder::tool_call_id)
    pub fn build(self) -> Result<ToolCallConfirmation, BuildError> {
        Ok(ToolCallConfirmation {
            confirmation: self
                .confirmation
                .ok_or_else(|| BuildError::missing_field("confirmation"))?,
            tool_call_id: self
                .tool_call_id
                .ok_or_else(|| BuildError::missing_field("tool_call_id"))?,
        })
    }
}
