pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListCampaignsResponse {
    #[serde(default)]
    pub campaigns: PaginatedResultCampaignPreview,
}

impl ListCampaignsResponse {
    pub fn builder() -> ListCampaignsResponseBuilder {
        <ListCampaignsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListCampaignsResponseBuilder {
    campaigns: Option<PaginatedResultCampaignPreview>,
}

impl ListCampaignsResponseBuilder {
    pub fn campaigns(mut self, value: PaginatedResultCampaignPreview) -> Self {
        self.campaigns = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListCampaignsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`campaigns`](ListCampaignsResponseBuilder::campaigns)
    pub fn build(self) -> Result<ListCampaignsResponse, BuildError> {
        Ok(ListCampaignsResponse {
            campaigns: self
                .campaigns
                .ok_or_else(|| BuildError::missing_field("campaigns"))?,
        })
    }
}
