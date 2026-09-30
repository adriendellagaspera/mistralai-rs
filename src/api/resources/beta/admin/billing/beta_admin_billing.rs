use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct BillingClient {
    pub http_client: HttpClient,
}

impl BillingClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Get Rate Limits
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
    ///         .billing
    ///         .users_api_admin_rate_limits_get_rate_limits(None)
    ///         .await;
    /// }
    /// ```
    pub async fn users_api_admin_rate_limits_get_rate_limits(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<RateLimitsOut, ApiError> {
        self.http_client
            .execute_request(Method::GET, "v1/admin/rate-limit", None, None, options)
            .await
    }

    /// Get usage, rate, and job limits for the Organization.
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
    ///         .billing
    ///         .users_api_admin_spend_limits_get_spend_limits(None)
    ///         .await;
    /// }
    /// ```
    pub async fn users_api_admin_spend_limits_get_spend_limits(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<LimitsOut, ApiError> {
        self.http_client
            .execute_request(Method::GET, "v1/admin/spend-limit", None, None, options)
            .await
    }

    /// Update the Organization usage limit.
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
    ///         .billing
    ///         .users_api_admin_spend_limits_update_spend_limits(
    ///             &NewUsageLimitIn {
    ///                 amount: 1,
    ///                 no_monthly_limit: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn users_api_admin_spend_limits_update_spend_limits(
        &self,
        request: &NewUsageLimitIn,
        options: Option<RequestOptions>,
    ) -> Result<LimitsOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/admin/spend-limit",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get usage and cost data for the Organization.
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
    ///         .billing
    ///         .users_api_admin_usage_get_usage(
    ///             &UsersAPIAdminUsageGetUsageQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn users_api_admin_usage_get_usage(
        &self,
        request: &UsersApiAdminUsageGetUsageQueryRequest,
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
}
