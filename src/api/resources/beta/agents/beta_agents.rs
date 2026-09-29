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

    /// Retrieve a list of agent entities sorted by creation time. Deprecated: some features such as agent sharing are not supported by this endpoint. Use the cursor-paginated `GET /v1/agents/pages` instead.
    ///
    /// # Arguments
    ///
    /// * `page` - Page number (0-indexed)
    /// * `page_size` - Number of agents per page
    /// * `name` - Filter by agent name
    /// * `search` - Search agents by name or ID
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
    ///         .agents
    ///         .agents_api_v1agents_list(
    ///             &AgentsAPIV1AgentsListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn agents_api_v1agents_list(
        &self,
        request: &AgentsApiV1AgentsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Vec<Agent>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/agents",
                None,
                QueryBuilder::new()
                    .int("page", request.page.clone())
                    .int("page_size", request.page_size.clone())
                    .serialize("deployment_chat", request.deployment_chat.clone())
                    .serialize("sources", request.sources.clone())
                    .serialize("name", request.name.clone())
                    .serialize("search", request.search.clone())
                    .serialize("id", request.id.clone())
                    .string("metadata", request.metadata.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a new agent giving it instructions, tools, description. The agent is then available to be used as a regular assistant in a conversation or as part of an agent pool from which it can be used.
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
    ///         .agents
    ///         .agents_api_v1agents_create(
    ///             &CreateAgentRequest {
    ///                 model: "model".to_string(),
    ///                 name: "name".to_string(),
    ///                 instructions: None,
    ///                 tools: None,
    ///                 completion_args: None,
    ///                 guardrails: None,
    ///                 description: None,
    ///                 handoffs: None,
    ///                 metadata: None,
    ///                 version_message: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn agents_api_v1agents_create(
        &self,
        request: &CreateAgentRequest,
        options: Option<RequestOptions>,
    ) -> Result<Agent, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/agents",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieve a page of agent entities. Unlike the deprecated `GET /v1/agents`, this endpoint paginates by opaque cursor and honors per-agent sharing, returning only agents the caller is authorized to see.
    ///
    /// # Arguments
    ///
    /// * `page_size` - Number of agents per page
    /// * `name` - Filter by agent name
    /// * `search` - Search agents by name or ID
    /// * `page_token` - Opaque cursor from a previous response's next_page_token. When set, results page forward from the cursor.
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
    ///         .agents
    ///         .agents_api_v1agents_list_pages(
    ///             &AgentsAPIV1AgentsListPagesQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn agents_api_v1agents_list_pages(
        &self,
        request: &AgentsApiV1AgentsListPagesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AgentListPage, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/agents/pages",
                None,
                QueryBuilder::new()
                    .int("page_size", request.page_size.clone())
                    .serialize("deployment_chat", request.deployment_chat.clone())
                    .serialize("sources", request.sources.clone())
                    .serialize("name", request.name.clone())
                    .serialize("search", request.search.clone())
                    .serialize("id", request.id.clone())
                    .string("metadata", request.metadata.clone())
                    .serialize("page_token", request.page_token.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Given an agent, retrieve an agent entity with its attributes. The agent_version parameter can be an integer version number or a string alias.
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
    ///         .agents
    ///         .agents_api_v1agents_get(
    ///             &"agent_id".to_string(),
    ///             &AgentsAPIV1AgentsGetQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn agents_api_v1agents_get(
        &self,
        agent_id: &str,
        request: &AgentsApiV1AgentsGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Agent, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/agents/{}", agent_id),
                None,
                QueryBuilder::new()
                    .serialize("agent_version", request.agent_version.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Delete an agent entity.
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
    ///         .agents
    ///         .agents_api_v1agents_delete(&"agent_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn agents_api_v1agents_delete(
        &self,
        agent_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/agents/{}", agent_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update an agent attributes and create a new version.
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
    ///         .agents
    ///         .agents_api_v1agents_update(
    ///             &"agent_id".to_string(),
    ///             &UpdateAgentRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn agents_api_v1agents_update(
        &self,
        agent_id: &str,
        request: &UpdateAgentRequest,
        options: Option<RequestOptions>,
    ) -> Result<Agent, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/agents/{}", agent_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Switch the version of an agent.
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
    ///         .agents
    ///         .agents_api_v1agents_update_version(
    ///             &"agent_id".to_string(),
    ///             &AgentsAPIV1AgentsUpdateVersionQueryRequest { version: 1 },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn agents_api_v1agents_update_version(
        &self,
        agent_id: &str,
        request: &AgentsApiV1AgentsUpdateVersionQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Agent, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/agents/{}/version", agent_id),
                None,
                QueryBuilder::new()
                    .int("version", request.version.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieve all versions for a specific agent with full agent context. Supports pagination.
    ///
    /// # Arguments
    ///
    /// * `page` - Page number (0-indexed)
    /// * `page_size` - Number of versions per page
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
    ///         .agents
    ///         .agents_api_v1agents_list_versions(
    ///             &"agent_id".to_string(),
    ///             &AgentsAPIV1AgentsListVersionsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn agents_api_v1agents_list_versions(
        &self,
        agent_id: &str,
        request: &AgentsApiV1AgentsListVersionsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Vec<Agent>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/agents/{}/versions", agent_id),
                None,
                QueryBuilder::new()
                    .int("page", request.page.clone())
                    .int("page_size", request.page_size.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get a specific agent version by version number.
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
    ///         .agents
    ///         .agents_api_v1agents_get_version(&"agent_id".to_string(), &"version".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn agents_api_v1agents_get_version(
        &self,
        agent_id: &str,
        version: &str,
        options: Option<RequestOptions>,
    ) -> Result<Agent, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/agents/{}/versions/{}", agent_id, version),
                None,
                None,
                options,
            )
            .await
    }

    /// Retrieve all version aliases for a specific agent.
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
    ///         .agents
    ///         .agents_api_v1agents_list_version_aliases(&"agent_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn agents_api_v1agents_list_version_aliases(
        &self,
        agent_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Vec<AgentAliasResponse>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/agents/{}/aliases", agent_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Create a new alias or update an existing alias to point to a specific version. Aliases are unique per agent and can be reassigned to different versions.
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
    ///         .agents
    ///         .agents_api_v1agents_create_or_update_alias(
    ///             &"agent_id".to_string(),
    ///             &AgentsAPIV1AgentsCreateOrUpdateAliasQueryRequest {
    ///                 alias: "alias".to_string(),
    ///                 version: 1,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn agents_api_v1agents_create_or_update_alias(
        &self,
        agent_id: &str,
        request: &AgentsApiV1AgentsCreateOrUpdateAliasQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AgentAliasResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("v1/agents/{}/aliases", agent_id),
                None,
                QueryBuilder::new()
                    .string("alias", request.alias.clone())
                    .int("version", request.version.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Delete an existing alias for an agent.
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
    ///         .agents
    ///         .agents_api_v1agents_delete_alias(
    ///             &"agent_id".to_string(),
    ///             &AgentsAPIV1AgentsDeleteAliasQueryRequest {
    ///                 alias: "alias".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn agents_api_v1agents_delete_alias(
        &self,
        agent_id: &str,
        request: &AgentsApiV1AgentsDeleteAliasQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/agents/{}/aliases", agent_id),
                None,
                QueryBuilder::new()
                    .string("alias", request.alias.clone())
                    .build(),
                options,
            )
            .await
    }
}
