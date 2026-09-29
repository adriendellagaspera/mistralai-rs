pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaginationInfo {
    #[serde(default)]
    pub total_items: i64,
    #[serde(default)]
    pub total_pages: i64,
    #[serde(default)]
    pub current_page: i64,
    #[serde(default)]
    pub page_size: i64,
    #[serde(default)]
    pub has_more: bool,
}

impl PaginationInfo {
    pub fn builder() -> PaginationInfoBuilder {
        <PaginationInfoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaginationInfoBuilder {
    total_items: Option<i64>,
    total_pages: Option<i64>,
    current_page: Option<i64>,
    page_size: Option<i64>,
    has_more: Option<bool>,
}

impl PaginationInfoBuilder {
    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    pub fn total_pages(mut self, value: i64) -> Self {
        self.total_pages = Some(value);
        self
    }

    pub fn current_page(mut self, value: i64) -> Self {
        self.current_page = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaginationInfo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`total_items`](PaginationInfoBuilder::total_items)
    /// - [`total_pages`](PaginationInfoBuilder::total_pages)
    /// - [`current_page`](PaginationInfoBuilder::current_page)
    /// - [`page_size`](PaginationInfoBuilder::page_size)
    /// - [`has_more`](PaginationInfoBuilder::has_more)
    pub fn build(self) -> Result<PaginationInfo, BuildError> {
        Ok(PaginationInfo {
            total_items: self
                .total_items
                .ok_or_else(|| BuildError::missing_field("total_items"))?,
            total_pages: self
                .total_pages
                .ok_or_else(|| BuildError::missing_field("total_pages"))?,
            current_page: self
                .current_page
                .ok_or_else(|| BuildError::missing_field("current_page"))?,
            page_size: self
                .page_size
                .ok_or_else(|| BuildError::missing_field("page_size"))?,
            has_more: self
                .has_more
                .ok_or_else(|| BuildError::missing_field("has_more"))?,
        })
    }
}
