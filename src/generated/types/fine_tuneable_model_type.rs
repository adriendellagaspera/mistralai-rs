#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum FineTuneableModelType {
    #[default]
    #[serde(rename = "completion")]
    Completion,
    #[serde(rename = "classifier")]
    Classifier,
}
