pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolExecutionEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<ToolExecutionEntryObject>,
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
    pub name: ToolExecutionEntryName,
    #[serde(default)]
    pub arguments: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<ToolExecutionInfo>,
}

impl ToolExecutionEntry {
    pub fn builder() -> ToolExecutionEntryBuilder {
        <ToolExecutionEntryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ToolExecutionEntryBuilder {
    object: Option<ToolExecutionEntryObject>,
    created_at: Option<DateTime<FixedOffset>>,
    completed_at: Option<DateTime<FixedOffset>>,
    agent_id: Option<String>,
    model: Option<String>,
    id: Option<String>,
    name: Option<ToolExecutionEntryName>,
    arguments: Option<String>,
    info: Option<ToolExecutionInfo>,
}

impl ToolExecutionEntryBuilder {
    pub fn object(mut self, value: ToolExecutionEntryObject) -> Self {
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

    pub fn name(mut self, value: ToolExecutionEntryName) -> Self {
        self.name = Some(value);
        self
    }

    pub fn arguments(mut self, value: impl Into<String>) -> Self {
        self.arguments = Some(value.into());
        self
    }

    pub fn info(mut self, value: ToolExecutionInfo) -> Self {
        self.info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ToolExecutionEntry`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ToolExecutionEntryBuilder::name)
    /// - [`arguments`](ToolExecutionEntryBuilder::arguments)
    pub fn build(self) -> Result<ToolExecutionEntry, BuildError> {
        Ok(ToolExecutionEntry {
            object: self.object,
            created_at: self.created_at,
            completed_at: self.completed_at,
            agent_id: self.agent_id,
            model: self.model,
            id: self.id,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            arguments: self
                .arguments
                .ok_or_else(|| BuildError::missing_field("arguments"))?,
            info: self.info,
        })
    }
}
