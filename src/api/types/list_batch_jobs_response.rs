pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListBatchJobsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<BatchJob>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<ListBatchJobsResponseObject>,
    #[serde(default)]
    pub total: i64,
}

impl ListBatchJobsResponse {
    pub fn builder() -> ListBatchJobsResponseBuilder {
        <ListBatchJobsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListBatchJobsResponseBuilder {
    data: Option<Vec<BatchJob>>,
    object: Option<ListBatchJobsResponseObject>,
    total: Option<i64>,
}

impl ListBatchJobsResponseBuilder {
    pub fn data(mut self, value: Vec<BatchJob>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn object(mut self, value: ListBatchJobsResponseObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn total(mut self, value: i64) -> Self {
        self.total = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListBatchJobsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`total`](ListBatchJobsResponseBuilder::total)
    pub fn build(self) -> Result<ListBatchJobsResponse, BuildError> {
        Ok(ListBatchJobsResponse {
            data: self.data,
            object: self.object,
            total: self
                .total
                .ok_or_else(|| BuildError::missing_field("total"))?,
        })
    }
}
