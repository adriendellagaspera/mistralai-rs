pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListEvaluationRunRecordsResponse {
    #[serde(default)]
    pub records: PaginatedResultEvaluationRunRecordV2Response,
}

impl ListEvaluationRunRecordsResponse {
    pub fn builder() -> ListEvaluationRunRecordsResponseBuilder {
        <ListEvaluationRunRecordsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListEvaluationRunRecordsResponseBuilder {
    records: Option<PaginatedResultEvaluationRunRecordV2Response>,
}

impl ListEvaluationRunRecordsResponseBuilder {
    pub fn records(mut self, value: PaginatedResultEvaluationRunRecordV2Response) -> Self {
        self.records = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListEvaluationRunRecordsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`records`](ListEvaluationRunRecordsResponseBuilder::records)
    pub fn build(self) -> Result<ListEvaluationRunRecordsResponse, BuildError> {
        Ok(ListEvaluationRunRecordsResponse {
            records: self
                .records
                .ok_or_else(|| BuildError::missing_field("records"))?,
        })
    }
}
