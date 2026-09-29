pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ToolCallConfirmation {
    #[serde(default)]
    pub tool_call_id: String,
    pub confirmation: ToolCallConfirmationConfirmation,
}

impl ToolCallConfirmation {
    pub fn builder() -> ToolCallConfirmationBuilder {
        <ToolCallConfirmationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ToolCallConfirmationBuilder {
    tool_call_id: Option<String>,
    confirmation: Option<ToolCallConfirmationConfirmation>,
}

impl ToolCallConfirmationBuilder {
    pub fn tool_call_id(mut self, value: impl Into<String>) -> Self {
        self.tool_call_id = Some(value.into());
        self
    }

    pub fn confirmation(mut self, value: ToolCallConfirmationConfirmation) -> Self {
        self.confirmation = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ToolCallConfirmation`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tool_call_id`](ToolCallConfirmationBuilder::tool_call_id)
    /// - [`confirmation`](ToolCallConfirmationBuilder::confirmation)
    pub fn build(self) -> Result<ToolCallConfirmation, BuildError> {
        Ok(ToolCallConfirmation {
            tool_call_id: self
                .tool_call_id
                .ok_or_else(|| BuildError::missing_field("tool_call_id"))?,
            confirmation: self
                .confirmation
                .ok_or_else(|| BuildError::missing_field("confirmation"))?,
        })
    }
}
