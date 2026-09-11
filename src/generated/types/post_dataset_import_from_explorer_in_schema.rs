#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PostDatasetImportFromExplorerInSchema {
    ///Constraint: maxItems=500
    pub completion_event_ids: Vec<String>,
}
