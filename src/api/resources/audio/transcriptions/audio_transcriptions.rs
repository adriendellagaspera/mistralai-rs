use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions, SseStream};
use reqwest::Method;

pub struct TranscriptionsClient {
    pub http_client: HttpClient,
}

impl TranscriptionsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Create Transcription
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
    ///         .audio
    ///         .transcriptions
    ///         .complete(
    ///             &CompleteRequest {
    ///                 model: "model".to_string(),
    ///                 file: None,
    ///                 file_url: None,
    ///                 file_id: None,
    ///                 language: None,
    ///                 temperature: None,
    ///                 stream: None,
    ///                 diarize: None,
    ///                 context_bias: None,
    ///                 timestamp_granularities: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn complete(
        &self,
        request: &CompleteRequest,
        options: Option<RequestOptions>,
    ) -> Result<TranscriptionResponse, ApiError> {
        self.http_client
            .execute_multipart_request(
                Method::POST,
                "v1/audio/transcriptions",
                request.clone().to_multipart(),
                None,
                options,
            )
            .await
    }

    /// Create Streaming Transcription (SSE)
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
    ///         .audio
    ///         .transcriptions
    ///         .stream(
    ///             &StreamRequest {
    ///                 model: "model".to_string(),
    ///                 file: None,
    ///                 file_url: None,
    ///                 file_id: None,
    ///                 language: None,
    ///                 temperature: None,
    ///                 stream: None,
    ///                 diarize: None,
    ///                 context_bias: None,
    ///                 timestamp_granularities: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn stream(
        &self,
        request: &StreamRequest,
        options: Option<RequestOptions>,
    ) -> Result<SseStream<TranscriptionStreamEvents>, ApiError> {
        self.http_client
            .execute_multipart_sse_request::<TranscriptionStreamEvents>(
                Method::POST,
                "v1/audio/transcriptions#stream",
                request.clone().to_multipart(),
                None,
                options,
                None,
            )
            .await
    }
}
