pub use crate::prelude::*;

/// Query parameters for get_by_user_stats_v1_admin_analytics_vibe_work_usage_by_user_stats
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetByUserStatsV1AdminAnalyticsVibeWorkUsageByUserStatsQueryRequest {
    /// Start of the queried window, as a Unix timestamp in seconds.
    #[serde(default)]
    pub start_time: i64,
    /// End of the queried window, as a Unix timestamp in seconds.
    #[serde(default)]
    pub end_time: i64,
}

impl GetByUserStatsV1AdminAnalyticsVibeWorkUsageByUserStatsQueryRequest {
    pub fn builder() -> GetByUserStatsV1AdminAnalyticsVibeWorkUsageByUserStatsQueryRequestBuilder {
        <GetByUserStatsV1AdminAnalyticsVibeWorkUsageByUserStatsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetByUserStatsV1AdminAnalyticsVibeWorkUsageByUserStatsQueryRequestBuilder {
    start_time: Option<i64>,
    end_time: Option<i64>,
}

impl GetByUserStatsV1AdminAnalyticsVibeWorkUsageByUserStatsQueryRequestBuilder {
    pub fn start_time(mut self, value: i64) -> Self {
        self.start_time = Some(value);
        self
    }

    pub fn end_time(mut self, value: i64) -> Self {
        self.end_time = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetByUserStatsV1AdminAnalyticsVibeWorkUsageByUserStatsQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`start_time`](GetByUserStatsV1AdminAnalyticsVibeWorkUsageByUserStatsQueryRequestBuilder::start_time)
    /// - [`end_time`](GetByUserStatsV1AdminAnalyticsVibeWorkUsageByUserStatsQueryRequestBuilder::end_time)
    pub fn build(
        self,
    ) -> Result<GetByUserStatsV1AdminAnalyticsVibeWorkUsageByUserStatsQueryRequest, BuildError>
    {
        Ok(
            GetByUserStatsV1AdminAnalyticsVibeWorkUsageByUserStatsQueryRequest {
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
