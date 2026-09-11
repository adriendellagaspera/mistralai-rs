#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum UnarchiveFTModelOutObject {
    #[default]
    #[serde(rename = "model")]
    Model,
}
