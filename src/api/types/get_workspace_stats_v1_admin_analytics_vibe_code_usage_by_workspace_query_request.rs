pub use crate::prelude::*;

/// Query parameters for get_workspace_stats_v1_admin_analytics_vibe_code_usage_by_workspace
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetWorkspaceStatsV1AdminAnalyticsVibeCodeUsageByWorkspaceQueryRequest {
    /// Start of the queried window, as a Unix timestamp in seconds.
    #[serde(default)]
    pub start_time: i64,
    /// End of the queried window, as a Unix timestamp in seconds.
    #[serde(default)]
    pub end_time: i64,
    /// Workspace ID to filter results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}

impl GetWorkspaceStatsV1AdminAnalyticsVibeCodeUsageByWorkspaceQueryRequest {
    pub fn builder() -> GetWorkspaceStatsV1AdminAnalyticsVibeCodeUsageByWorkspaceQueryRequestBuilder
    {
        <GetWorkspaceStatsV1AdminAnalyticsVibeCodeUsageByWorkspaceQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetWorkspaceStatsV1AdminAnalyticsVibeCodeUsageByWorkspaceQueryRequestBuilder {
    start_time: Option<i64>,
    end_time: Option<i64>,
    workspace_id: Option<String>,
}

impl GetWorkspaceStatsV1AdminAnalyticsVibeCodeUsageByWorkspaceQueryRequestBuilder {
    pub fn start_time(mut self, value: i64) -> Self {
        self.start_time = Some(value);
        self
    }

    pub fn end_time(mut self, value: i64) -> Self {
        self.end_time = Some(value);
        self
    }

    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetWorkspaceStatsV1AdminAnalyticsVibeCodeUsageByWorkspaceQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`start_time`](GetWorkspaceStatsV1AdminAnalyticsVibeCodeUsageByWorkspaceQueryRequestBuilder::start_time)
    /// - [`end_time`](GetWorkspaceStatsV1AdminAnalyticsVibeCodeUsageByWorkspaceQueryRequestBuilder::end_time)
    pub fn build(
        self,
    ) -> Result<GetWorkspaceStatsV1AdminAnalyticsVibeCodeUsageByWorkspaceQueryRequest, BuildError>
    {
        Ok(
            GetWorkspaceStatsV1AdminAnalyticsVibeCodeUsageByWorkspaceQueryRequest {
                start_time: self
                    .start_time
                    .ok_or_else(|| BuildError::missing_field("start_time"))?,
                end_time: self
                    .end_time
                    .ok_or_else(|| BuildError::missing_field("end_time"))?,
                workspace_id: self.workspace_id,
            },
        )
    }
}
