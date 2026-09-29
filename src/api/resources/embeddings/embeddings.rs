use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct EmbeddingsClient {
    pub http_client: HttpClient,
}

impl EmbeddingsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Embeddings
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
    ///         .embeddings
    ///         .create(
    ///             &EmbeddingRequest {
    ///                 input: EmbeddingRequestInput::String("input".to_string()),
    ///                 model: "mistral-embed".to_string(),
    ///                 encoding_format: None,
    ///                 metadata: None,
    ///                 output_dimension: None,
    ///                 output_dtype: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &EmbeddingRequest,
        options: Option<RequestOptions>,
    ) -> Result<EmbeddingResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/embeddings",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
