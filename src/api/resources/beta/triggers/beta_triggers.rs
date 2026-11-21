use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct TriggersClient {
    pub http_client: HttpClient,
}

impl TriggersClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// List Triggers
    ///
    /// # Arguments
    ///
    /// * `handler` - Filter by handler type
    /// * `primitive_id` - Filter by primitive ID (workflow, agent, or app)
    /// * `event_source_id` - Filter by event source ID (webhook id)
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
    ///         .triggers
    ///         .list_triggers_v1triggers_get(
    ///             &ListTriggersV1TriggersGetQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_triggers_v1triggers_get(
        &self,
        request: &ListTriggersV1TriggersGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Vec<Trigger>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/triggers",
                None,
                QueryBuilder::new()
                    .serialize("handler", request.handler.clone())
                    .serialize("primitive_id", request.primitive_id.clone())
                    .serialize("event_source_id", request.event_source_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create Trigger
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
    ///         .triggers
    ///         .create_trigger_v1triggers_post(
    ///             &CreateTrigger {
    ///                 action: CreateTriggerAction {
    ///                     primitive_id: "primitive_id".to_string(),
    ///                     r#type: TriggerActionType::StartWorkflow,
    ///                 },
    ///                 event_source_id: "event_source_id".to_string(),
    ///                 handler: TriggerHandler::Workflows,
    ///                 owner_type: TriggersOwnerType::Workspace,
    ///                 title: "title".to_string(),
    ///                 filters: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_trigger_v1triggers_post(
        &self,
        request: &CreateTrigger,
        options: Option<RequestOptions>,
    ) -> Result<Trigger, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/triggers",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get Trigger
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
    ///         .triggers
    ///         .get_trigger_v1triggers_trigger_id_get(&"trigger_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_trigger_v1triggers_trigger_id_get(
        &self,
        trigger_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Trigger, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/triggers/{}", trigger_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Delete Trigger
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
    ///         .triggers
    ///         .delete_trigger_v1triggers_trigger_id_delete(&"trigger_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_trigger_v1triggers_trigger_id_delete(
        &self,
        trigger_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/triggers/{}", trigger_id),
                None,
                None,
                options,
            )
            .await
    }

    /// List Trigger Events
    ///
    /// # Arguments
    ///
    /// * `limit` - Maximum number of events to return
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
    ///         .triggers
    ///         .list_trigger_events_v1triggers_trigger_id_events_get(
    ///             &"trigger_id".to_string(),
    ///             &ListTriggerEventsV1TriggersTriggerIDEventsGetQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_trigger_events_v1triggers_trigger_id_events_get(
        &self,
        trigger_id: &str,
        request: &ListTriggerEventsV1TriggersTriggerIdEventsGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<TriggerEventList, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/triggers/{}/events", trigger_id),
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .build(),
                options,
            )
            .await
    }
}
