#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ClassifierFTModelOutModelType {
    #[default]
    #[serde(rename = "classifier")]
    Classifier,
}
