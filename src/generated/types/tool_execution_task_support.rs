#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ToolExecutionTaskSupport {
    #[default]
    #[serde(rename = "forbidden")]
    Forbidden,
    #[serde(rename = "optional")]
    Optional,
    #[serde(rename = "required")]
    Required,
}
