#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum FilesApiRoutesUploadFileRequestVisibilityWorkspaceEnum {
    #[default]
    #[serde(rename = "workspace")]
    Workspace,
    #[serde(rename = "user")]
    User,
}
