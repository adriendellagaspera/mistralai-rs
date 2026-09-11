#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PaginatedResultDatasetImportTask {
    pub count: i64,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub next: Option<Option<String>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub previous: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<DatasetImportTask>>,
}
