#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum BatchJobOutObject {
    #[default]
    #[serde(rename = "batch")]
    Batch,
}
