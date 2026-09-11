#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum CompletionFTModelOutObject {
    #[default]
    #[serde(rename = "model")]
    Model,
}
