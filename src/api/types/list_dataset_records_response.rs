pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListDatasetRecordsResponse {
    #[serde(default)]
    pub records: PaginatedResultDatasetRecord,
}

impl ListDatasetRecordsResponse {
    pub fn builder() -> ListDatasetRecordsResponseBuilder {
        <ListDatasetRecordsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListDatasetRecordsResponseBuilder {
    records: Option<PaginatedResultDatasetRecord>,
}

impl ListDatasetRecordsResponseBuilder {
    pub fn records(mut self, value: PaginatedResultDatasetRecord) -> Self {
        self.records = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListDatasetRecordsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`records`](ListDatasetRecordsResponseBuilder::records)
    pub fn build(self) -> Result<ListDatasetRecordsResponse, BuildError> {
        Ok(ListDatasetRecordsResponse {
            records: self
                .records
                .ok_or_else(|| BuildError::missing_field("records"))?,
        })
    }
}
