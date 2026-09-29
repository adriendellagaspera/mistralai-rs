pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteDatasetRecordsRequest {
    #[serde(default)]
    pub dataset_record_ids: Vec<String>,
}

impl DeleteDatasetRecordsRequest {
    pub fn builder() -> DeleteDatasetRecordsRequestBuilder {
        <DeleteDatasetRecordsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteDatasetRecordsRequestBuilder {
    dataset_record_ids: Option<Vec<String>>,
}

impl DeleteDatasetRecordsRequestBuilder {
    pub fn dataset_record_ids(mut self, value: Vec<String>) -> Self {
        self.dataset_record_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeleteDatasetRecordsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dataset_record_ids`](DeleteDatasetRecordsRequestBuilder::dataset_record_ids)
    pub fn build(self) -> Result<DeleteDatasetRecordsRequest, BuildError> {
        Ok(DeleteDatasetRecordsRequest {
            dataset_record_ids: self
                .dataset_record_ids
                .ok_or_else(|| BuildError::missing_field("dataset_record_ids"))?,
        })
    }
}
