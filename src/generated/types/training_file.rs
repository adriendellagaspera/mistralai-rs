#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TrainingFile {
    pub file_id: uuid::Uuid,
    ///Constraint: exclusiveMinimum=0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
}
