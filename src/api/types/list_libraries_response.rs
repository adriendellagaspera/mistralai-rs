pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListLibrariesResponse {
    /// Deprecated: offset pagination metadata. Only populated for callers using the deprecated `page` parameter; omitted when `page_token` is used. While RBAC filtering is being rolled out `total_items` is a rough estimate (candidate count before per-library checks). Use `next_page_token` instead — this field will be removed once offset paging is retired.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<PaginationInfo>,
    #[serde(default)]
    pub data: Vec<Library>,
    /// Opaque continuation token for the next page. Pass it back as `page_token` to fetch the next page. Null when there are no more results. Prefer this over the deprecated offset `page` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl ListLibrariesResponse {
    pub fn builder() -> ListLibrariesResponseBuilder {
        <ListLibrariesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListLibrariesResponseBuilder {
    pagination: Option<PaginationInfo>,
    data: Option<Vec<Library>>,
    next_page_token: Option<String>,
}

impl ListLibrariesResponseBuilder {
    pub fn pagination(mut self, value: PaginationInfo) -> Self {
        self.pagination = Some(value);
        self
    }

    pub fn data(mut self, value: Vec<Library>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn next_page_token(mut self, value: impl Into<String>) -> Self {
        self.next_page_token = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListLibrariesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListLibrariesResponseBuilder::data)
    pub fn build(self) -> Result<ListLibrariesResponse, BuildError> {
        Ok(ListLibrariesResponse {
            pagination: self.pagination,
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            next_page_token: self.next_page_token,
        })
    }
}
