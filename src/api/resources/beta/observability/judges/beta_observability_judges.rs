use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct JudgesClient {
    pub http_client: HttpClient,
}

impl JudgesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Get judges with optional filtering and search
    ///
    /// # Arguments
    ///
    /// * `type_filter` - Filter by judge output types
    /// * `model_filter` - Filter by model names
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
    ///         .observability
    ///         .judges
    ///         .get_judges_v1observability_judges_get(
    ///             &GetJudgesV1ObservabilityJudgesGetQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_judges_v1observability_judges_get(
        &self,
        request: &GetJudgesV1ObservabilityJudgesGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListJudgesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/observability/judges",
                None,
                QueryBuilder::new()
                    .serialize("type_filter", request.type_filter.clone())
                    .serialize("model_filter", request.model_filter.clone())
                    .int("page_size", request.page_size.clone())
                    .int("page", request.page.clone())
                    .serialize("q", request.q.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a new judge
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
    ///         .observability
    ///         .judges
    ///         .create_judge_v1observability_judges_post(
    ///             &CreateJudgeRequest {
    ///                 name: "name".to_string(),
    ///                 description: "description".to_string(),
    ///                 model_name: "model_name".to_string(),
    ///                 output: CreateJudgeRequestOutput::Classification {
    ///                     data: JudgeClassificationOutput {
    ///                         options: vec![JudgeClassificationOutputOption {
    ///                             value: "value".to_string(),
    ///                             description: "description".to_string(),
    ///                             ..Default::default()
    ///                         }],
    ///                         ..Default::default()
    ///                     },
    ///                 },
    ///                 instructions: "instructions".to_string(),
    ///                 tools: vec!["tools".to_string()],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_judge_v1observability_judges_post(
        &self,
        request: &CreateJudgeRequest,
        options: Option<RequestOptions>,
    ) -> Result<Judge, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/observability/judges",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get judge by id
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
    ///         .observability
    ///         .judges
    ///         .get_judge_by_id_v1observability_judges_judge_id_get(&"judge_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_judge_by_id_v1observability_judges_judge_id_get(
        &self,
        judge_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Judge, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/observability/judges/{}", judge_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a judge
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
    ///         .observability
    ///         .judges
    ///         .update_judge_v1observability_judges_judge_id_put(
    ///             &"judge_id".to_string(),
    ///             &UpdateJudgeRequest {
    ///                 name: "name".to_string(),
    ///                 description: "description".to_string(),
    ///                 model_name: "model_name".to_string(),
    ///                 output: UpdateJudgeRequestOutput::Classification {
    ///                     data: JudgeClassificationOutput {
    ///                         options: vec![JudgeClassificationOutputOption {
    ///                             value: "value".to_string(),
    ///                             description: "description".to_string(),
    ///                             ..Default::default()
    ///                         }],
    ///                         ..Default::default()
    ///                     },
    ///                 },
    ///                 instructions: "instructions".to_string(),
    ///                 tools: vec!["tools".to_string()],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_judge_v1observability_judges_judge_id_put(
        &self,
        judge_id: &str,
        request: &UpdateJudgeRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("v1/observability/judges/{}", judge_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Delete a judge
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
    ///         .observability
    ///         .judges
    ///         .delete_judge_v1observability_judges_judge_id_delete(&"judge_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_judge_v1observability_judges_judge_id_delete(
        &self,
        judge_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/observability/judges/{}", judge_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Run a saved judge on a conversation
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
    ///         .observability
    ///         .judges
    ///         .judge_conversation_v1observability_judges_judge_id_live_judging_post(
    ///             &"judge_id".to_string(),
    ///             &JudgeConversationRequest {
    ///                 messages: vec![HashMap::from([(
    ///                     "key".to_string(),
    ///                     serde_json::json!("value"),
    ///                 )])],
    ///                 properties: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn judge_conversation_v1observability_judges_judge_id_live_judging_post(
        &self,
        judge_id: &str,
        request: &JudgeConversationRequest,
        options: Option<RequestOptions>,
    ) -> Result<JudgeOutput, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/observability/judges/{}/live-judging", judge_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
