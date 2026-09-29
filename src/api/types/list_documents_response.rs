pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListDocumentsResponse {
    #[serde(default)]
    pub pagination: PaginationInfo,
    #[serde(default)]
    pub data: Vec<Document>,
}

impl ListDocumentsResponse {
    pub fn builder() -> ListDocumentsResponseBuilder {
        <ListDocumentsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListDocumentsResponseBuilder {
    pagination: Option<PaginationInfo>,
    data: Option<Vec<Document>>,
}

impl ListDocumentsResponseBuilder {
    pub fn pagination(mut self, value: PaginationInfo) -> Self {
        self.pagination = Some(value);
        self
    }

    pub fn data(mut self, value: Vec<Document>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListDocumentsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`pagination`](ListDocumentsResponseBuilder::pagination)
    /// - [`data`](ListDocumentsResponseBuilder::data)
    pub fn build(self) -> Result<ListDocumentsResponse, BuildError> {
        Ok(ListDocumentsResponse {
            pagination: self
                .pagination
                .ok_or_else(|| BuildError::missing_field("pagination"))?,
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
        })
    }
}
