pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FunctionCallEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    pub arguments: FunctionCallEntryArguments,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmation_status: Option<FunctionCallEntryConfirmationStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_at: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<FunctionCallEntryObject>,
    #[serde(default)]
    pub tool_call_id: String,
}

impl FunctionCallEntry {
    pub fn builder() -> FunctionCallEntryBuilder {
        <FunctionCallEntryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FunctionCallEntryBuilder {
    agent_id: Option<String>,
    arguments: Option<FunctionCallEntryArguments>,
    completed_at: Option<DateTime<FixedOffset>>,
    confirmation_status: Option<FunctionCallEntryConfirmationStatus>,
    created_at: Option<DateTime<FixedOffset>>,
    id: Option<String>,
    model: Option<String>,
    name: Option<String>,
    object: Option<FunctionCallEntryObject>,
    tool_call_id: Option<String>,
}

impl FunctionCallEntryBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn arguments(mut self, value: FunctionCallEntryArguments) -> Self {
        self.arguments = Some(value);
        self
    }

    pub fn completed_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.completed_at = Some(value);
        self
    }

    pub fn confirmation_status(mut self, value: FunctionCallEntryConfirmationStatus) -> Self {
        self.confirmation_status = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn object(mut self, value: FunctionCallEntryObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn tool_call_id(mut self, value: impl Into<String>) -> Self {
        self.tool_call_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FunctionCallEntry`].
    /// This method will fail if any of the following fields are not set:
    /// - [`arguments`](FunctionCallEntryBuilder::arguments)
    /// - [`name`](FunctionCallEntryBuilder::name)
    /// - [`tool_call_id`](FunctionCallEntryBuilder::tool_call_id)
    pub fn build(self) -> Result<FunctionCallEntry, BuildError> {
        Ok(FunctionCallEntry {
            agent_id: self.agent_id,
            arguments: self
                .arguments
                .ok_or_else(|| BuildError::missing_field("arguments"))?,
            completed_at: self.completed_at,
            confirmation_status: self.confirmation_status,
            created_at: self.created_at,
            id: self.id,
            model: self.model,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            object: self.object,
            tool_call_id: self
                .tool_call_id
                .ok_or_else(|| BuildError::missing_field("tool_call_id"))?,
        })
    }
}
