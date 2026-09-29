pub use crate::prelude::*;

/// Schema for voice list response
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct VoiceListResponse {
    #[serde(default)]
    pub items: Vec<VoiceResponse>,
    #[serde(default)]
    pub total: i64,
    #[serde(default)]
    pub page: i64,
    #[serde(default)]
    pub page_size: i64,
    #[serde(default)]
    pub total_pages: i64,
}

impl VoiceListResponse {
    pub fn builder() -> VoiceListResponseBuilder {
        <VoiceListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VoiceListResponseBuilder {
    items: Option<Vec<VoiceResponse>>,
    total: Option<i64>,
    page: Option<i64>,
    page_size: Option<i64>,
    total_pages: Option<i64>,
}

impl VoiceListResponseBuilder {
    pub fn items(mut self, value: Vec<VoiceResponse>) -> Self {
        self.items = Some(value);
        self
    }

    pub fn total(mut self, value: i64) -> Self {
        self.total = Some(value);
        self
    }

    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn total_pages(mut self, value: i64) -> Self {
        self.total_pages = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VoiceListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`items`](VoiceListResponseBuilder::items)
    /// - [`total`](VoiceListResponseBuilder::total)
    /// - [`page`](VoiceListResponseBuilder::page)
    /// - [`page_size`](VoiceListResponseBuilder::page_size)
    /// - [`total_pages`](VoiceListResponseBuilder::total_pages)
    pub fn build(self) -> Result<VoiceListResponse, BuildError> {
        Ok(VoiceListResponse {
            items: self
                .items
                .ok_or_else(|| BuildError::missing_field("items"))?,
            total: self
                .total
                .ok_or_else(|| BuildError::missing_field("total"))?,
            page: self.page.ok_or_else(|| BuildError::missing_field("page"))?,
            page_size: self
                .page_size
                .ok_or_else(|| BuildError::missing_field("page_size"))?,
            total_pages: self
                .total_pages
                .ok_or_else(|| BuildError::missing_field("total_pages"))?,
        })
    }
}
