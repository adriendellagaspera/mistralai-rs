pub use crate::prelude::*;

/// Query parameters for list_versions
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListVersionsQueryRequest {
    /// Page number (0-indexed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Number of versions per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}

impl ListVersionsQueryRequest {
    pub fn builder() -> ListVersionsQueryRequestBuilder {
        <ListVersionsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListVersionsQueryRequestBuilder {
    page: Option<i64>,
    page_size: Option<i64>,
}

impl ListVersionsQueryRequestBuilder {
    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListVersionsQueryRequest`].
    pub fn build(self) -> Result<ListVersionsQueryRequest, BuildError> {
        Ok(ListVersionsQueryRequest {
            page: self.page,
            page_size: self.page_size,
        })
    }
}
