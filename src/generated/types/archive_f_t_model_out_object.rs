#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ArchiveFTModelOutObject {
    #[default]
    #[serde(rename = "model")]
    Model,
}
