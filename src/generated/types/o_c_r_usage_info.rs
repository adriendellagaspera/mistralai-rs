#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OCRUsageInfo {
    ///Document size in bytes
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub doc_size_bytes: Option<Option<i64>>,
    ///Number of pages processed
    ///Constraint: minimum=0
    pub pages_processed: i64,
}
