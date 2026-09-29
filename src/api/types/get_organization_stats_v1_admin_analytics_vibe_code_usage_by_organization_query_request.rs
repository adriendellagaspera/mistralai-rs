pub use crate::prelude::*;

/// Query parameters for get_organization_stats_v1_admin_analytics_vibe_code_usage_by_organization
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetOrganizationStatsV1AdminAnalyticsVibeCodeUsageByOrganizationQueryRequest {
    /// Start of the queried window, as a Unix timestamp in seconds.
    #[serde(default)]
    pub start_time: i64,
    /// End of the queried window, as a Unix timestamp in seconds.
    #[serde(default)]
    pub end_time: i64,
}

impl GetOrganizationStatsV1AdminAnalyticsVibeCodeUsageByOrganizationQueryRequest {
    pub fn builder(
    ) -> GetOrganizationStatsV1AdminAnalyticsVibeCodeUsageByOrganizationQueryRequestBuilder {
        <GetOrganizationStatsV1AdminAnalyticsVibeCodeUsageByOrganizationQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetOrganizationStatsV1AdminAnalyticsVibeCodeUsageByOrganizationQueryRequestBuilder {
    start_time: Option<i64>,
    end_time: Option<i64>,
}

impl GetOrganizationStatsV1AdminAnalyticsVibeCodeUsageByOrganizationQueryRequestBuilder {
    pub fn start_time(mut self, value: i64) -> Self {
        self.start_time = Some(value);
        self
    }

    pub fn end_time(mut self, value: i64) -> Self {
        self.end_time = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetOrganizationStatsV1AdminAnalyticsVibeCodeUsageByOrganizationQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`start_time`](GetOrganizationStatsV1AdminAnalyticsVibeCodeUsageByOrganizationQueryRequestBuilder::start_time)
    /// - [`end_time`](GetOrganizationStatsV1AdminAnalyticsVibeCodeUsageByOrganizationQueryRequestBuilder::end_time)
    pub fn build(
        self,
    ) -> Result<
        GetOrganizationStatsV1AdminAnalyticsVibeCodeUsageByOrganizationQueryRequest,
        BuildError,
    > {
        Ok(
            GetOrganizationStatsV1AdminAnalyticsVibeCodeUsageByOrganizationQueryRequest {
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
