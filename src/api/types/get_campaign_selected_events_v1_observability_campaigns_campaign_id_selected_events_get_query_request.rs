pub use crate::prelude::*;

/// Query parameters for get_campaign_selected_events_v1_observability_campaigns__campaign_id__selected_events_get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCampaignSelectedEventsV1ObservabilityCampaignsCampaignIdSelectedEventsGetQueryRequest
{
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
}

impl GetCampaignSelectedEventsV1ObservabilityCampaignsCampaignIdSelectedEventsGetQueryRequest {
    pub fn builder() -> GetCampaignSelectedEventsV1ObservabilityCampaignsCampaignIdSelectedEventsGetQueryRequestBuilder{
        <GetCampaignSelectedEventsV1ObservabilityCampaignsCampaignIdSelectedEventsGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCampaignSelectedEventsV1ObservabilityCampaignsCampaignIdSelectedEventsGetQueryRequestBuilder
{
    page_size: Option<i64>,
    page: Option<i64>,
}

impl
    GetCampaignSelectedEventsV1ObservabilityCampaignsCampaignIdSelectedEventsGetQueryRequestBuilder
{
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetCampaignSelectedEventsV1ObservabilityCampaignsCampaignIdSelectedEventsGetQueryRequest`].
    pub fn build(
        self,
    ) -> Result<
        GetCampaignSelectedEventsV1ObservabilityCampaignsCampaignIdSelectedEventsGetQueryRequest,
        BuildError,
    > {
        Ok(GetCampaignSelectedEventsV1ObservabilityCampaignsCampaignIdSelectedEventsGetQueryRequest {
            page_size: self.page_size,
            page: self.page,
        })
    }
}
