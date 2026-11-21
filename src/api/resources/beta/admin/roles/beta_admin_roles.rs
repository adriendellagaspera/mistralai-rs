use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct RolesClient {
    pub http_client: HttpClient,
}

impl RolesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// List Organization and Workspace roles.
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
    ///     client.beta.admin.roles.list(None).await;
    /// }
    /// ```
    pub async fn list(&self, options: Option<RequestOptions>) -> Result<RolesOut, ApiError> {
        self.http_client
            .execute_request(Method::GET, "v1/admin/roles", None, None, options)
            .await
    }
}
