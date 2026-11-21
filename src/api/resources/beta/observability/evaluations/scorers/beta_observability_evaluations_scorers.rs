use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ScorersClient {
    pub http_client: HttpClient,
}

impl ScorersClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// List scorers
    ///
    /// # Arguments
    ///
    /// * `created_by_me` - Only return scorers created by the current user
    /// * `ootb` - Only return out-of-the-box scorers
    /// * `q` - Case-insensitive filter on scorer slug, name, or description
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
    ///         .observability
    ///         .evaluations
    ///         .scorers
    ///         .list_scorers_v1observability_scorers_get(
    ///             &ListScorersV1ObservabilityScorersGetQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_scorers_v1observability_scorers_get(
        &self,
        request: &ListScorersV1ObservabilityScorersGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListScorersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/observability/scorers",
                None,
                QueryBuilder::new()
                    .bool("created_by_me", request.created_by_me.clone())
                    .bool("ootb", request.ootb.clone())
                    .serialize("q", request.q.clone())
                    .int("page_size", request.page_size.clone())
                    .int("page", request.page.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a scorer
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
    ///         .observability
    ///         .evaluations
    ///         .scorers
    ///         .create_scorer_v1observability_scorers_post(
    ///             &CreateScorerRequest {
    ///                 config: LlmJudgeConfig {
    ///                     model: "model".to_string(),
    ///                     output_spec: EvaluationResultSpec::boolean(),
    ///                     params: None,
    ///                     prompt_template: "prompt_template".to_string(),
    ///                 },
    ///                 kind: ScorerKind::LlmJudge,
    ///                 name: "name".to_string(),
    ///                 description: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_scorer_v1observability_scorers_post(
        &self,
        request: &CreateScorerRequest,
        options: Option<RequestOptions>,
    ) -> Result<Scorer, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/observability/scorers",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get a scorer by slug
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
    ///         .observability
    ///         .evaluations
    ///         .scorers
    ///         .get_scorer_v1observability_scorers_scorer_slug_get(&"scorer_slug".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_scorer_v1observability_scorers_scorer_slug_get(
        &self,
        scorer_slug: &str,
        options: Option<RequestOptions>,
    ) -> Result<Scorer, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/observability/scorers/{}", scorer_slug),
                None,
                None,
                options,
            )
            .await
    }

    /// Edit a scorer
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
    ///         .observability
    ///         .evaluations
    ///         .scorers
    ///         .edit_scorer_v1observability_scorers_scorer_slug_patch(
    ///             &"scorer_slug".to_string(),
    ///             &EditScorerRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn edit_scorer_v1observability_scorers_scorer_slug_patch(
        &self,
        scorer_slug: &str,
        request: &EditScorerRequest,
        options: Option<RequestOptions>,
    ) -> Result<Scorer, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/observability/scorers/{}", scorer_slug),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
