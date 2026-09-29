pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VibeWorkByTimeStat {
    #[serde(default)]
    pub files_count: i64,
    #[serde(default)]
    pub images_count: i64,
    #[serde(default)]
    pub messages_count: i64,
    #[serde(default)]
    pub messages_to_agents_count: i64,
    #[serde(default)]
    pub spreadsheets_count: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub time_bucket: Option<DateTime<FixedOffset>>,
    #[serde(default)]
    pub unique_agents_count: i64,
    #[serde(default)]
    pub unique_conversations_count: i64,
    #[serde(default)]
    pub unique_users_count: i64,
}

impl VibeWorkByTimeStat {
    pub fn builder() -> VibeWorkByTimeStatBuilder {
        <VibeWorkByTimeStatBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VibeWorkByTimeStatBuilder {
    files_count: Option<i64>,
    images_count: Option<i64>,
    messages_count: Option<i64>,
    messages_to_agents_count: Option<i64>,
    spreadsheets_count: Option<i64>,
    time_bucket: Option<DateTime<FixedOffset>>,
    unique_agents_count: Option<i64>,
    unique_conversations_count: Option<i64>,
    unique_users_count: Option<i64>,
}

impl VibeWorkByTimeStatBuilder {
    pub fn files_count(mut self, value: i64) -> Self {
        self.files_count = Some(value);
        self
    }

    pub fn images_count(mut self, value: i64) -> Self {
        self.images_count = Some(value);
        self
    }

    pub fn messages_count(mut self, value: i64) -> Self {
        self.messages_count = Some(value);
        self
    }

    pub fn messages_to_agents_count(mut self, value: i64) -> Self {
        self.messages_to_agents_count = Some(value);
        self
    }

    pub fn spreadsheets_count(mut self, value: i64) -> Self {
        self.spreadsheets_count = Some(value);
        self
    }

    pub fn time_bucket(mut self, value: DateTime<FixedOffset>) -> Self {
        self.time_bucket = Some(value);
        self
    }

    pub fn unique_agents_count(mut self, value: i64) -> Self {
        self.unique_agents_count = Some(value);
        self
    }

    pub fn unique_conversations_count(mut self, value: i64) -> Self {
        self.unique_conversations_count = Some(value);
        self
    }

    pub fn unique_users_count(mut self, value: i64) -> Self {
        self.unique_users_count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VibeWorkByTimeStat`].
    /// This method will fail if any of the following fields are not set:
    /// - [`files_count`](VibeWorkByTimeStatBuilder::files_count)
    /// - [`images_count`](VibeWorkByTimeStatBuilder::images_count)
    /// - [`messages_count`](VibeWorkByTimeStatBuilder::messages_count)
    /// - [`messages_to_agents_count`](VibeWorkByTimeStatBuilder::messages_to_agents_count)
    /// - [`spreadsheets_count`](VibeWorkByTimeStatBuilder::spreadsheets_count)
    /// - [`unique_agents_count`](VibeWorkByTimeStatBuilder::unique_agents_count)
    /// - [`unique_conversations_count`](VibeWorkByTimeStatBuilder::unique_conversations_count)
    /// - [`unique_users_count`](VibeWorkByTimeStatBuilder::unique_users_count)
    pub fn build(self) -> Result<VibeWorkByTimeStat, BuildError> {
        Ok(VibeWorkByTimeStat {
            files_count: self
                .files_count
                .ok_or_else(|| BuildError::missing_field("files_count"))?,
            images_count: self
                .images_count
                .ok_or_else(|| BuildError::missing_field("images_count"))?,
            messages_count: self
                .messages_count
                .ok_or_else(|| BuildError::missing_field("messages_count"))?,
            messages_to_agents_count: self
                .messages_to_agents_count
                .ok_or_else(|| BuildError::missing_field("messages_to_agents_count"))?,
            spreadsheets_count: self
                .spreadsheets_count
                .ok_or_else(|| BuildError::missing_field("spreadsheets_count"))?,
            time_bucket: self.time_bucket,
            unique_agents_count: self
                .unique_agents_count
                .ok_or_else(|| BuildError::missing_field("unique_agents_count"))?,
            unique_conversations_count: self
                .unique_conversations_count
                .ok_or_else(|| BuildError::missing_field("unique_conversations_count"))?,
            unique_users_count: self
                .unique_users_count
                .ok_or_else(|| BuildError::missing_field("unique_users_count"))?,
        })
    }
}
