#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum FileVisibility {
    #[default]
    #[serde(rename = "workspace")]
    Workspace,
    #[serde(rename = "user")]
    User,
}
