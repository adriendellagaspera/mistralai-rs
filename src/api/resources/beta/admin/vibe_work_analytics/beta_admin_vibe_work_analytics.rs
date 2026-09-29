use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct VibeWorkAnalyticsClient {
    pub http_client: HttpClient,
}

impl VibeWorkAnalyticsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Get Vibe Work usage by agent for a time range.
    ///
    /// # Arguments
    ///
    /// * `start_time` - Start of the queried window, as a Unix timestamp in seconds.
    /// * `end_time` - End of the queried window, as a Unix timestamp in seconds.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use adriendellagaspera_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Mistral::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .admin
    ///         .vibe_work_analytics
    ///         .get_by_agent_stats_v1admin_analytics_vibe_work_usage_by_agent_stats(
    ///             &GetByAgentStatsV1AdminAnalyticsVibeWorkUsageByAgentStatsQueryRequest {
    ///                 start_time: 1764547200,
    ///                 end_time: 1767225600,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_by_agent_stats_v1admin_analytics_vibe_work_usage_by_agent_stats(
        &self,
        request: &GetByAgentStatsV1AdminAnalyticsVibeWorkUsageByAgentStatsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<VibeWorkByAgentStatsOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/admin/analytics/vibe/work/usage/by_agent_stats",
                None,
                QueryBuilder::new()
                    .int("start_time", request.start_time.clone())
                    .int("end_time", request.end_time.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get Vibe Work usage over time.
    ///
    /// # Arguments
    ///
    /// * `start_time` - Start of the queried window, as a Unix timestamp in seconds.
    /// * `end_time` - End of the queried window, as a Unix timestamp in seconds.
    /// * `granularity` - Time interval used to group usage results.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use adriendellagaspera_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Mistral::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .admin
    ///         .vibe_work_analytics
    ///         .get_by_time_stats_v1admin_analytics_vibe_work_usage_by_time_stats(
    ///             &GetByTimeStatsV1AdminAnalyticsVibeWorkUsageByTimeStatsQueryRequest {
    ///                 start_time: 1764547200,
    ///                 end_time: 1767225600,
    ///                 granularity: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_by_time_stats_v1admin_analytics_vibe_work_usage_by_time_stats(
        &self,
        request: &GetByTimeStatsV1AdminAnalyticsVibeWorkUsageByTimeStatsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<VibeWorkByTimeStatsOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/admin/analytics/vibe/work/usage/by_time_stats",
                None,
                QueryBuilder::new()
                    .int("start_time", request.start_time.clone())
                    .int("end_time", request.end_time.clone())
                    .serialize("granularity", request.granularity.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get Vibe Work usage by user for a time range.
    ///
    /// # Arguments
    ///
    /// * `start_time` - Start of the queried window, as a Unix timestamp in seconds.
    /// * `end_time` - End of the queried window, as a Unix timestamp in seconds.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use adriendellagaspera_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Mistral::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .admin
    ///         .vibe_work_analytics
    ///         .get_by_user_stats_v1admin_analytics_vibe_work_usage_by_user_stats(
    ///             &GetByUserStatsV1AdminAnalyticsVibeWorkUsageByUserStatsQueryRequest {
    ///                 start_time: 1764547200,
    ///                 end_time: 1767225600,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_by_user_stats_v1admin_analytics_vibe_work_usage_by_user_stats(
        &self,
        request: &GetByUserStatsV1AdminAnalyticsVibeWorkUsageByUserStatsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<VibeWorkByUserStatsOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/admin/analytics/vibe/work/usage/by_user_stats",
                None,
                QueryBuilder::new()
                    .int("start_time", request.start_time.clone())
                    .int("end_time", request.end_time.clone())
                    .build(),
                options,
            )
            .await
    }
}
