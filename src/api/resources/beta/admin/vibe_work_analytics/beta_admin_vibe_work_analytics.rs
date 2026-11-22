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
    /// use mistralai_sdk::prelude::*;
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
    ///         .agent_stats(
    ///             &AgentStatsQueryRequest {
    ///                 start_time: 1764547200,
    ///                 end_time: 1767225600,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn agent_stats(
        &self,
        request: &AgentStatsQueryRequest,
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
    /// use mistralai_sdk::prelude::*;
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
    ///         .time_stats(
    ///             &TimeStatsQueryRequest {
    ///                 start_time: 1764547200,
    ///                 end_time: 1767225600,
    ///                 granularity: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn time_stats(
        &self,
        request: &TimeStatsQueryRequest,
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
    /// use mistralai_sdk::prelude::*;
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
    ///         .user_stats(
    ///             &UserStatsQueryRequest {
    ///                 start_time: 1764547200,
    ///                 end_time: 1767225600,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn user_stats(
        &self,
        request: &UserStatsQueryRequest,
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
