#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ResourceLinkType {
    #[default]
    #[serde(rename = "resource_link")]
    ResourceLink,
}
