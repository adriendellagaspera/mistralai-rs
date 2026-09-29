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
    ///         .agents
    ///         .agents_completion_v1agents_completions_post(
    ///             &AgentsCompletionRequest {
    ///                 messages: vec![AgentsCompletionRequestMessagesItem::User {
    ///                     data: UserMessage {
    ///                         ..Default::default()
    ///                     },
    ///                 }],
    ///                 agent_id: "agent_id".to_string(),
    ///                 max_tokens: None,
    ///                 stream: None,
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
