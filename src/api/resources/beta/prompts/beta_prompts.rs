use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct PromptsClient {
    pub http_client: HttpClient,
}

impl PromptsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// ListPrompts
    ///
    /// # Arguments
    ///
    /// * `sort_field` - Defaults to created_at when omitted.
    /// * `sort_direction_legacy` - Defaults to descending for timestamp fields and ascending for text fields.
    /// * `sort_by` - REST-friendly alias for sort.field. Supported values: created_at, last_modified_at, name, title.
    /// * `sort_direction` - REST-friendly alias for sort.direction. Supported values: asc, desc.
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
    ///         .prompts
    ///         .prompts_list(
    ///             &PromptsListQueryRequest {
    ///                 page_size: None,
    ///                 page_token: None,
    ///                 alias: None,
    ///                 fields: vec![],
    ///                 sort_field: None,
    ///                 sort_direction_legacy: None,
    ///                 sort_by: None,
    ///                 sort_direction: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn prompts_list(
        &self,
        request: &PromptsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListPromptsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v2/prompts",
                None,
                QueryBuilder::new()
                    .int("pageSize", request.page_size.clone())
                    .string("pageToken", request.page_token.clone())
                    .string("alias", request.alias.clone())
                    .string_array("fields", request.fields.clone())
                    .serialize("sort.field", request.sort_field.clone())
                    .serialize("sort.direction", request.sort_direction_legacy.clone())
                    .string("sort_by", request.sort_by.clone())
                    .string("sort_direction", request.sort_direction.clone())
                    .build(),
                options,
            )
            .await
    }

    /// CreatePrompt
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
    ///         .prompts
    ///         .prompts_create(
    ///             &CreatePromptRequest {
    ///                 definition: PromptDefinition {
    ///                     content: "content".to_string(),
    ///                     ..Default::default()
    ///                 },
    ///                 name: "name".to_string(),
    ///                 aliases: None,
    ///                 description: None,
    ///                 notes: None,
    ///                 sharing_scope: None,
    ///                 title: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn prompts_create(
        &self,
        request: &CreatePromptRequest,
        options: Option<RequestOptions>,
    ) -> Result<Prompt, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v2/prompts",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// GetPrompt
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
    ///         .prompts
    ///         .prompts_get(
    ///             &"prompt_id".to_string(),
    ///             &PromptsGetQueryRequest {
    ///                 version: Some(1),
    ///                 alias: None,
    ///                 fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn prompts_get(
        &self,
        prompt_id: &str,
        request: &PromptsGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Prompt, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v2/prompts/{}", prompt_id),
                None,
                QueryBuilder::new()
                    .int("version", request.version.clone())
                    .string("alias", request.alias.clone())
                    .string_array("fields", request.fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// DeletePrompt
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
    ///         .prompts
    ///         .prompts_delete(&"prompt_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn prompts_delete(
        &self,
        prompt_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DeletePromptResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v2/prompts/{}", prompt_id),
                None,
                None,
                options,
            )
            .await
    }

    /// UpdatePrompt
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
    ///         .prompts
    ///         .prompts_update(
    ///             &"prompt_id".to_string(),
    ///             &PromptsUpdatePromptsRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn prompts_update(
        &self,
        prompt_id: &str,
        request: &PromptsUpdatePromptsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Prompt, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v2/prompts/{}", prompt_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// ListPromptVersions
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
    ///         .prompts
    ///         .prompts_list_versions(&"prompt_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn prompts_list_versions(
        &self,
        prompt_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ListPromptVersionsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v2/prompts/{}/versions", prompt_id),
                None,
                None,
                options,
            )
            .await
    }

    /// CreatePromptVersion
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
    ///         .prompts
    ///         .prompts_create_version(
    ///             &"prompt_id".to_string(),
    ///             &PromptsCreateVersionPromptsRequest {
    ///                 definition: PromptDefinition {
    ///                     content: "content".to_string(),
    ///                     ..Default::default()
    ///                 },
    ///                 aliases: None,
    ///                 notes: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn prompts_create_version(
        &self,
        prompt_id: &str,
        request: &PromptsCreateVersionPromptsRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreatePromptVersionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v2/prompts/{}/versions", prompt_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// GetPromptVersion
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
    ///         .prompts
    ///         .prompts_get_version(
    ///             &"prompt_id".to_string(),
    ///             1,
    ///             &PromptsGetVersionQueryRequest { fields: vec![] },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn prompts_get_version(
        &self,
        prompt_id: &str,
        version: i64,
        request: &PromptsGetVersionQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Prompt, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v2/prompts/{}/versions/{}", prompt_id, version),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// UpdatePromptVersionMetadata
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
    ///         .prompts
    ///         .prompts_update_version_metadata(
    ///             &"prompt_id".to_string(),
    ///             1,
    ///             &PromptsUpdateVersionMetadataPromptsRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn prompts_update_version_metadata(
        &self,
        prompt_id: &str,
        version: i64,
        request: &PromptsUpdateVersionMetadataPromptsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Prompt, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v2/prompts/{}/versions/{}", prompt_id, version),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
