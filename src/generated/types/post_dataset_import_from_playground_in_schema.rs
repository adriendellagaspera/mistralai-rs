#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PostDatasetImportFromPlaygroundInSchema {
    pub conversation_ids: Vec<String>,
}
