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
    ///         .aggregate_spans_v1observability_spans_aggregate_post(
    ///             &AggregateSpansV1ObservabilitySpansAggregatePostRequest {
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
    pub async fn aggregate_spans_v1observability_spans_aggregate_post(
        &self,
        request: &AggregateSpansV1ObservabilitySpansAggregatePostRequest,
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
    ///         .get_span_evaluation_fields_v1observability_spans_evaluations_fields_get(None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_span_evaluation_fields_v1observability_spans_evaluations_fields_get(
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
    ///     client.beta.observability.spans.get_span_evaluation_field_options_v1observability_spans_evaluations_fields_field_name_options_get(&"field_name".to_string(), &GetSpanEvaluationFieldOptionsV1ObservabilitySpansEvaluationsFieldsFieldNameOptionsGetQueryRequest {
    ///         ..Default::default()
    ///     }, None).await;
    /// }
    /// ```
    pub async fn get_span_evaluation_field_options_v1observability_spans_evaluations_fields_field_name_options_get(
        &self,
        field_name: &str,
        request: &GetSpanEvaluationFieldOptionsV1ObservabilitySpansEvaluationsFieldsFieldNameOptionsGetQueryRequest,
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
    ///         .search_span_evaluations_v1observability_spans_evaluations_search_post(
    ///             &SearchSpanEvaluationsV1ObservabilitySpansEvaluationsSearchPostRequest {
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
    pub async fn search_span_evaluations_v1observability_spans_evaluations_search_post(
        &self,
        request: &SearchSpanEvaluationsV1ObservabilitySpansEvaluationsSearchPostRequest,
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
    ///         .search_latest_span_evaluations_v1observability_spans_evaluations_search_latest_post(
    ///             &SearchLatestSpanEvaluationsV1ObservabilitySpansEvaluationsSearchLatestPostRequest {
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
    pub async fn search_latest_span_evaluations_v1observability_spans_evaluations_search_latest_post(
        &self,
        request: &SearchLatestSpanEvaluationsV1ObservabilitySpansEvaluationsSearchLatestPostRequest,
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
    ///     client
    ///         .beta
    ///         .observability
    ///         .spans
    ///         .get_span_fields_v1observability_spans_fields_get(None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_span_fields_v1observability_spans_fields_get(
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
    ///         .get_span_field_options_v1observability_spans_fields_field_name_options_get(
    ///             &"field_name".to_string(),
    ///             &GetSpanFieldOptionsV1ObservabilitySpansFieldsFieldNameOptionsGetQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_span_field_options_v1observability_spans_fields_field_name_options_get(
        &self,
        field_name: &str,
        request: &GetSpanFieldOptionsV1ObservabilitySpansFieldsFieldNameOptionsGetQueryRequest,
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
    ///         .search_spans_v1observability_spans_search_post(
    ///             &SpansRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn search_spans_v1observability_spans_search_post(
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
