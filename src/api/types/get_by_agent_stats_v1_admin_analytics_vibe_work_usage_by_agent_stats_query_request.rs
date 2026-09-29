pub use crate::prelude::*;

/// Query parameters for get_by_agent_stats_v1_admin_analytics_vibe_work_usage_by_agent_stats
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetByAgentStatsV1AdminAnalyticsVibeWorkUsageByAgentStatsQueryRequest {
    /// Start of the queried window, as a Unix timestamp in seconds.
    #[serde(default)]
    pub start_time: i64,
    /// End of the queried window, as a Unix timestamp in seconds.
    #[serde(default)]
    pub end_time: i64,
}

impl GetByAgentStatsV1AdminAnalyticsVibeWorkUsageByAgentStatsQueryRequest {
    pub fn builder() -> GetByAgentStatsV1AdminAnalyticsVibeWorkUsageByAgentStatsQueryRequestBuilder
    {
        <GetByAgentStatsV1AdminAnalyticsVibeWorkUsageByAgentStatsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetByAgentStatsV1AdminAnalyticsVibeWorkUsageByAgentStatsQueryRequestBuilder {
    start_time: Option<i64>,
    end_time: Option<i64>,
}

impl GetByAgentStatsV1AdminAnalyticsVibeWorkUsageByAgentStatsQueryRequestBuilder {
    pub fn start_time(mut self, value: i64) -> Self {
        self.start_time = Some(value);
        self
    }

    pub fn end_time(mut self, value: i64) -> Self {
        self.end_time = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetByAgentStatsV1AdminAnalyticsVibeWorkUsageByAgentStatsQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`start_time`](GetByAgentStatsV1AdminAnalyticsVibeWorkUsageByAgentStatsQueryRequestBuilder::start_time)
    /// - [`end_time`](GetByAgentStatsV1AdminAnalyticsVibeWorkUsageByAgentStatsQueryRequestBuilder::end_time)
    pub fn build(
        self,
    ) -> Result<GetByAgentStatsV1AdminAnalyticsVibeWorkUsageByAgentStatsQueryRequest, BuildError>
    {
        Ok(
            GetByAgentStatsV1AdminAnalyticsVibeWorkUsageByAgentStatsQueryRequest {
                start_time: self
                    .start_time
                    .ok_or_else(|| BuildError::missing_field("start_time"))?,
                end_time: self
                    .end_time
                    .ok_or_else(|| BuildError::missing_field("end_time"))?,
            },
        )
    }
}
