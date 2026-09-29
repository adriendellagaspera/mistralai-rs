pub use crate::prelude::*;

/// Query parameters for libraries_list_v1
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LibrariesListV1QueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Continuation token from a previous response's next_page_token. Preferred over `page`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// Deprecated: use page_token. Offset paging re-scans earlier pages and is being phased out.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Case-insensitive search on the library name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    /// Deprecated: this parameter will be removed in a future version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_owned_by_me: Option<bool>,
}

impl LibrariesListV1QueryRequest {
    pub fn builder() -> LibrariesListV1QueryRequestBuilder {
        <LibrariesListV1QueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LibrariesListV1QueryRequestBuilder {
    page_size: Option<i64>,
    page_token: Option<String>,
    page: Option<i64>,
    search: Option<String>,
    filter_owned_by_me: Option<bool>,
}

impl LibrariesListV1QueryRequestBuilder {
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn page_token(mut self, value: impl Into<String>) -> Self {
        self.page_token = Some(value.into());
        self
    }

    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn search(mut self, value: impl Into<String>) -> Self {
        self.search = Some(value.into());
        self
    }

    pub fn filter_owned_by_me(mut self, value: bool) -> Self {
        self.filter_owned_by_me = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LibrariesListV1QueryRequest`].
    pub fn build(self) -> Result<LibrariesListV1QueryRequest, BuildError> {
        Ok(LibrariesListV1QueryRequest {
            page_size: self.page_size,
            page_token: self.page_token,
            page: self.page,
            search: self.search,
            filter_owned_by_me: self.filter_owned_by_me,
        })
    }
}
