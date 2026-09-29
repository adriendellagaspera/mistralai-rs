use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct ClassifiersClient {
    pub http_client: HttpClient,
}

impl ClassifiersClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Moderations
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
    ///         .classifiers
    ///         .moderations_v1moderations_post(
    ///             &ClassificationRequest {
    ///                 model: "mistral-moderation-latest".to_string(),
    ///                 metadata: None,
    ///                 input: ClassificationRequestInput::String("input".to_string()),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn moderations_v1moderations_post(
        &self,
        request: &ClassificationRequest,
        options: Option<RequestOptions>,
    ) -> Result<ModerationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/moderations",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Chat Moderations
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
    ///         .classifiers
    ///         .moderate(
    ///             &ChatModerationRequest {
    ///                 input: ChatModerationRequestInput::ChatModerationRequestInputZeroItemList(vec![
    ///                     ChatModerationRequestInputZeroItem::Assistant {
    ///                         data: AssistantMessage {
    ///                             ..Default::default()
    ///                         },
    ///                     },
    ///                 ]),
    ///                 model: "model".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn moderate(
        &self,
        request: &ChatModerationRequest,
        options: Option<RequestOptions>,
    ) -> Result<ModerationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/chat/moderations",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Classifications
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
    ///         .classifiers
    ///         .classifications_v1classifications_post(
    ///             &ClassificationRequest {
    ///                 model: "mistral-moderation-latest".to_string(),
    ///                 metadata: None,
    ///                 input: ClassificationRequestInput::String("input".to_string()),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn classifications_v1classifications_post(
        &self,
        request: &ClassificationRequest,
        options: Option<RequestOptions>,
    ) -> Result<ClassificationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/classifications",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Chat Classifications
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
    ///         .classifiers
    ///         .classify(
    ///             &ChatClassificationRequest {
    ///                 model: "model".to_string(),
    ///                 input: ChatClassificationRequestInputs::InstructRequest(InstructRequest {
    ///                     messages: vec![InstructRequestMessagesItem::Assistant {
    ///                         data: AssistantMessage {
    ///                             ..Default::default()
    ///                         },
    ///                     }],
    ///                     ..Default::default()
    ///                 }),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn classify(
        &self,
        request: &ChatClassificationRequest,
        options: Option<RequestOptions>,
    ) -> Result<ClassificationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/chat/classifications",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
