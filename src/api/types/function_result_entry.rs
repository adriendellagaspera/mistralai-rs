pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FunctionResultEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_at: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<FunctionResultEntryObject>,
    #[serde(default)]
    pub result: String,
    #[serde(default)]
    pub tool_call_id: String,
}

impl FunctionResultEntry {
    pub fn builder() -> FunctionResultEntryBuilder {
        <FunctionResultEntryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FunctionResultEntryBuilder {
    completed_at: Option<DateTime<FixedOffset>>,
    created_at: Option<DateTime<FixedOffset>>,
    id: Option<String>,
    object: Option<FunctionResultEntryObject>,
    result: Option<String>,
    tool_call_id: Option<String>,
}

impl FunctionResultEntryBuilder {
    pub fn completed_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.completed_at = Some(value);
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

    pub fn object(mut self, value: FunctionResultEntryObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn result(mut self, value: impl Into<String>) -> Self {
        self.result = Some(value.into());
        self
    }

    pub fn tool_call_id(mut self, value: impl Into<String>) -> Self {
        self.tool_call_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FunctionResultEntry`].
    /// This method will fail if any of the following fields are not set:
    /// - [`result`](FunctionResultEntryBuilder::result)
    /// - [`tool_call_id`](FunctionResultEntryBuilder::tool_call_id)
    pub fn build(self) -> Result<FunctionResultEntry, BuildError> {
        Ok(FunctionResultEntry {
            completed_at: self.completed_at,
            created_at: self.created_at,
            id: self.id,
            object: self.object,
            result: self
                .result
                .ok_or_else(|| BuildError::missing_field("result"))?,
            tool_call_id: self
                .tool_call_id
                .ok_or_else(|| BuildError::missing_field("tool_call_id"))?,
        })
    }
}
