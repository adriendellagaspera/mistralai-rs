#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum JudgeClassificationOutputType {
    #[default]
    #[serde(rename = "CLASSIFICATION")]
    Classification,
}
