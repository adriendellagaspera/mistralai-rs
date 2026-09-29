pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct FetchCampaignStatusResponse {
    pub status: BaseTaskStatus,
}

impl FetchCampaignStatusResponse {
    pub fn builder() -> FetchCampaignStatusResponseBuilder {
        <FetchCampaignStatusResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FetchCampaignStatusResponseBuilder {
    status: Option<BaseTaskStatus>,
}

impl FetchCampaignStatusResponseBuilder {
    pub fn status(mut self, value: BaseTaskStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FetchCampaignStatusResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](FetchCampaignStatusResponseBuilder::status)
    pub fn build(self) -> Result<FetchCampaignStatusResponse, BuildError> {
        Ok(FetchCampaignStatusResponse {
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
