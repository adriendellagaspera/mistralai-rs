#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum FTClassifierLossFunction {
    #[default]
    #[serde(rename = "single_class")]
    SingleClass,
    #[serde(rename = "multi_class")]
    MultiClass,
}
