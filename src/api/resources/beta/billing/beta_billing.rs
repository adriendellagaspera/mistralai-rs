use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
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

    /// Return the organization's API and Vibe budgets for the current billing period.
    ///
    /// Requires the `see_org_inference_usage` permission.
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
    ///     client.beta.billing.get_budget(None).await;
    /// }
    /// ```
    pub async fn get_budget(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<GetBudgetResponse, ApiError> {
        self.http_client
            .execute_request(Method::GET, "v1/billing/budget", None, None, options)
            .await
    }

    /// Return the organization's plan and whether pay-as-you-go is enabled.
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
    ///     client.beta.billing.get_subscription(None).await;
    /// }
    /// ```
    pub async fn get_subscription(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<GetSubscriptionResponse, ApiError> {
        self.http_client
            .execute_request(Method::GET, "v1/billing/subscription", None, None, options)
            .await
    }
}
