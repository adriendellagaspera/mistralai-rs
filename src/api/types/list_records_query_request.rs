pub use crate::prelude::*;

/// Query parameters for list_records
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListRecordsQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
}

impl ListRecordsQueryRequest {
    pub fn builder() -> ListRecordsQueryRequestBuilder {
        <ListRecordsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListRecordsQueryRequestBuilder {
    page_size: Option<i64>,
    page: Option<i64>,
}

impl ListRecordsQueryRequestBuilder {
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListRecordsQueryRequest`].
    pub fn build(self) -> Result<ListRecordsQueryRequest, BuildError> {
        Ok(ListRecordsQueryRequest {
            page_size: self.page_size,
            page: self.page,
        })
    }
}
