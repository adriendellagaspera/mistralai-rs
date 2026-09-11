#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum JudgeRegressionOutputType {
    #[default]
    #[serde(rename = "REGRESSION")]
    Regression,
}
