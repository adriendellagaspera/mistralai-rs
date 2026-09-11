#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum WorkflowType {
    #[default]
    #[serde(rename = "code")]
    Code,
}
