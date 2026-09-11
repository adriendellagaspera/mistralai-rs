#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DatasetImportTasks {
    pub tasks: PaginatedResultDatasetImportTask,
}
