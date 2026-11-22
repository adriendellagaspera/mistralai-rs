use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct RecordsClient2 {
    pub http_client: HttpClient,
}

impl RecordsClient2 {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Search records for an evaluation run with content filters
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
    ///         .records
    ///         .search(
    ///             &"run_id".to_string(),
    ///             &SearchRequest {
    ///                 body: Some(EvaluationRunRecordFilterInput {
    ///                     ..Default::default()
    ///                 }),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn search(
        &self,
        run_id: &str,
        request: &SearchRequest3,
        options: Option<RequestOptions>,
    ) -> Result<ListEvaluationRunRecordsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/observability/evaluation-runs_v2/{}/records/search",
                    run_id
                ),
                Some(serde_json::to_value(&request.body).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .int("page_size", request.page_size.clone())
                    .int("page", request.page.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get a specific record from an evaluation run
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
    ///         .records
    ///         .get(&"run_id".to_string(), &"record_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        run_id: &str,
        record_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<EvaluationRunRecordV2Response, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/observability/evaluation-runs_v2/{}/records/{}",
                    run_id, record_id
                ),
                None,
                None,
                options,
            )
            .await
    }
}
