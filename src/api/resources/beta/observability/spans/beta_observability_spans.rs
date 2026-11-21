use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct SpansClient {
    pub http_client: HttpClient,
}

impl SpansClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Search pipeline results
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
    ///         .spans
    ///         .search_pipeline_results(
    ///             &PipelineResultsRequest {
    ///                 pipeline_config_slugs: vec!["pipeline_config_slugs".to_string()],
    ///                 from: None,
    ///                 to: None,
    ///                 page_size: None,
    ///                 cursor: None,
    ///                 search_expression: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn search_pipeline_results(
        &self,
        request: &PipelineResultsRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetPipelineResults, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/observability/pipeline-results/search",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .serialize("from", request.from.clone())
                    .serialize("to", request.to.clone())
                    .int("page_size", request.page_size.clone())
                    .serialize("cursor", request.cursor.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get pipeline result field definitions
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
    ///         .spans
    ///         .list_pipeline_result_fields(&"pipeline_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn list_pipeline_result_fields(
        &self,
        pipeline_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<GetPipelineResultFields, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/observability/pipelines/{}/results/fields", pipeline_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Aggregate spans
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
    ///         .spans
    ///         .aggregate(
    ///             &AggregateRequest {
    ///                 body: AggregationRequest {
    ///                     dimensions: None,
    ///                     limit: None,
    ///                     metric: MetricDefinition {
    ///                         aggregation: MetricAggregation::Count,
    ///                         measure: "measure".to_string(),
    ///                     },
    ///                     order_by: None,
    ///                     search_expression: None,
    ///                     time_dimension: None,
    ///                 },
    ///                 from: None,
    ///                 to: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn aggregate(
        &self,
        request: &AggregateRequest,
        options: Option<RequestOptions>,
    ) -> Result<Aggregation, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/observability/spans/aggregate",
                Some(serde_json::to_value(&request.body).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .serialize("from", request.from.clone())
                    .serialize("to", request.to.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Aggregate span evaluations
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
    ///         .spans
    ///         .aggregate_span_evaluations(
    ///             &AggregateSpanEvaluationsRequest {
    ///                 body: AggregationRequest {
    ///                     dimensions: None,
    ///                     limit: None,
    ///                     metric: MetricDefinition {
    ///                         aggregation: MetricAggregation::Count,
    ///                         measure: "measure".to_string(),
    ///                     },
    ///                     order_by: None,
    ///                     search_expression: None,
    ///                     time_dimension: None,
    ///                 },
    ///                 from: None,
    ///                 to: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn aggregate_span_evaluations(
        &self,
        request: &AggregateSpanEvaluationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Aggregation, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/observability/spans/evaluations/aggregate",
                Some(serde_json::to_value(&request.body).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .serialize("from", request.from.clone())
                    .serialize("to", request.to.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get span evaluation field definitions
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
    ///         .spans
    ///         .list_span_eval_fields(None)
    ///         .await;
    /// }
    /// ```
    pub async fn list_span_eval_fields(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<GetSpanEvaluationFields, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/observability/spans/evaluations/fields",
                None,
                None,
                options,
            )
            .await
    }

    /// Get options for a span evaluation field
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
    ///         .spans
    ///         .fetch_span_eval_field_options(
    ///             &"field_name".to_string(),
    ///             &FetchSpanEvalFieldOptionsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn fetch_span_eval_field_options(
        &self,
        field_name: &str,
        request: &FetchSpanEvalFieldOptionsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetSpanEvaluationFieldOptions, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/observability/spans/evaluations/fields/{}/options",
                    field_name
                ),
                None,
                QueryBuilder::new()
                    .serialize("from", request.from.clone())
                    .serialize("to", request.to.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Search span evaluations
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
    ///         .spans
    ///         .search_span_evaluations(
    ///             &SearchSpanEvaluationsRequest {
    ///                 body: SpanEvaluationsRequest {
    ///                     ..Default::default()
    ///                 },
    ///                 from: None,
    ///                 to: None,
    ///                 page_size: None,
    ///                 cursor: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn search_span_evaluations(
        &self,
        request: &SearchSpanEvaluationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetSpanEvaluations, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/observability/spans/evaluations/search",
                Some(serde_json::to_value(&request.body).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .serialize("from", request.from.clone())
                    .serialize("to", request.to.clone())
                    .int("page_size", request.page_size.clone())
                    .serialize("cursor", request.cursor.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Search latest span evaluations
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
    ///         .spans
    ///         .search_latest_span_evaluations(
    ///             &SearchLatestSpanEvaluationsRequest {
    ///                 body: SpanEvaluationsRequest {
    ///                     ..Default::default()
    ///                 },
    ///                 from: None,
    ///                 to: None,
    ///                 page_size: None,
    ///                 cursor: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn search_latest_span_evaluations(
        &self,
        request: &SearchLatestSpanEvaluationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetSpanEvaluations, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/observability/spans/evaluations/search/latest",
                Some(serde_json::to_value(&request.body).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .serialize("from", request.from.clone())
                    .serialize("to", request.to.clone())
                    .int("page_size", request.page_size.clone())
                    .serialize("cursor", request.cursor.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get span field definitions
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
    ///     client.beta.observability.spans.list_span_fields(None).await;
    /// }
    /// ```
    pub async fn list_span_fields(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<GetSpanFields, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/observability/spans/fields",
                None,
                None,
                options,
            )
            .await
    }

    /// Get options for a span field
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
    ///         .spans
    ///         .fetch_span_field_options(
    ///             &"field_name".to_string(),
    ///             &FetchSpanFieldOptionsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn fetch_span_field_options(
        &self,
        field_name: &str,
        request: &FetchSpanFieldOptionsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetSpanFieldOptions, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/observability/spans/fields/{}/options", field_name),
                None,
                QueryBuilder::new()
                    .serialize("from", request.from.clone())
                    .serialize("to", request.to.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Search spans
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
    ///         .spans
    ///         .search_spans(
    ///             &SpansRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn search_spans(
        &self,
        request: &SpansRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetSpans, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/observability/spans/search",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .serialize("from", request.from.clone())
                    .serialize("to", request.to.clone())
                    .int("page_size", request.page_size.clone())
                    .serialize("cursor", request.cursor.clone())
                    .build(),
                options,
            )
            .await
    }
}
