pub use crate::prelude::*;

/// Source-agnostic counts for a completed dataset import.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DatasetImportResult {
    #[serde(default)]
    pub imported_record_count: i64,
    #[serde(default)]
    pub requested_record_count: i64,
    #[serde(default)]
    pub skipped_record_count: i64,
}

impl DatasetImportResult {
    pub fn builder() -> DatasetImportResultBuilder {
        <DatasetImportResultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DatasetImportResultBuilder {
    imported_record_count: Option<i64>,
    requested_record_count: Option<i64>,
    skipped_record_count: Option<i64>,
}

impl DatasetImportResultBuilder {
    pub fn imported_record_count(mut self, value: i64) -> Self {
        self.imported_record_count = Some(value);
        self
    }

    pub fn requested_record_count(mut self, value: i64) -> Self {
        self.requested_record_count = Some(value);
        self
    }

    pub fn skipped_record_count(mut self, value: i64) -> Self {
        self.skipped_record_count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DatasetImportResult`].
    /// This method will fail if any of the following fields are not set:
    /// - [`imported_record_count`](DatasetImportResultBuilder::imported_record_count)
    /// - [`requested_record_count`](DatasetImportResultBuilder::requested_record_count)
    /// - [`skipped_record_count`](DatasetImportResultBuilder::skipped_record_count)
    pub fn build(self) -> Result<DatasetImportResult, BuildError> {
        Ok(DatasetImportResult {
            imported_record_count: self
                .imported_record_count
                .ok_or_else(|| BuildError::missing_field("imported_record_count"))?,
            requested_record_count: self
                .requested_record_count
                .ok_or_else(|| BuildError::missing_field("requested_record_count"))?,
            skipped_record_count: self
                .skipped_record_count
                .ok_or_else(|| BuildError::missing_field("skipped_record_count"))?,
        })
    }
}
