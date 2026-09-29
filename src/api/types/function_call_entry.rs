pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FunctionCallEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<FunctionCallEntryObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_at: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    pub tool_call_id: String,
    #[serde(default)]
    pub name: String,
    pub arguments: FunctionCallEntryArguments,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmation_status: Option<FunctionCallEntryConfirmationStatus>,
}

impl FunctionCallEntry {
    pub fn builder() -> FunctionCallEntryBuilder {
        <FunctionCallEntryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FunctionCallEntryBuilder {
    object: Option<FunctionCallEntryObject>,
    created_at: Option<DateTime<FixedOffset>>,
    completed_at: Option<DateTime<FixedOffset>>,
    agent_id: Option<String>,
    model: Option<String>,
    id: Option<String>,
    tool_call_id: Option<String>,
    name: Option<String>,
    arguments: Option<FunctionCallEntryArguments>,
    confirmation_status: Option<FunctionCallEntryConfirmationStatus>,
}

impl FunctionCallEntryBuilder {
    pub fn object(mut self, value: FunctionCallEntryObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn completed_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.completed_at = Some(value);
        self
    }

    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn tool_call_id(mut self, value: impl Into<String>) -> Self {
        self.tool_call_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn arguments(mut self, value: FunctionCallEntryArguments) -> Self {
        self.arguments = Some(value);
        self
    }

    pub fn confirmation_status(mut self, value: FunctionCallEntryConfirmationStatus) -> Self {
        self.confirmation_status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FunctionCallEntry`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tool_call_id`](FunctionCallEntryBuilder::tool_call_id)
    /// - [`name`](FunctionCallEntryBuilder::name)
    /// - [`arguments`](FunctionCallEntryBuilder::arguments)
    pub fn build(self) -> Result<FunctionCallEntry, BuildError> {
        Ok(FunctionCallEntry {
            object: self.object,
            created_at: self.created_at,
            completed_at: self.completed_at,
            agent_id: self.agent_id,
            model: self.model,
            id: self.id,
            tool_call_id: self
                .tool_call_id
                .ok_or_else(|| BuildError::missing_field("tool_call_id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            arguments: self
                .arguments
                .ok_or_else(|| BuildError::missing_field("arguments"))?,
            confirmation_status: self.confirmation_status,
        })
    }
}
