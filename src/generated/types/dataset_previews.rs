#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DatasetPreviews {
    pub datasets: PaginatedResultDatasetPreview,
}
