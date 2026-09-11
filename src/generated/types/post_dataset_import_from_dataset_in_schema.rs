#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PostDatasetImportFromDatasetInSchema {
    ///Constraint: minItems=1, maxItems=10000
    pub dataset_record_ids: Vec<String>,
}
