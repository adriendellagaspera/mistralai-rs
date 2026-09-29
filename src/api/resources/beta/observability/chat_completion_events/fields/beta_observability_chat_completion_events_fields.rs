use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct FieldsClient {
    pub http_client: HttpClient,
}

impl FieldsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Get Chat Completion Fields
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
    ///         .chat_completion_events
    ///         .fields
    ///         .get_chat_completion_fields_v1observability_chat_completion_fields_get(None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_chat_completion_fields_v1observability_chat_completion_fields_get(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<ListChatCompletionFieldsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/observability/chat-completion-fields",
                None,
                None,
                options,
            )
            .await
    }

    /// Get Chat Completion Field Options
    ///
    /// # Arguments
    ///
    /// * `operator` - The operator to use for filtering options
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
    ///     client.beta.observability.chat_completion_events.fields.get_chat_completion_field_options_v1observability_chat_completion_fields_field_name_options_get(&"field_name".to_string(), &GetChatCompletionFieldOptionsV1ObservabilityChatCompletionFieldsFieldNameOptionsGetQueryRequest {
    ///         operator: GetChatCompletionFieldOptionsV1ObservabilityChatCompletionFieldsFieldNameOptionsGetFieldsRequestOperator::Lt
    ///     }, None).await;
    /// }
    /// ```
    pub async fn get_chat_completion_field_options_v1observability_chat_completion_fields_field_name_options_get(
        &self,
        field_name: &str,
        request: &GetChatCompletionFieldOptionsV1ObservabilityChatCompletionFieldsFieldNameOptionsGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<FetchChatCompletionFieldOptionsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/observability/chat-completion-fields/{}/options",
                    field_name
                ),
                None,
                QueryBuilder::new()
                    .serialize("operator", Some(request.operator.clone()))
                    .build(),
                options,
            )
            .await
    }

    /// Get Chat Completion Field Options Counts
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
    ///     client.beta.observability.chat_completion_events.fields.get_chat_completion_field_options_counts_v1observability_chat_completion_fields_field_name_options_counts_post(&"field_name".to_string(), &FetchFieldOptionCountsRequest {
    ///         ..Default::default()
    ///     }, None).await;
    /// }
    /// ```
    pub async fn get_chat_completion_field_options_counts_v1observability_chat_completion_fields_field_name_options_counts_post(
        &self,
        field_name: &str,
        request: &FetchFieldOptionCountsRequest,
        options: Option<RequestOptions>,
    ) -> Result<FetchFieldOptionCountsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/observability/chat-completion-fields/{}/options-counts",
                    field_name
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
