pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PaginatedResultCampaignPreview {
    #[serde(default)]
    pub count: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<Campaign>>,
}

impl PaginatedResultCampaignPreview {
    pub fn builder() -> PaginatedResultCampaignPreviewBuilder {
        <PaginatedResultCampaignPreviewBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaginatedResultCampaignPreviewBuilder {
    count: Option<i64>,
    next: Option<String>,
    previous: Option<String>,
    results: Option<Vec<Campaign>>,
}

impl PaginatedResultCampaignPreviewBuilder {
    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn next(mut self, value: impl Into<String>) -> Self {
        self.next = Some(value.into());
        self
    }

    pub fn previous(mut self, value: impl Into<String>) -> Self {
        self.previous = Some(value.into());
        self
    }

    pub fn results(mut self, value: Vec<Campaign>) -> Self {
        self.results = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaginatedResultCampaignPreview`].
    /// This method will fail if any of the following fields are not set:
    /// - [`count`](PaginatedResultCampaignPreviewBuilder::count)
    pub fn build(self) -> Result<PaginatedResultCampaignPreview, BuildError> {
        Ok(PaginatedResultCampaignPreview {
            count: self
                .count
                .ok_or_else(|| BuildError::missing_field("count"))?,
            next: self.next,
            previous: self.previous,
            results: self.results,
        })
    }
}
