pub use crate::prelude::*;

/// Query parameters for libraries_documents_list_v1
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LibrariesDocumentsListV1QueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Deprecated: this parameter will be removed in a future version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters_attributes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

impl LibrariesDocumentsListV1QueryRequest {
    pub fn builder() -> LibrariesDocumentsListV1QueryRequestBuilder {
        <LibrariesDocumentsListV1QueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LibrariesDocumentsListV1QueryRequestBuilder {
    search: Option<String>,
    page_size: Option<i64>,
    page: Option<i64>,
    filters_attributes: Option<String>,
    sort_by: Option<String>,
    sort_order: Option<String>,
}

impl LibrariesDocumentsListV1QueryRequestBuilder {
    pub fn search(mut self, value: impl Into<String>) -> Self {
        self.search = Some(value.into());
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn filters_attributes(mut self, value: impl Into<String>) -> Self {
        self.filters_attributes = Some(value.into());
        self
    }

    pub fn sort_by(mut self, value: impl Into<String>) -> Self {
        self.sort_by = Some(value.into());
        self
    }

    pub fn sort_order(mut self, value: impl Into<String>) -> Self {
        self.sort_order = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LibrariesDocumentsListV1QueryRequest`].
    pub fn build(self) -> Result<LibrariesDocumentsListV1QueryRequest, BuildError> {
        Ok(LibrariesDocumentsListV1QueryRequest {
            search: self.search,
            page_size: self.page_size,
            page: self.page,
            filters_attributes: self.filters_attributes,
            sort_by: self.sort_by,
            sort_order: self.sort_order,
        })
    }
}
