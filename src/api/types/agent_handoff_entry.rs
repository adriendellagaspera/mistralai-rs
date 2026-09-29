pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentHandoffEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<AgentHandoffEntryObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_at: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    pub previous_agent_id: String,
    #[serde(default)]
    pub previous_agent_name: String,
    #[serde(default)]
    pub next_agent_id: String,
    #[serde(default)]
    pub next_agent_name: String,
}

impl AgentHandoffEntry {
    pub fn builder() -> AgentHandoffEntryBuilder {
        <AgentHandoffEntryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentHandoffEntryBuilder {
    object: Option<AgentHandoffEntryObject>,
    created_at: Option<DateTime<FixedOffset>>,
    completed_at: Option<DateTime<FixedOffset>>,
    id: Option<String>,
    previous_agent_id: Option<String>,
    previous_agent_name: Option<String>,
    next_agent_id: Option<String>,
    next_agent_name: Option<String>,
}

impl AgentHandoffEntryBuilder {
    pub fn object(mut self, value: AgentHandoffEntryObject) -> Self {
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

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn previous_agent_id(mut self, value: impl Into<String>) -> Self {
        self.previous_agent_id = Some(value.into());
        self
    }

    pub fn previous_agent_name(mut self, value: impl Into<String>) -> Self {
        self.previous_agent_name = Some(value.into());
        self
    }

    pub fn next_agent_id(mut self, value: impl Into<String>) -> Self {
        self.next_agent_id = Some(value.into());
        self
    }

    pub fn next_agent_name(mut self, value: impl Into<String>) -> Self {
        self.next_agent_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentHandoffEntry`].
    /// This method will fail if any of the following fields are not set:
    /// - [`previous_agent_id`](AgentHandoffEntryBuilder::previous_agent_id)
    /// - [`previous_agent_name`](AgentHandoffEntryBuilder::previous_agent_name)
    /// - [`next_agent_id`](AgentHandoffEntryBuilder::next_agent_id)
    /// - [`next_agent_name`](AgentHandoffEntryBuilder::next_agent_name)
    pub fn build(self) -> Result<AgentHandoffEntry, BuildError> {
        Ok(AgentHandoffEntry {
            object: self.object,
            created_at: self.created_at,
            completed_at: self.completed_at,
            id: self.id,
            previous_agent_id: self
                .previous_agent_id
                .ok_or_else(|| BuildError::missing_field("previous_agent_id"))?,
            previous_agent_name: self
                .previous_agent_name
                .ok_or_else(|| BuildError::missing_field("previous_agent_name"))?,
            next_agent_id: self
                .next_agent_id
                .ok_or_else(|| BuildError::missing_field("next_agent_id"))?,
            next_agent_name: self
                .next_agent_name
                .ok_or_else(|| BuildError::missing_field("next_agent_name"))?,
        })
    }
}
