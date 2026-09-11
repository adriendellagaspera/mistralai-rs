#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum EmbeddedResourceType {
    #[default]
    #[serde(rename = "resource")]
    Resource,
}
