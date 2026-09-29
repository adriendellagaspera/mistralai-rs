pub use crate::prelude::*;

/// Query parameters for get_campaigns_v1_observability_campaigns_get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCampaignsV1ObservabilityCampaignsGetQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
}

impl GetCampaignsV1ObservabilityCampaignsGetQueryRequest {
    pub fn builder() -> GetCampaignsV1ObservabilityCampaignsGetQueryRequestBuilder {
        <GetCampaignsV1ObservabilityCampaignsGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCampaignsV1ObservabilityCampaignsGetQueryRequestBuilder {
    page_size: Option<i64>,
    page: Option<i64>,
    q: Option<String>,
}

impl GetCampaignsV1ObservabilityCampaignsGetQueryRequestBuilder {
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn q(mut self, value: impl Into<String>) -> Self {
        self.q = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetCampaignsV1ObservabilityCampaignsGetQueryRequest`].
    pub fn build(self) -> Result<GetCampaignsV1ObservabilityCampaignsGetQueryRequest, BuildError> {
        Ok(GetCampaignsV1ObservabilityCampaignsGetQueryRequest {
            page_size: self.page_size,
            page: self.page,
            q: self.q,
        })
    }
}
