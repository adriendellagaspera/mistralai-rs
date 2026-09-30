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
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Mistral::new(config).expect("Failed to build client");
    ///     client
    ///         .chat
    ///         .complete_stream(
    ///             &CompleteChatStreamRequest {
    ///                 messages: vec![CompleteChatStreamRequestMessagesItem::User {
    ///                     data: UserMessage {
    ///                         ..Default::default()
    ///                     },
    ///                 }],
    ///                 model: "mistral-large-latest".to_string(),
    ///                 stream: true,
    ///                 frequency_penalty: None,
    ///                 guardrails: None,
    ///                 max_tokens: None,
    ///                 metadata: None,
    ///                 n: None,
    ///                 parallel_tool_calls: None,
    ///                 prediction: None,
    ///                 presence_penalty: None,
    ///                 prompt_cache_key: None,
    ///                 prompt_mode: None,
    ///                 random_seed: None,
    ///                 reasoning_effort: None,
    ///                 response_format: None,
    ///                 safe_prompt: None,
    ///                 service_tier: None,
    ///                 stop: None,
    ///                 temperature: None,
    ///                 tool_choice: None,
    ///                 tools: None,
    ///                 top_p: None,
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
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Mistral::new(config).expect("Failed to build client");
    ///     client
    ///         .chat
    ///         .complete(
    ///             &CompleteChatRequest {
    ///                 messages: vec![CompleteChatRequestMessagesItem::User {
    ///                     data: UserMessage {
    ///                         ..Default::default()
    ///                     },
    ///                 }],
    ///                 model: "mistral-large-latest".to_string(),
    ///                 stream: false,
    ///                 frequency_penalty: None,
    ///                 guardrails: None,
    ///                 max_tokens: None,
    ///                 metadata: None,
    ///                 n: None,
    ///                 parallel_tool_calls: None,
    ///                 prediction: None,
    ///                 presence_penalty: None,
    ///                 prompt_cache_key: None,
    ///                 prompt_mode: None,
    ///                 random_seed: None,
    ///                 reasoning_effort: None,
    ///                 response_format: None,
    ///                 safe_prompt: None,
    ///                 service_tier: None,
    ///                 stop: None,
    ///                 temperature: None,
    ///                 tool_choice: None,
    ///                 tools: None,
    ///                 top_p: None,
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
