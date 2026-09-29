use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct CampaignsClient {
    pub http_client: HttpClient,
}

impl CampaignsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Get all campaigns
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
    ///         .campaigns
    ///         .get_campaigns_v1observability_campaigns_get(
    ///             &GetCampaignsV1ObservabilityCampaignsGetQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_campaigns_v1observability_campaigns_get(
        &self,
        request: &GetCampaignsV1ObservabilityCampaignsGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListCampaignsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/observability/campaigns",
                None,
                QueryBuilder::new()
                    .int("page_size", request.page_size.clone())
                    .int("page", request.page.clone())
                    .serialize("q", request.q.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create and start a new campaign
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
    ///         .campaigns
    ///         .create_campaign_v1observability_campaigns_post(
    ///             &CreateCampaignRequest {
    ///                 description: "description".to_string(),
    ///                 judge_id: "judge_id".to_string(),
    ///                 max_nb_events: 1,
    ///                 name: "name".to_string(),
    ///                 search_params: FilterPayload {
    ///                     ..Default::default()
    ///                 },
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_campaign_v1observability_campaigns_post(
        &self,
        request: &CreateCampaignRequest,
        options: Option<RequestOptions>,
    ) -> Result<Campaign, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/observability/campaigns",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get campaign by id
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
    ///         .campaigns
    ///         .get_campaign_by_id_v1observability_campaigns_campaign_id_get(
    ///             &"campaign_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_campaign_by_id_v1observability_campaigns_campaign_id_get(
        &self,
        campaign_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Campaign, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/observability/campaigns/{}", campaign_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Delete a campaign
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
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .observability
    ///         .campaigns
    ///         .delete_campaign_v1observability_campaigns_campaign_id_delete(
    ///             &"campaign_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete_campaign_v1observability_campaigns_campaign_id_delete(
        &self,
        campaign_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/observability/campaigns/{}", campaign_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Get event ids that were selected by the given campaign
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
    ///     client.beta.observability.campaigns.get_campaign_selected_events_v1observability_campaigns_campaign_id_selected_events_get(&"campaign_id".to_string(), &GetCampaignSelectedEventsV1ObservabilityCampaignsCampaignIDSelectedEventsGetQueryRequest {
    ///         ..Default::default()
    ///     }, None).await;
    /// }
    /// ```
    pub async fn get_campaign_selected_events_v1observability_campaigns_campaign_id_selected_events_get(
        &self,
        campaign_id: &str,
        request: &GetCampaignSelectedEventsV1ObservabilityCampaignsCampaignIdSelectedEventsGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListCampaignSelectedEventsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/observability/campaigns/{}/selected-events", campaign_id),
                None,
                QueryBuilder::new()
                    .int("page_size", request.page_size.clone())
                    .int("page", request.page.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get campaign status by campaign id
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
    ///         .campaigns
    ///         .get_campaign_status_by_id_v1observability_campaigns_campaign_id_status_get(
    ///             &"campaign_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_campaign_status_by_id_v1observability_campaigns_campaign_id_status_get(
        &self,
        campaign_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<FetchCampaignStatusResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/observability/campaigns/{}/status", campaign_id),
                None,
                None,
                options,
            )
            .await
    }
}
