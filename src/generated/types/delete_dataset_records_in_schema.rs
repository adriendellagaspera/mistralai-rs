#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeleteDatasetRecordsInSchema {
    ///Constraint: minItems=1, maxItems=500
    pub dataset_record_ids: Vec<String>,
}
