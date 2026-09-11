#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ShareEnum {
    #[default]
    #[serde(rename = "Viewer")]
    Viewer,
    #[serde(rename = "Editor")]
    Editor,
}
