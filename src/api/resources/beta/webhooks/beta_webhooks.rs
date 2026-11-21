use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct WebhooksClient {
    pub http_client: HttpClient,
}

impl WebhooksClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// List Webhooks
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
    ///         .webhooks
    ///         .list_webhooks_v1webhooks_get(None)
    ///         .await;
    /// }
    /// ```
    pub async fn list_webhooks_v1webhooks_get(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<Vec<Webhook>, ApiError> {
        self.http_client
            .execute_request(Method::GET, "v1/webhooks", None, None, options)
            .await
    }

    /// Create Webhook
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
    ///         .webhooks
    ///         .create_webhook_v1webhooks_post(
    ///             &CreateWebhook {
    ///                 name: "name".to_string(),
    ///                 provider: WebhookProvider::Slack,
    ///                 encryption_key: None,
    ///                 event_id_pattern: None,
    ///                 event_type_pattern: None,
    ///                 expires_at: None,
    ///                 owner_type: None,
    ///                 secret: None,
    ///                 signature_encoding: None,
    ///                 signature_extractor_regex: None,
    ///                 signature_hash_algorithm: None,
    ///                 signature_header_name: None,
    ///                 signature_payload_pattern: None,
    ///                 success_status_code: None,
    ///                 timestamp_extractor_regex: None,
    ///                 url_verification_token: None,
    ///                 validity_period_second: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_webhook_v1webhooks_post(
        &self,
        request: &CreateWebhook,
        options: Option<RequestOptions>,
    ) -> Result<Webhook, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/webhooks",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get Webhook
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
    ///         .webhooks
    ///         .get_webhook_v1webhooks_webhook_id_get(&"webhook_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_webhook_v1webhooks_webhook_id_get(
        &self,
        webhook_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Webhook, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/webhooks/{}", webhook_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Delete Webhook
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
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
    ///         .webhooks
    ///         .delete_webhook_v1webhooks_webhook_id_delete(&"webhook_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_webhook_v1webhooks_webhook_id_delete(
        &self,
        webhook_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/webhooks/{}", webhook_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Get Webhook Signing Secret
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
    ///         .webhooks
    ///         .get_webhook_signing_secret_v1webhooks_webhook_id_signing_secret_get(
    ///             &"webhook_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_webhook_signing_secret_v1webhooks_webhook_id_signing_secret_get(
        &self,
        webhook_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<WebhookSigningSecret, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/webhooks/{}/signing-secret", webhook_id),
                None,
                None,
                options,
            )
            .await
    }
}
