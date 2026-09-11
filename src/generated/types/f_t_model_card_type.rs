#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum FTModelCardType {
    #[default]
    #[serde(rename = "fine-tuned")]
    FineTuned,
}
