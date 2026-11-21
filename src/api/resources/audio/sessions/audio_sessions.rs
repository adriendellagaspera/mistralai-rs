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

    /// Create a real-time audio agent session.
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
    ///         .audio
    ///         .sessions
    ///         .create_session_v1audio_sessions_post(
    ///             &AudioSessionCreateRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_session_v1audio_sessions_post(
        &self,
        request: &AudioSessionCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<AudioSessionCreateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/audio/sessions",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieve an audio agent session with the transcript of its conversation.
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
    ///         .audio
    ///         .sessions
    ///         .get_session_v1audio_sessions_session_id_get(&"session_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_session_v1audio_sessions_session_id_get(
        &self,
        session_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<AudioSessionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/audio/sessions/{}", session_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Stop an audio agent session and release its resources.
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
    ///         .audio
    ///         .sessions
    ///         .delete_session_v1audio_sessions_session_id_delete(&"session_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_session_v1audio_sessions_session_id_delete(
        &self,
        session_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/audio/sessions/{}", session_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update the instructions, tools or turn handling of a live session.
    ///
    /// The agent applies the update at its next turn boundary, so it is not yet
    /// applied when this returns.
    /// The agent's `session_updated` and `session_update_failed` webhooks report
    /// the outcome to your server.
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
    ///         .audio
    ///         .sessions
    ///         .update_session_v1audio_sessions_session_id_patch(
    ///             &"session_id".to_string(),
    ///             &AudioSessionUpdateRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_session_v1audio_sessions_session_id_patch(
        &self,
        session_id: &str,
        request: &AudioSessionUpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<AudioSessionUpdateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/audio/sessions/{}", session_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
