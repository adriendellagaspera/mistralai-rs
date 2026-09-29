use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct TracesClient {
    pub http_client: HttpClient,
}

impl TracesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Aggregate traces
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
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .observability
    ///         .traces
    ///         .aggregate_traces_v1observability_traces_aggregate_post(
    ///             &AggregateTracesV1ObservabilityTracesAggregatePostRequest {
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
    pub async fn aggregate_traces_v1observability_traces_aggregate_post(
        &self,
        request: &AggregateTracesV1ObservabilityTracesAggregatePostRequest,
        options: Option<RequestOptions>,
    ) -> Result<Aggregation, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/observability/traces/aggregate",
                Some(serde_json::to_value(&request.body).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .serialize("from", request.from.clone())
                    .serialize("to", request.to.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get trace field definitions
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
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .observability
    ///         .traces
    ///         .get_trace_fields_v1observability_traces_fields_get(None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_trace_fields_v1observability_traces_fields_get(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<GetTraceFields, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/observability/traces/fields",
                None,
                None,
                options,
            )
            .await
    }

    /// Get options for a trace field
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
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .observability
    ///         .traces
    ///         .get_trace_field_options_v1observability_traces_fields_field_name_options_get(
    ///             &"field_name".to_string(),
    ///             &GetTraceFieldOptionsV1ObservabilityTracesFieldsFieldNameOptionsGetQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_trace_field_options_v1observability_traces_fields_field_name_options_get(
        &self,
        field_name: &str,
        request: &GetTraceFieldOptionsV1ObservabilityTracesFieldsFieldNameOptionsGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetTraceFieldOptions, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/observability/traces/fields/{}/options", field_name),
                None,
                QueryBuilder::new()
                    .serialize("from", request.from.clone())
                    .serialize("to", request.to.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Search traces
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
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .observability
    ///         .traces
    ///         .search_traces_v1observability_traces_search_post(
    ///             &TracesRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn search_traces_v1observability_traces_search_post(
        &self,
        request: &TracesRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetTraces, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/observability/traces/search",
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

    /// Get trace by id
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
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .observability
    ///         .traces
    ///         .get_trace_by_id_v1observability_traces_trace_id_get(&"trace_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_trace_by_id_v1observability_traces_trace_id_get(
        &self,
        trace_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<GetTrace, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/observability/traces/{}", trace_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Get trace spans
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
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .observability
    ///         .traces
    ///         .get_trace_spans_v1observability_traces_trace_id_spans_get(
    ///             &"trace_id".to_string(),
    ///             &GetTraceSpansV1ObservabilityTracesTraceIDSpansGetQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_trace_spans_v1observability_traces_trace_id_spans_get(
        &self,
        trace_id: &str,
        request: &GetTraceSpansV1ObservabilityTracesTraceIdSpansGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetSpans, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/observability/traces/{}/spans", trace_id),
                None,
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

    /// Get span by id
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
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .observability
    ///         .traces
    ///         .get_span_by_id_v1observability_traces_trace_id_spans_span_id_get(
    ///             &"trace_id".to_string(),
    ///             &"span_id".to_string(),
    ///             &GetSpanByIDV1ObservabilityTracesTraceIDSpansSpanIDGetQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_span_by_id_v1observability_traces_trace_id_spans_span_id_get(
        &self,
        trace_id: &str,
        span_id: &str,
        request: &GetSpanByIdV1ObservabilityTracesTraceIdSpansSpanIdGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetSpan, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/observability/traces/{}/spans/{}", trace_id, span_id),
                None,
                QueryBuilder::new()
                    .serialize("from", request.from.clone())
                    .serialize("to", request.to.clone())
                    .build(),
                options,
            )
            .await
    }
}
