use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct UsageClient {
    pub http_client: HttpClient,
}

impl UsageClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Return the organization's usage for a calendar month in a fixed, product-oriented format.
    ///
    /// The response includes the applicable price catalog and currency. For custom time ranges, filters, grouping, and
    /// pagination, use `POST /v1/admin/usage/query`.
    ///
    /// # Arguments
    ///
    /// * `month` - Month to return usage for.
    /// * `year` - Year to return usage for.
    /// * `workspace_id` - Workspace ID to filter results.
    /// * `api_zone` - Regional inference zone to filter results.
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
    ///         .usage
    ///         .get(
    ///             &BetaAdminUsageGetQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        request: &BetaAdminUsageGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<UsageOutjson, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/admin/usage",
                None,
                QueryBuilder::new()
                    .serialize("month", request.month.clone())
                    .serialize("year", request.year.clone())
                    .serialize("workspace_id", request.workspace_id.clone())
                    .serialize("api_zone", request.api_zone.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Query the organization's detailed usage for a required time range.
    ///
    /// Optional filters, caller-selected grouping and time buckets, pagination, aggregates, and distinct API-call counts
    /// are supported. The response contains usage quantities only; it does not include prices, currency, or calculated
    /// costs. For a fixed monthly report with pricing, use `GET /v1/admin/usage`.
    ///
    /// # Arguments
    ///
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
    ///         .usage
    ///         .query(
    ///             &UsageQueryIn {
    ///                 end: DateTime::parse_from_rfc3339("2026-09-01T00:00:00Z").unwrap(),
    ///                 start: DateTime::parse_from_rfc3339("2026-08-01T00:00:00Z").unwrap(),
    ///                 api_key_id: None,
    ///                 api_zone: None,
    ///                 billing_display_name: None,
    ///                 billing_group: None,
    ///                 billing_metric: None,
    ///                 filters: None,
    ///                 group_by: None,
    ///                 include_aggregates: None,
    ///                 include_nb_api_calls: None,
    ///                 limit: None,
    ///                 offset: None,
    ///                 organization_id: None,
    ///                 service_account_id: None,
    ///                 service_tier: None,
    ///                 time_bucket: None,
    ///                 untracked: None,
    ///                 usage_type: None,
    ///                 user_id: None,
    ///                 workspace_ids: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn query(
        &self,
        request: &UsageQueryIn,
        options: Option<RequestOptions>,
    ) -> Result<UsageQueryOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/admin/usage/query",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
