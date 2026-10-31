use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct ResponsesClient {
    pub http_client: HttpClient,
}

impl ResponsesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Create a response.
    ///
    /// Generate a model response from text or image inputs. The response can
    /// include text, reasoning, and function calls. Set stream to true to
    /// receive Server-Sent Events.
    ///
    /// For multi-turn conversations, send the full history in input, including
    /// prior output items and function results. Responses are not stored. Only
    /// client-executed function tools are supported.
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
    ///         .responses
    ///         .create_response_v1responses_post(
    ///             &CreateResponseRequest {
    ///                 input: CreateResponseRequestInput::String("input".to_string()),
    ///                 model: "model".to_string(),
    ///                 background: None,
    ///                 conversation: None,
    ///                 instructions: None,
    ///                 max_output_tokens: None,
    ///                 max_tool_calls: None,
    ///                 metadata: None,
    ///                 moderation: None,
    ///                 parallel_tool_calls: None,
    ///                 previous_response_id: None,
    ///                 prompt: None,
    ///                 prompt_cache_key: None,
    ///                 reasoning: None,
    ///                 store: None,
    ///                 stream: None,
    ///                 temperature: None,
    ///                 text: None,
    ///                 tool_choice: None,
    ///                 tools: None,
    ///                 top_logprobs: None,
    ///                 top_p: None,
    ///                 truncation: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_response_v1responses_post(
        &self,
        request: &CreateResponseRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateResponseResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/responses",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Count response input tokens.
    ///
    /// Count the input tokens for a Responses request without generating a
    /// response. The count includes the supplied conversation, instructions,
    /// and tool definitions.
    ///
    /// Use the same input, instructions, and tools that you plan to send when
    /// creating a response. For multi-turn conversations, include prior output
    /// items and function results in input.
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
    ///         .responses
    ///         .count_input_tokens_v1responses_input_tokens_post(
    ///             &CountResponseInputTokensRequest {
    ///                 input: CountResponseInputTokensRequestInput::String("input".to_string()),
    ///                 model: "model".to_string(),
    ///                 conversation: None,
    ///                 instructions: None,
    ///                 parallel_tool_calls: None,
    ///                 previous_response_id: None,
    ///                 prompt: None,
    ///                 reasoning: None,
    ///                 text: None,
    ///                 tool_choice: None,
    ///                 tools: None,
    ///                 truncation: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn count_input_tokens_v1responses_input_tokens_post(
        &self,
        request: &CountResponseInputTokensRequest,
        options: Option<RequestOptions>,
    ) -> Result<CountResponseInputTokensResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/responses/input_tokens",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
