use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct AgentsClient2 {
    pub http_client: HttpClient,
}

impl AgentsClient2 {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// List audio agents
    ///
    /// # Arguments
    ///
    /// * `page_size` - Maximum number of agents to return
    /// * `page_token` - Cursor returned as next_page_token by the previous page
    /// * `search` - Substring match on the agent name
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
    ///         .agents
    ///         .list_agents_v1audio_agents_get(
    ///             &ListAgentsV1AudioAgentsGetQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_agents_v1audio_agents_get(
        &self,
        request: &ListAgentsV1AudioAgentsGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AudioAgentListPage, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/audio/agents",
                None,
                QueryBuilder::new()
                    .int("page_size", request.page_size.clone())
                    .serialize("page_token", request.page_token.clone())
                    .serialize("search", request.search.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create an audio agent
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
    ///         .agents
    ///         .create_agent_v1audio_agents_post(
    ///             &AudioAgentCreateRequest {
    ///                 configuration: AudioAgentConfigurationRequest {
    ///                     ..Default::default()
    ///                 },
    ///                 name: "name".to_string(),
    ///                 description: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_agent_v1audio_agents_post(
        &self,
        request: &AudioAgentCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<AudioAgentResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/audio/agents",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get an audio agent
    ///
    /// # Arguments
    ///
    /// * `version` - Version to return. Defaults to the latest version.
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
    ///         .agents
    ///         .get_agent_v1audio_agents_agent_id_get(
    ///             &"agent_id".to_string(),
    ///             &GetAgentV1AudioAgentsAgentIDGetQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_agent_v1audio_agents_agent_id_get(
        &self,
        agent_id: &str,
        request: &GetAgentV1AudioAgentsAgentIdGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AudioAgentResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/audio/agents/{}", agent_id),
                None,
                QueryBuilder::new()
                    .serialize("version", request.version.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Update an audio agent
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
    ///         .agents
    ///         .update_agent_v1audio_agents_agent_id_put(
    ///             &"agent_id".to_string(),
    ///             &AudioAgentUpdateRequest {
    ///                 configuration: AudioAgentConfigurationRequest {
    ///                     ..Default::default()
    ///                 },
    ///                 name: "name".to_string(),
    ///                 description: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_agent_v1audio_agents_agent_id_put(
        &self,
        agent_id: &str,
        request: &AudioAgentUpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<AudioAgentResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("v1/audio/agents/{}", agent_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Delete an audio agent
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
    ///         .agents
    ///         .delete_agent_v1audio_agents_agent_id_delete(&"agent_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_agent_v1audio_agents_agent_id_delete(
        &self,
        agent_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/audio/agents/{}", agent_id),
                None,
                None,
                options,
            )
            .await
    }

    /// List audio agent sessions
    ///
    /// # Arguments
    ///
    /// * `page_size` - Maximum number of sessions to return
    /// * `page_token` - Cursor returned as next_page_token by the previous page
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
    ///         .agents
    ///         .list_agent_sessions_v1audio_agents_agent_id_sessions_get(
    ///             &"agent_id".to_string(),
    ///             &ListAgentSessionsV1AudioAgentsAgentIDSessionsGetQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_agent_sessions_v1audio_agents_agent_id_sessions_get(
        &self,
        agent_id: &str,
        request: &ListAgentSessionsV1AudioAgentsAgentIdSessionsGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AudioAgentSessionListPage, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/audio/agents/{}/sessions", agent_id),
                None,
                QueryBuilder::new()
                    .int("page_size", request.page_size.clone())
                    .serialize("page_token", request.page_token.clone())
                    .build(),
                options,
            )
            .await
    }

    /// List audio agent versions
    ///
    /// # Arguments
    ///
    /// * `page_size` - Maximum number of versions to return
    /// * `page_token` - Cursor returned as next_page_token by the previous page
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
    ///         .agents
    ///         .list_agent_versions_v1audio_agents_agent_id_versions_get(
    ///             &"agent_id".to_string(),
    ///             &ListAgentVersionsV1AudioAgentsAgentIDVersionsGetQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_agent_versions_v1audio_agents_agent_id_versions_get(
        &self,
        agent_id: &str,
        request: &ListAgentVersionsV1AudioAgentsAgentIdVersionsGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AudioAgentListPage, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/audio/agents/{}/versions", agent_id),
                None,
                QueryBuilder::new()
                    .int("page_size", request.page_size.clone())
                    .serialize("page_token", request.page_token.clone())
                    .build(),
                options,
            )
            .await
    }
}
