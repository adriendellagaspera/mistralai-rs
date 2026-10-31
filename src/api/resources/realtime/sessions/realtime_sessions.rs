use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct SessionsClient {
    pub http_client: HttpClient,
}

impl SessionsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Create a client session. Requires the `create_client_session` permission.
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
    ///         .realtime
    ///         .sessions
    ///         .create_client_session_v1client_sessions_post(
    ///             &CreateClientSessionV1ClientSessionsPostSessionsRequest::Realtime {
    ///                 data: CreateRealtimeSessionRequest {
    ///                     model: "model".to_string(),
    ///                     ..Default::default()
    ///                 },
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_client_session_v1client_sessions_post(
        &self,
        request: &CreateClientSessionV1ClientSessionsPostSessionsRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateRealtimeSessionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/client/sessions",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
