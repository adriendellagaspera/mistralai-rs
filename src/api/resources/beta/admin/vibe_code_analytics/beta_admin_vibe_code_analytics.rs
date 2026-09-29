use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct VibeCodeAnalyticsClient {
    pub http_client: HttpClient,
}

impl VibeCodeAnalyticsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Get Vibe Code usage for a Workspace.
    ///
    /// # Arguments
    ///
    /// * `start_time` - Start of the queried window, as a Unix timestamp in seconds.
    /// * `end_time` - End of the queried window, as a Unix timestamp in seconds.
    /// * `workspace_id` - Workspace ID to filter results.
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
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .admin
    ///         .vibe_code_analytics
    ///         .get_workspace_stats_v1admin_analytics_vibe_code_usage_by_workspace(
    ///             &GetWorkspaceStatsV1AdminAnalyticsVibeCodeUsageByWorkspaceQueryRequest {
    ///                 start_time: 1764547200,
    ///                 end_time: 1767225600,
    ///                 workspace_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_workspace_stats_v1admin_analytics_vibe_code_usage_by_workspace(
        &self,
        request: &GetWorkspaceStatsV1AdminAnalyticsVibeCodeUsageByWorkspaceQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<VibeWorkspaceStatsOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/admin/analytics/vibe/code/usage/by_workspace",
                None,
                QueryBuilder::new()
                    .int("start_time", request.start_time.clone())
                    .int("end_time", request.end_time.clone())
                    .serialize("workspace_id", request.workspace_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get Vibe Code usage for the Organization.
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
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .admin
    ///         .vibe_code_analytics
    ///         .get_organization_stats_v1admin_analytics_vibe_code_usage_by_organization(
    ///             &GetOrganizationStatsV1AdminAnalyticsVibeCodeUsageByOrganizationQueryRequest {
    ///                 start_time: 1764547200,
    ///                 end_time: 1767225600,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_organization_stats_v1admin_analytics_vibe_code_usage_by_organization(
        &self,
        request: &GetOrganizationStatsV1AdminAnalyticsVibeCodeUsageByOrganizationQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<VibeOrganizationStatsOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/admin/analytics/vibe/code/usage/by_organization",
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
