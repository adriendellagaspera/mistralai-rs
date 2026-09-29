use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions, SseStream};
use reqwest::Method;

pub struct ConversationsClient {
    pub http_client: HttpClient,
}

impl ConversationsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Retrieve a list of conversation entities sorted by creation time.
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
    ///         .beta
    ///         .conversations
    ///         .agents_api_v1conversations_list(
    ///             &AgentsAPIV1ConversationsListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn agents_api_v1conversations_list(
        &self,
        request: &AgentsApiV1ConversationsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Vec<AgentsApiV1ConversationsListConversationsResponseItem>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/conversations",
                None,
                QueryBuilder::new()
                    .int("page", request.page.clone())
                    .int("page_size", request.page_size.clone())
                    .string("metadata", request.metadata.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a new conversation, using a base model or an agent and append entries. Completion and tool executions are run and the response is appended to the conversation.Use the returned conversation_id to continue the conversation.
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
    ///         .beta
    ///         .conversations
    ///         .agents_api_v1conversations_start(
    ///             &ConversationRequest {
    ///                 inputs: ConversationInputs::String("inputs".to_string()),
    ///                 stream: None,
    ///                 store: None,
    ///                 handoff_execution: None,
    ///                 instructions: None,
    ///                 tools: None,
    ///                 completion_args: None,
    ///                 guardrails: None,
    ///                 name: None,
    ///                 description: None,
    ///                 metadata: None,
    ///                 agent_id: None,
    ///                 agent_version: None,
    ///                 model: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn agents_api_v1conversations_start(
        &self,
        request: &ConversationRequest,
        options: Option<RequestOptions>,
    ) -> Result<ConversationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/conversations",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Given a conversation_id retrieve a conversation entity with its attributes.
    ///
    /// # Arguments
    ///
    /// * `conversation_id` - ID of the conversation from which we are fetching metadata.
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
    ///         .beta
    ///         .conversations
    ///         .agents_api_v1conversations_get(&"conversation_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn agents_api_v1conversations_get(
        &self,
        conversation_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<AgentsApiV1ConversationsGetConversationsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/conversations/{}", conversation_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Run completion on the history of the conversation and the user entries. Return the new created entries.
    ///
    /// # Arguments
    ///
    /// * `conversation_id` - ID of the conversation to which we append entries.
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
    ///         .beta
    ///         .conversations
    ///         .agents_api_v1conversations_append(
    ///             &"conversation_id".to_string(),
    ///             &ConversationAppendRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn agents_api_v1conversations_append(
        &self,
        conversation_id: &str,
        request: &ConversationAppendRequest,
        options: Option<RequestOptions>,
    ) -> Result<ConversationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/conversations/{}", conversation_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Delete a conversation given a conversation_id.
    ///
    /// # Arguments
    ///
    /// * `conversation_id` - ID of the conversation from which we are fetching metadata.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
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
    ///         .beta
    ///         .conversations
    ///         .agents_api_v1conversations_delete(&"conversation_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn agents_api_v1conversations_delete(
        &self,
        conversation_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/conversations/{}", conversation_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Given a conversation_id retrieve all the entries belonging to that conversation. The entries are sorted in the order they were appended, those can be messages, connectors or function_call.
    ///
    /// # Arguments
    ///
    /// * `conversation_id` - ID of the conversation from which we are fetching entries.
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
    ///         .beta
    ///         .conversations
    ///         .agents_api_v1conversations_history(&"conversation_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn agents_api_v1conversations_history(
        &self,
        conversation_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ConversationHistory, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/conversations/{}/history", conversation_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Given a conversation_id retrieve all the messages belonging to that conversation. This is similar to retrieving all entries except we filter the messages only.
    ///
    /// # Arguments
    ///
    /// * `conversation_id` - ID of the conversation from which we are fetching messages.
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
    ///         .beta
    ///         .conversations
    ///         .agents_api_v1conversations_messages(&"conversation_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn agents_api_v1conversations_messages(
        &self,
        conversation_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ConversationMessages, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/conversations/{}/messages", conversation_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Given a conversation_id and an id, recreate a conversation from this point and run completion. A new conversation is returned with the new entries returned.
    ///
    /// # Arguments
    ///
    /// * `conversation_id` - ID of the original conversation which is being restarted.
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
    ///         .beta
    ///         .conversations
    ///         .agents_api_v1conversations_restart(
    ///             &"conversation_id".to_string(),
    ///             &ConversationRestartRequest {
    ///                 from_entry_id: "from_entry_id".to_string(),
    ///                 inputs: None,
    ///                 stream: None,
    ///                 store: None,
    ///                 handoff_execution: None,
    ///                 completion_args: None,
    ///                 guardrails: None,
    ///                 metadata: None,
    ///                 agent_version: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn agents_api_v1conversations_restart(
        &self,
        conversation_id: &str,
        request: &ConversationRestartRequest,
        options: Option<RequestOptions>,
    ) -> Result<ConversationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/conversations/{}/restart", conversation_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Create a new conversation, using a base model or an agent and append entries. Completion and tool executions are run and the response is appended to the conversation.Use the returned conversation_id to continue the conversation.
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
    ///         .beta
    ///         .conversations
    ///         .agents_api_v1conversations_start_stream(
    ///             &ConversationStreamRequest {
    ///                 inputs: ConversationInputs::String("inputs".to_string()),
    ///                 stream: None,
    ///                 store: None,
    ///                 handoff_execution: None,
    ///                 instructions: None,
    ///                 tools: None,
    ///                 completion_args: None,
    ///                 guardrails: None,
    ///                 name: None,
    ///                 description: None,
    ///                 metadata: None,
    ///                 agent_id: None,
    ///                 agent_version: None,
    ///                 model: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn agents_api_v1conversations_start_stream(
        &self,
        request: &ConversationStreamRequest,
        options: Option<RequestOptions>,
    ) -> Result<SseStream<ConversationEvents>, ApiError> {
        self.http_client
            .execute_sse_request(
                Method::POST,
                "v1/conversations#stream",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
                None,
            )
            .await
    }

    /// Run completion on the history of the conversation and the user entries. Return the new created entries.
    ///
    /// # Arguments
    ///
    /// * `conversation_id` - ID of the conversation to which we append entries.
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
    ///         .beta
    ///         .conversations
    ///         .agents_api_v1conversations_append_stream(
    ///             &"conversation_id".to_string(),
    ///             &ConversationAppendStreamRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn agents_api_v1conversations_append_stream(
        &self,
        conversation_id: &str,
        request: &ConversationAppendStreamRequest,
        options: Option<RequestOptions>,
    ) -> Result<SseStream<ConversationEvents>, ApiError> {
        self.http_client
            .execute_sse_request(
                Method::POST,
                &format!("v1/conversations/{}#stream", conversation_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
                None,
            )
            .await
    }

    /// Given a conversation_id and an id, recreate a conversation from this point and run completion. A new conversation is returned with the new entries returned.
    ///
    /// # Arguments
    ///
    /// * `conversation_id` - ID of the original conversation which is being restarted.
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
    ///         .beta
    ///         .conversations
    ///         .agents_api_v1conversations_restart_stream(
    ///             &"conversation_id".to_string(),
    ///             &ConversationRestartStreamRequest {
    ///                 from_entry_id: "from_entry_id".to_string(),
    ///                 inputs: None,
    ///                 stream: None,
    ///                 store: None,
    ///                 handoff_execution: None,
    ///                 completion_args: None,
    ///                 guardrails: None,
    ///                 metadata: None,
    ///                 agent_version: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn agents_api_v1conversations_restart_stream(
        &self,
        conversation_id: &str,
        request: &ConversationRestartStreamRequest,
        options: Option<RequestOptions>,
    ) -> Result<SseStream<ConversationEvents>, ApiError> {
        self.http_client
            .execute_sse_request(
                Method::POST,
                &format!("v1/conversations/{}/restart#stream", conversation_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
                None,
            )
            .await
    }
}
