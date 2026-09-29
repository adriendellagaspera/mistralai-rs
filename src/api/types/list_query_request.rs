pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_total: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_type: Option<Vec<SampleType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Vec<Source>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<FilePurpose>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mimetypes: Option<Vec<String>>,
}

impl ListQueryRequest {
    pub fn builder() -> ListQueryRequestBuilder {
        <ListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListQueryRequestBuilder {
    page: Option<i64>,
    page_size: Option<i64>,
    include_total: Option<bool>,
    sample_type: Option<Vec<SampleType>>,
    source: Option<Vec<Source>>,
    search: Option<String>,
    purpose: Option<FilePurpose>,
    mimetypes: Option<Vec<String>>,
}

impl ListQueryRequestBuilder {
    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn include_total(mut self, value: bool) -> Self {
        self.include_total = Some(value);
        self
    }

    pub fn sample_type(mut self, value: Vec<SampleType>) -> Self {
        self.sample_type = Some(value);
        self
    }

    pub fn source(mut self, value: Vec<Source>) -> Self {
        self.source = Some(value);
        self
    }

    pub fn search(mut self, value: impl Into<String>) -> Self {
        self.search = Some(value.into());
        self
    }

    pub fn purpose(mut self, value: FilePurpose) -> Self {
        self.purpose = Some(value);
        self
    }

    pub fn mimetypes(mut self, value: Vec<String>) -> Self {
        self.mimetypes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListQueryRequest`].
    pub fn build(self) -> Result<ListQueryRequest, BuildError> {
        Ok(ListQueryRequest {
            page: self.page,
            page_size: self.page_size,
            include_total: self.include_total,
            sample_type: self.sample_type,
            source: self.source,
            search: self.search,
            purpose: self.purpose,
            mimetypes: self.mimetypes,
        })
    }
}
