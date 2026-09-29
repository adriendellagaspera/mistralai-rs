use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions, SseStream};
use reqwest::Method;

pub struct FimClient {
    pub http_client: HttpClient,
}

impl FimClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// FIM completion.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Server-Sent Events stream (use futures::StreamExt to iterate)
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
    ///         .fim
    ///         .complete_stream(
    ///             &CompleteFimStreamRequest {
    ///                 model: "codestral-latest".to_string(),
    ///                 stream: true,
    ///                 prompt: "def".to_string(),
    ///                 temperature: None,
    ///                 top_p: None,
    ///                 max_tokens: None,
    ///                 stop: None,
    ///                 random_seed: None,
    ///                 metadata: None,
    ///                 suffix: None,
    ///                 min_tokens: None,
    ///                 prompt_cache_key: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn complete_stream(
        &self,
        request: &CompleteFimStreamRequest,
        options: Option<RequestOptions>,
    ) -> Result<SseStream<CompletionChunk>, ApiError> {
        self.http_client
            .execute_sse_request(
                Method::POST,
                "v1/fim/completions",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
                None,
            )
            .await
    }

    /// FIM completion.
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
    ///         .fim
    ///         .complete(
    ///             &CompleteFimRequest {
    ///                 model: "codestral-latest".to_string(),
    ///                 stream: false,
    ///                 prompt: "def".to_string(),
    ///                 temperature: None,
    ///                 top_p: None,
    ///                 max_tokens: None,
    ///                 stop: None,
    ///                 random_seed: None,
    ///                 metadata: None,
    ///                 suffix: None,
    ///                 min_tokens: None,
    ///                 prompt_cache_key: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn complete(
        &self,
        request: &CompleteFimRequest,
        options: Option<RequestOptions>,
    ) -> Result<FimCompletionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/fim/completions",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
