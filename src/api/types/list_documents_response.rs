pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListDocumentsResponse {
    #[serde(default)]
    pub data: Vec<Document>,
    #[serde(default)]
    pub pagination: PaginationInfo,
}

impl ListDocumentsResponse {
    pub fn builder() -> ListDocumentsResponseBuilder {
        <ListDocumentsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListDocumentsResponseBuilder {
    data: Option<Vec<Document>>,
    pagination: Option<PaginationInfo>,
}

impl ListDocumentsResponseBuilder {
    pub fn data(mut self, value: Vec<Document>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn pagination(mut self, value: PaginationInfo) -> Self {
        self.pagination = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListDocumentsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListDocumentsResponseBuilder::data)
    /// - [`pagination`](ListDocumentsResponseBuilder::pagination)
    pub fn build(self) -> Result<ListDocumentsResponse, BuildError> {
        Ok(ListDocumentsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            pagination: self
                .pagination
                .ok_or_else(|| BuildError::missing_field("pagination"))?,
        })
    }
}
