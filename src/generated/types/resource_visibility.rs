#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ResourceVisibility {
    #[default]
    #[serde(rename = "shared_global")]
    SharedGlobal,
    #[serde(rename = "shared_org")]
    SharedOrg,
    #[serde(rename = "shared_workspace")]
    SharedWorkspace,
    #[serde(rename = "private")]
    Private,
}
