use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct SkillsClient {
    pub http_client: HttpClient,
}

impl SkillsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// ListSkills
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
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .skills
    ///         .skills_list(
    ///             &SkillsListQueryRequest {
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
    pub async fn skills_list(
        &self,
        request: &SkillsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListSkillsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v2/skills",
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

    /// CreateSkill
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
    ///         .skills
    ///         .skills_create(
    ///             &CreateSkillRequest {
    ///                 name: "name".to_string(),
    ///                 definition: SkillDefinition {
    ///                     ..Default::default()
    ///                 },
    ///                 notes: None,
    ///                 sharing_scope: None,
    ///                 aliases: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn skills_create(
        &self,
        request: &CreateSkillRequest,
        options: Option<RequestOptions>,
    ) -> Result<Skill, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v2/skills",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// GetSkill
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
    ///         .skills
    ///         .skills_get(
    ///             &"skill_id".to_string(),
    ///             &SkillsGetQueryRequest {
    ///                 version: Some(1),
    ///                 alias: None,
    ///                 fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn skills_get(
        &self,
        skill_id: &str,
        request: &SkillsGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Skill, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v2/skills/{}", skill_id),
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

    /// DeleteSkill
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
    ///         .skills
    ///         .skills_delete(&"skill_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn skills_delete(
        &self,
        skill_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DeleteSkillResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v2/skills/{}", skill_id),
                None,
                None,
                options,
            )
            .await
    }

    /// UpdateSkill
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
    ///         .skills
    ///         .skills_update(
    ///             &"skill_id".to_string(),
    ///             &SkillsUpdateSkillsRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn skills_update(
        &self,
        skill_id: &str,
        request: &SkillsUpdateSkillsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Skill, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v2/skills/{}", skill_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// ListSkillVersions
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
    ///         .skills
    ///         .skills_list_versions(&"skill_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn skills_list_versions(
        &self,
        skill_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ListSkillVersionsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v2/skills/{}/versions", skill_id),
                None,
                None,
                options,
            )
            .await
    }

    /// CreateSkillVersion
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
    ///         .skills
    ///         .skills_create_version(
    ///             &"skill_id".to_string(),
    ///             &SkillsCreateVersionSkillsRequest {
    ///                 definition: SkillDefinition {
    ///                     ..Default::default()
    ///                 },
    ///                 notes: None,
    ///                 aliases: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn skills_create_version(
        &self,
        skill_id: &str,
        request: &SkillsCreateVersionSkillsRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateSkillVersionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v2/skills/{}/versions", skill_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// GetSkillVersion
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
    ///         .skills
    ///         .skills_get_version(
    ///             &"skill_id".to_string(),
    ///             1,
    ///             &SkillsGetVersionQueryRequest { fields: vec![] },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn skills_get_version(
        &self,
        skill_id: &str,
        version: i64,
        request: &SkillsGetVersionQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Skill, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v2/skills/{}/versions/{}", skill_id, version),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// UpdateSkillVersionMetadata
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
    ///         .skills
    ///         .skills_update_version_metadata(
    ///             &"skill_id".to_string(),
    ///             1,
    ///             &SkillsUpdateVersionMetadataSkillsRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn skills_update_version_metadata(
        &self,
        skill_id: &str,
        version: i64,
        request: &SkillsUpdateVersionMetadataSkillsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Skill, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v2/skills/{}/versions/{}", skill_id, version),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
