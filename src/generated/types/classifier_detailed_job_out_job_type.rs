#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ClassifierDetailedJobOutJobType {
    #[default]
    #[serde(rename = "classifier")]
    Classifier,
}
