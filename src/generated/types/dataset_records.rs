#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DatasetRecords {
    pub records: PaginatedResultDatasetRecord,
}
