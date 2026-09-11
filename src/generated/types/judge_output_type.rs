#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum JudgeOutputType {
    #[default]
    #[serde(rename = "REGRESSION")]
    Regression,
    #[serde(rename = "CLASSIFICATION")]
    Classification,
}
