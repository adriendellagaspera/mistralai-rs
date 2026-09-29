pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VibeWorkByAgentStat {
    #[serde(default)]
    pub messages_count: i64,
    #[serde(default)]
    pub files_count: i64,
    #[serde(default)]
    pub images_count: i64,
    #[serde(default)]
    pub spreadsheets_count: i64,
    #[serde(default)]
    pub unique_conversations_count: i64,
    #[serde(default)]
    pub agent_id: String,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub last_message_at: DateTime<FixedOffset>,
    #[serde(default)]
    pub unique_users_count: i64,
}

impl VibeWorkByAgentStat {
    pub fn builder() -> VibeWorkByAgentStatBuilder {
        <VibeWorkByAgentStatBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VibeWorkByAgentStatBuilder {
    messages_count: Option<i64>,
    files_count: Option<i64>,
    images_count: Option<i64>,
    spreadsheets_count: Option<i64>,
    unique_conversations_count: Option<i64>,
    agent_id: Option<String>,
    last_message_at: Option<DateTime<FixedOffset>>,
    unique_users_count: Option<i64>,
}

impl VibeWorkByAgentStatBuilder {
    pub fn messages_count(mut self, value: i64) -> Self {
        self.messages_count = Some(value);
        self
    }

    pub fn files_count(mut self, value: i64) -> Self {
        self.files_count = Some(value);
        self
    }

    pub fn images_count(mut self, value: i64) -> Self {
        self.images_count = Some(value);
        self
    }

    pub fn spreadsheets_count(mut self, value: i64) -> Self {
        self.spreadsheets_count = Some(value);
        self
    }

    pub fn unique_conversations_count(mut self, value: i64) -> Self {
        self.unique_conversations_count = Some(value);
        self
    }

    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn last_message_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_message_at = Some(value);
        self
    }

    pub fn unique_users_count(mut self, value: i64) -> Self {
        self.unique_users_count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VibeWorkByAgentStat`].
    /// This method will fail if any of the following fields are not set:
    /// - [`messages_count`](VibeWorkByAgentStatBuilder::messages_count)
    /// - [`files_count`](VibeWorkByAgentStatBuilder::files_count)
    /// - [`images_count`](VibeWorkByAgentStatBuilder::images_count)
    /// - [`spreadsheets_count`](VibeWorkByAgentStatBuilder::spreadsheets_count)
    /// - [`unique_conversations_count`](VibeWorkByAgentStatBuilder::unique_conversations_count)
    /// - [`agent_id`](VibeWorkByAgentStatBuilder::agent_id)
    /// - [`last_message_at`](VibeWorkByAgentStatBuilder::last_message_at)
    /// - [`unique_users_count`](VibeWorkByAgentStatBuilder::unique_users_count)
    pub fn build(self) -> Result<VibeWorkByAgentStat, BuildError> {
        Ok(VibeWorkByAgentStat {
            messages_count: self
                .messages_count
                .ok_or_else(|| BuildError::missing_field("messages_count"))?,
            files_count: self
                .files_count
                .ok_or_else(|| BuildError::missing_field("files_count"))?,
            images_count: self
                .images_count
                .ok_or_else(|| BuildError::missing_field("images_count"))?,
            spreadsheets_count: self
                .spreadsheets_count
                .ok_or_else(|| BuildError::missing_field("spreadsheets_count"))?,
            unique_conversations_count: self
                .unique_conversations_count
                .ok_or_else(|| BuildError::missing_field("unique_conversations_count"))?,
            agent_id: self
                .agent_id
                .ok_or_else(|| BuildError::missing_field("agent_id"))?,
            last_message_at: self
                .last_message_at
                .ok_or_else(|| BuildError::missing_field("last_message_at"))?,
            unique_users_count: self
                .unique_users_count
                .ok_or_else(|| BuildError::missing_field("unique_users_count"))?,
        })
    }
}
