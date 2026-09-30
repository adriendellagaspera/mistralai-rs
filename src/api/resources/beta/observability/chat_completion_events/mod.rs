use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub mod fields;
pub use fields::FieldsClient;
pub struct ChatCompletionEventsClient {
    pub http_client: HttpClient,
    pub fields: FieldsClient,
}

impl ChatCompletionEventsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            fields: FieldsClient::new(config.clone())?,
        })
    }

    /// Get Chat Completion Events
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
    ///     let client = Mistral::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .observability
    ///         .chat_completion_events
    ///         .get_chat_completion_events_v1observability_chat_completion_events_search_post(
    ///             &SearchChatCompletionEventsRequest {
    ///                 search_params: FilterPayload {
    ///                     ..Default::default()
    ///                 },
    ///                 page_size: None,
    ///                 cursor: None,
    ///                 extra_fields: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_chat_completion_events_v1observability_chat_completion_events_search_post(
        &self,
        request: &SearchChatCompletionEventsRequest,
        options: Option<RequestOptions>,
    ) -> Result<SearchChatCompletionEventsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/observability/chat-completion-events/search",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .int("page_size", request.page_size.clone())
                    .serialize("cursor", request.cursor.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Alternative to /search that returns only the IDs and that can return many IDs at once
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
    ///     let client = Mistral::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .observability
    ///         .chat_completion_events
    ///         .get_chat_completion_event_ids_v1observability_chat_completion_events_search_ids_post(
    ///             &SearchChatCompletionEventIDsRequest {
    ///                 search_params: FilterPayload {
    ///                     ..Default::default()
    ///                 },
    ///                 extra_fields: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_chat_completion_event_ids_v1observability_chat_completion_events_search_ids_post(
        &self,
        request: &SearchChatCompletionEventIdsRequest,
        options: Option<RequestOptions>,
    ) -> Result<SearchChatCompletionEventIdsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/observability/chat-completion-events/search-ids",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get Chat Completion Event
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
    ///     let client = Mistral::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .observability
    ///         .chat_completion_events
    ///         .get_chat_completion_event_v1observability_chat_completion_events_event_id_get(
    ///             &"event_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_chat_completion_event_v1observability_chat_completion_events_event_id_get(
        &self,
        event_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ChatCompletionEvent, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/observability/chat-completion-events/{}", event_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Run Judge on an event based on the given options
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
    ///     let client = Mistral::new(config).expect("Failed to build client");
    ///     client.beta.observability.chat_completion_events.judge_chat_completion_event_v1observability_chat_completion_events_event_id_live_judging_post(&"event_id".to_string(), &JudgeChatCompletionEventRequest {
    ///         judge_definition: CreateJudgeRequest {
    ///             description: "description".to_string(),
    ///             instructions: "instructions".to_string(),
    ///             model_name: "model_name".to_string(),
    ///             name: "name".to_string(),
    ///             output: CreateJudgeRequestOutput::Classification {
    ///                 data: JudgeClassificationOutput {
    ///                     options: vec![JudgeClassificationOutputOption {
    ///                         description: "description".to_string(),
    ///                         value: "value".to_string(),
    ///                         ..Default::default()
    ///                     }],
    ///                     ..Default::default()
    ///                 }
    ///             },
    ///             tools: vec!["tools".to_string()]
    ///         }
    ///     }, None).await;
    /// }
    /// ```
    pub async fn judge_chat_completion_event_v1observability_chat_completion_events_event_id_live_judging_post(
        &self,
        event_id: &str,
        request: &JudgeChatCompletionEventRequest,
        options: Option<RequestOptions>,
    ) -> Result<JudgeOutput, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/observability/chat-completion-events/{}/live-judging",
                    event_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get Similar Chat Completion Events
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
    ///     let client = Mistral::new(config).expect("Failed to build client");
    ///     client.beta.observability.chat_completion_events.get_similar_chat_completion_events_v1observability_chat_completion_events_event_id_similar_events_get(&"event_id".to_string(), None).await;
    /// }
    /// ```
    pub async fn get_similar_chat_completion_events_v1observability_chat_completion_events_event_id_similar_events_get(
        &self,
        event_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<SearchChatCompletionEventsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/observability/chat-completion-events/{}/similar-events",
                    event_id
                ),
                None,
                None,
                options,
            )
            .await
    }
}
