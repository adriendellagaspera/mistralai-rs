use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct LogsClient {
    pub http_client: HttpClient,
}

impl LogsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Search logs
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
    ///         .logs
    ///         .search_logs_v1observability_logs_search_post(
    ///             &LogsRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn search_logs_v1observability_logs_search_post(
        &self,
        request: &LogsRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetLogs, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/observability/logs/search",
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

    /// Get log field definitions
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
    ///         .logs
    ///         .get_log_fields_v1observability_logs_fields_get(None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_log_fields_v1observability_logs_fields_get(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<GetLogFields, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/observability/logs/fields",
                None,
                None,
                options,
            )
            .await
    }

    /// Get options for a log field
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
    ///         .logs
    ///         .get_log_field_options_v1observability_logs_fields_field_name_options_get(
    ///             &"field_name".to_string(),
    ///             &GetLogFieldOptionsV1ObservabilityLogsFieldsFieldNameOptionsGetQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_log_field_options_v1observability_logs_fields_field_name_options_get(
        &self,
        field_name: &str,
        request: &GetLogFieldOptionsV1ObservabilityLogsFieldsFieldNameOptionsGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetLogFieldOptions, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/observability/logs/fields/{}/options", field_name),
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
