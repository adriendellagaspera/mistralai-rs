#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum CompletionFTModelOutModelType {
    #[default]
    #[serde(rename = "completion")]
    Completion,
}
