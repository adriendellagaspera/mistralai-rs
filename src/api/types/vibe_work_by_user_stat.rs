pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VibeWorkByUserStat {
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
    pub user_id: String,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub last_message_at: DateTime<FixedOffset>,
    #[serde(default)]
    pub unique_agents_count: i64,
    #[serde(default)]
    pub messages_to_agents_count: i64,
}

impl VibeWorkByUserStat {
    pub fn builder() -> VibeWorkByUserStatBuilder {
        <VibeWorkByUserStatBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VibeWorkByUserStatBuilder {
    messages_count: Option<i64>,
    files_count: Option<i64>,
    images_count: Option<i64>,
    spreadsheets_count: Option<i64>,
    unique_conversations_count: Option<i64>,
    user_id: Option<String>,
    last_message_at: Option<DateTime<FixedOffset>>,
    unique_agents_count: Option<i64>,
    messages_to_agents_count: Option<i64>,
}

impl VibeWorkByUserStatBuilder {
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

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn last_message_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_message_at = Some(value);
        self
    }

    pub fn unique_agents_count(mut self, value: i64) -> Self {
        self.unique_agents_count = Some(value);
        self
    }

    pub fn messages_to_agents_count(mut self, value: i64) -> Self {
        self.messages_to_agents_count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VibeWorkByUserStat`].
    /// This method will fail if any of the following fields are not set:
    /// - [`messages_count`](VibeWorkByUserStatBuilder::messages_count)
    /// - [`files_count`](VibeWorkByUserStatBuilder::files_count)
    /// - [`images_count`](VibeWorkByUserStatBuilder::images_count)
    /// - [`spreadsheets_count`](VibeWorkByUserStatBuilder::spreadsheets_count)
    /// - [`unique_conversations_count`](VibeWorkByUserStatBuilder::unique_conversations_count)
    /// - [`user_id`](VibeWorkByUserStatBuilder::user_id)
    /// - [`last_message_at`](VibeWorkByUserStatBuilder::last_message_at)
    /// - [`unique_agents_count`](VibeWorkByUserStatBuilder::unique_agents_count)
    /// - [`messages_to_agents_count`](VibeWorkByUserStatBuilder::messages_to_agents_count)
    pub fn build(self) -> Result<VibeWorkByUserStat, BuildError> {
        Ok(VibeWorkByUserStat {
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
            user_id: self
                .user_id
                .ok_or_else(|| BuildError::missing_field("user_id"))?,
            last_message_at: self
                .last_message_at
                .ok_or_else(|| BuildError::missing_field("last_message_at"))?,
            unique_agents_count: self
                .unique_agents_count
                .ok_or_else(|| BuildError::missing_field("unique_agents_count"))?,
            messages_to_agents_count: self
                .messages_to_agents_count
                .ok_or_else(|| BuildError::missing_field("messages_to_agents_count"))?,
        })
    }
}
