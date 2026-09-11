#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ClassifierFTModelOutObject {
    #[default]
    #[serde(rename = "model")]
    Model,
}
