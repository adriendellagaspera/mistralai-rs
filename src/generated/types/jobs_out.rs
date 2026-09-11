#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JobsOut {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<JobsOutDataItemUnion>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<JobsOutObject>,
    pub total: i64,
}
