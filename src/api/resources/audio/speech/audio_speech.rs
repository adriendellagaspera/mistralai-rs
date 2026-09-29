use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct SpeechClient {
    pub http_client: HttpClient,
}

impl SpeechClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Speech
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
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .audio
    ///         .speech
    ///         .complete(
    ///             &SpeechRequest {
    ///                 input: "input".to_string(),
    ///                 metadata: None,
    ///                 model: None,
    ///                 prompt_cache_key: None,
    ///                 ref_audio: None,
    ///                 response_format: None,
    ///                 stream: None,
    ///                 voice_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn complete(
        &self,
        request: &SpeechRequest,
        options: Option<RequestOptions>,
    ) -> Result<CompleteSpeechResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/audio/speech",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
