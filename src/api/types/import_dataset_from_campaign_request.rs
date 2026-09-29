pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ImportDatasetFromCampaignRequest {
    #[serde(default)]
    pub campaign_id: String,
}

impl ImportDatasetFromCampaignRequest {
    pub fn builder() -> ImportDatasetFromCampaignRequestBuilder {
        <ImportDatasetFromCampaignRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ImportDatasetFromCampaignRequestBuilder {
    campaign_id: Option<String>,
}

impl ImportDatasetFromCampaignRequestBuilder {
    pub fn campaign_id(mut self, value: impl Into<String>) -> Self {
        self.campaign_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ImportDatasetFromCampaignRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`campaign_id`](ImportDatasetFromCampaignRequestBuilder::campaign_id)
    pub fn build(self) -> Result<ImportDatasetFromCampaignRequest, BuildError> {
        Ok(ImportDatasetFromCampaignRequest {
            campaign_id: self
                .campaign_id
                .ok_or_else(|| BuildError::missing_field("campaign_id"))?,
        })
    }
}
