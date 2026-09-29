pub use crate::prelude::*;

/// Query parameters for get_by_time_stats_v1_admin_analytics_vibe_work_usage_by_time_stats
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetByTimeStatsV1AdminAnalyticsVibeWorkUsageByTimeStatsQueryRequest {
    /// Start of the queried window, as a Unix timestamp in seconds.
    #[serde(default)]
    pub start_time: i64,
    /// End of the queried window, as a Unix timestamp in seconds.
    #[serde(default)]
    pub end_time: i64,
    /// Time interval used to group usage results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granularity: Option<
        GetByTimeStatsV1AdminAnalyticsVibeWorkUsageByTimeStatsVibeWorkAnalyticsRequestGranularity,
    >,
}

impl GetByTimeStatsV1AdminAnalyticsVibeWorkUsageByTimeStatsQueryRequest {
    pub fn builder() -> GetByTimeStatsV1AdminAnalyticsVibeWorkUsageByTimeStatsQueryRequestBuilder {
        <GetByTimeStatsV1AdminAnalyticsVibeWorkUsageByTimeStatsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetByTimeStatsV1AdminAnalyticsVibeWorkUsageByTimeStatsQueryRequestBuilder {
    start_time: Option<i64>,
    end_time: Option<i64>,
    granularity: Option<
        GetByTimeStatsV1AdminAnalyticsVibeWorkUsageByTimeStatsVibeWorkAnalyticsRequestGranularity,
    >,
}

impl GetByTimeStatsV1AdminAnalyticsVibeWorkUsageByTimeStatsQueryRequestBuilder {
    pub fn start_time(mut self, value: i64) -> Self {
        self.start_time = Some(value);
        self
    }

    pub fn end_time(mut self, value: i64) -> Self {
        self.end_time = Some(value);
        self
    }

    pub fn granularity(
        mut self,
        value: GetByTimeStatsV1AdminAnalyticsVibeWorkUsageByTimeStatsVibeWorkAnalyticsRequestGranularity,
    ) -> Self {
        self.granularity = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetByTimeStatsV1AdminAnalyticsVibeWorkUsageByTimeStatsQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`start_time`](GetByTimeStatsV1AdminAnalyticsVibeWorkUsageByTimeStatsQueryRequestBuilder::start_time)
    /// - [`end_time`](GetByTimeStatsV1AdminAnalyticsVibeWorkUsageByTimeStatsQueryRequestBuilder::end_time)
    pub fn build(
        self,
    ) -> Result<GetByTimeStatsV1AdminAnalyticsVibeWorkUsageByTimeStatsQueryRequest, BuildError>
    {
        Ok(
            GetByTimeStatsV1AdminAnalyticsVibeWorkUsageByTimeStatsQueryRequest {
                start_time: self
                    .start_time
                    .ok_or_else(|| BuildError::missing_field("start_time"))?,
                end_time: self
                    .end_time
                    .ok_or_else(|| BuildError::missing_field("end_time"))?,
                granularity: self.granularity,
            },
        )
    }
}
