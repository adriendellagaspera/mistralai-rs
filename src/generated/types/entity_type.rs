///The type of entity, used to share a library.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum EntityType {
    #[default]
    #[serde(rename = "User")]
    User,
    #[serde(rename = "Workspace")]
    Workspace,
    #[serde(rename = "Org")]
    Org,
}
