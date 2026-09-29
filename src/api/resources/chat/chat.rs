use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions, SseStream};
use reqwest::Method;

pub struct ChatClient {
    pub http_client: HttpClient,
}

impl ChatClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Chat Completion
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
    ///         .chat
    ///         .complete_stream(
    ///             &CompleteChatStreamRequest {
    ///                 model: "mistral-large-latest".to_string(),
    ///                 stream: true,
    ///                 messages: vec![CompleteChatStreamRequestMessagesItem::User {
    ///                     data: UserMessage {
    ///                         ..Default::default()
    ///                     },
    ///                 }],
    ///                 temperature: None,
    ///                 top_p: None,
    ///                 max_tokens: None,
    ///                 stop: None,
    ///                 random_seed: None,
    ///                 metadata: None,
    ///                 response_format: None,
    ///                 tools: None,
    ///                 tool_choice: None,
    ///                 presence_penalty: None,
    ///                 frequency_penalty: None,
    ///                 n: None,
    ///                 prediction: None,
    ///                 parallel_tool_calls: None,
    ///                 reasoning_effort: None,
    ///                 prompt_mode: None,
    ///                 guardrails: None,
    ///                 prompt_cache_key: None,
    ///                 service_tier: None,
    ///                 safe_prompt: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn complete_stream(
        &self,
        request: &CompleteChatStreamRequest,
        options: Option<RequestOptions>,
    ) -> Result<SseStream<CompletionChunk>, ApiError> {
        self.http_client
            .execute_sse_request(
                Method::POST,
                "v1/chat/completions",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
                None,
            )
            .await
    }

    /// Chat Completion
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
    ///         .chat
    ///         .complete(
    ///             &CompleteChatRequest {
    ///                 model: "mistral-large-latest".to_string(),
    ///                 stream: false,
    ///                 messages: vec![CompleteChatRequestMessagesItem::User {
    ///                     data: UserMessage {
    ///                         ..Default::default()
    ///                     },
    ///                 }],
    ///                 temperature: None,
    ///                 top_p: None,
    ///                 max_tokens: None,
    ///                 stop: None,
    ///                 random_seed: None,
    ///                 metadata: None,
    ///                 response_format: None,
    ///                 tools: None,
    ///                 tool_choice: None,
    ///                 presence_penalty: None,
    ///                 frequency_penalty: None,
    ///                 n: None,
    ///                 prediction: None,
    ///                 parallel_tool_calls: None,
    ///                 reasoning_effort: None,
    ///                 prompt_mode: None,
    ///                 guardrails: None,
    ///                 prompt_cache_key: None,
    ///                 service_tier: None,
    ///                 safe_prompt: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn complete(
        &self,
        request: &CompleteChatRequest,
        options: Option<RequestOptions>,
    ) -> Result<ChatCompletionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/chat/completions",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
