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
    /// * `fields` - Projection over each listed Skill. definition.description is the only definition path a list accepts; body and assets are read one skill at a time with GetSkill.
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
    ///         .beta
    ///         .skills
    ///         .list(
    ///             &BetaSkillsListQueryRequest {
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
    pub async fn list(
        &self,
        request: &BetaSkillsListQueryRequest,
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
    ///         .beta
    ///         .skills
    ///         .create(
    ///             &CreateSkillRequest {
    ///                 definition: SkillDefinition {
    ///                     ..Default::default()
    ///                 },
    ///                 name: "name".to_string(),
    ///                 aliases: None,
    ///                 notes: None,
    ///                 sharing_scope: None,
    ///                 workspace_relation: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
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
    ///         .beta
    ///         .skills
    ///         .get(
    ///             &"skill_id".to_string(),
    ///             &BetaSkillsGetQueryRequest {
    ///                 version: Some(1),
    ///                 alias: None,
    ///                 fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        skill_id: &str,
        request: &BetaSkillsGetQueryRequest,
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
    ///         .beta
    ///         .skills
    ///         .delete(&"skill_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
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
    ///         .beta
    ///         .skills
    ///         .update_metadata(
    ///             &"skill_id".to_string(),
    ///             &UpdateMetadataSkillsRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_metadata(
        &self,
        skill_id: &str,
        request: &UpdateMetadataSkillsRequest,
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
    ///         .beta
    ///         .skills
    ///         .list_versions(&"skill_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn list_versions(
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
    ///         .beta
    ///         .skills
    ///         .create_version(
    ///             &"skill_id".to_string(),
    ///             &CreateVersionSkillsRequest {
    ///                 definition: SkillDefinition {
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
    pub async fn create_version(
        &self,
        skill_id: &str,
        request: &CreateVersionSkillsRequest,
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
    ///         .beta
    ///         .skills
    ///         .get_version(
    ///             &"skill_id".to_string(),
    ///             1,
    ///             &BetaSkillsGetVersionQueryRequest { fields: vec![] },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_version(
        &self,
        skill_id: &str,
        version: i64,
        request: &BetaSkillsGetVersionQueryRequest,
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
    ///         .beta
    ///         .skills
    ///         .update_version_metadata(
    ///             &"skill_id".to_string(),
    ///             1,
    ///             &UpdateVersionMetadataSkillsRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_version_metadata(
        &self,
        skill_id: &str,
        version: i64,
        request: &UpdateVersionMetadataSkillsRequest,
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
