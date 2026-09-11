#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BatchJobsOut {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<BatchJobOut>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<BatchJobsOutObject>,
    pub total: i64,
}
