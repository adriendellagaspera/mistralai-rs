use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct AgentsClient {
    pub http_client: HttpClient,
}

impl AgentsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Agents Completion
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
    ///         .agents
    ///         .agents_completion_v1agents_completions_post(
    ///             &AgentsCompletionRequest {
    ///                 agent_id: "agent_id".to_string(),
    ///                 messages: vec![AgentsCompletionRequestMessagesItem::User {
    ///                     data: UserMessage {
    ///                         ..Default::default()
    ///                     },
    ///                 }],
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
    ///                 service_tier: None,
    ///                 stop: None,
    ///                 stream: None,
    ///                 tool_choice: None,
    ///                 tools: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn agents_completion_v1agents_completions_post(
        &self,
        request: &AgentsCompletionRequest,
        options: Option<RequestOptions>,
    ) -> Result<ChatCompletionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/agents/completions",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
