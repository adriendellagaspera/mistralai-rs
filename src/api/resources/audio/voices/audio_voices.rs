use crate::api::*;
use crate::{ApiError, ByteStream, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct VoicesClient {
    pub http_client: HttpClient,
}

impl VoicesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// List all voices (excluding sample data)
    ///
    /// # Arguments
    ///
    /// * `limit` - Maximum number of voices to return
    /// * `offset` - Offset for pagination
    /// * `type_` - Filter the voices between customs and presets
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
    ///         .voices
    ///         .list(
    ///             &AudioVoicesListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &AudioVoicesListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<VoiceListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/audio/voices",
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .serialize("type", request.r#type.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a new voice with a base64-encoded audio sample
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
    ///         .voices
    ///         .create(
    ///             &VoiceCreateRequest {
    ///                 name: "name".to_string(),
    ///                 sample_audio: "sample_audio".to_string(),
    ///                 slug: None,
    ///                 languages: None,
    ///                 gender: None,
    ///                 age: None,
    ///                 tags: None,
    ///                 color: None,
    ///                 description: None,
    ///                 retention_notice: None,
    ///                 sample_filename: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &VoiceCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<VoiceResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/audio/voices",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get voice details (excluding sample)
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
    ///     client.audio.voices.get(&"voice_id".to_string(), None).await;
    /// }
    /// ```
    pub async fn get(
        &self,
        voice_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<VoiceResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/audio/voices/{}", voice_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Delete a custom voice
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
    ///         .voices
    ///         .delete(&"voice_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        voice_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<VoiceResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/audio/voices/{}", voice_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update voice metadata (name, gender, languages, age, tags).
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
    ///         .voices
    ///         .update(
    ///             &"voice_id".to_string(),
    ///             &VoiceUpdateRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        voice_id: &str,
        request: &VoiceUpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<VoiceResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/audio/voices/{}", voice_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get the audio sample for a voice
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Streaming file download (use .into_bytes() to collect or stream chunks)
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
    ///         .voices
    ///         .get_sample(&"voice_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_sample(
        &self,
        voice_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ByteStream, ApiError> {
        self.http_client
            .execute_stream_request(
                Method::GET,
                &format!("v1/audio/voices/{}/sample", voice_id),
                None,
                None,
                options,
            )
            .await
    }
}
