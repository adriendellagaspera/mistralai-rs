pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PaginatedResultDatasetRecord {
    #[serde(default)]
    pub count: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<DatasetRecord>>,
}

impl PaginatedResultDatasetRecord {
    pub fn builder() -> PaginatedResultDatasetRecordBuilder {
        <PaginatedResultDatasetRecordBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaginatedResultDatasetRecordBuilder {
    count: Option<i64>,
    next: Option<String>,
    previous: Option<String>,
    results: Option<Vec<DatasetRecord>>,
}

impl PaginatedResultDatasetRecordBuilder {
    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn next(mut self, value: impl Into<String>) -> Self {
        self.next = Some(value.into());
        self
    }

    pub fn previous(mut self, value: impl Into<String>) -> Self {
        self.previous = Some(value.into());
        self
    }

    pub fn results(mut self, value: Vec<DatasetRecord>) -> Self {
        self.results = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaginatedResultDatasetRecord`].
    /// This method will fail if any of the following fields are not set:
    /// - [`count`](PaginatedResultDatasetRecordBuilder::count)
    pub fn build(self) -> Result<PaginatedResultDatasetRecord, BuildError> {
        Ok(PaginatedResultDatasetRecord {
            count: self
                .count
                .ok_or_else(|| BuildError::missing_field("count"))?,
            next: self.next,
            previous: self.previous,
            results: self.results,
        })
    }
}
